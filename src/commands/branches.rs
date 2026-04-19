use anyhow::Result;
use colored::Colorize;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use octocrab::Octocrab;
use serde_json::{json, Value};

use crate::config;
use crate::github::{compare_branches, default_branch};

pub async fn run(
    client: &Octocrab,
    repo: &str,
    behind_only: bool,
    output_json: bool,
    base_override: Option<&str>,
) -> Result<()> {
    let cfg = config::load()?;
    let (owner, name) = parse_repo(repo)?;

    // Check if this repo is in the ignore list
    let full_repo = format!("{owner}/{name}");
    if cfg.ignore.iter().any(|ig| ig == &full_repo) {
        println!("Repository '{full_repo}' is in the ignore list.");
        return Ok(());
    }

    let base = match base_override {
        Some(b) => b.to_string(),
        None => default_branch(client, owner, name).await?,
    };

    let mut all_branches: Vec<String> = vec![];
    let mut page: u32 = 1;
    loop {
        let branches: Value = client
            .get(
                format!("/repos/{owner}/{name}/branches?per_page=100&page={page}"),
                None::<&()>,
            )
            .await?;

        let batch: Vec<String> = branches
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|b| b["name"].as_str().map(str::to_string))
            .collect();

        if batch.is_empty() {
            break;
        }
        all_branches.extend(batch);
        page += 1;
    }

    let branch_names: Vec<String> = all_branches
        .into_iter()
        .filter(|b| b != &base)
        .collect();

    if branch_names.is_empty() {
        println!("No branches other than '{base}' found in {owner}/{name}.");
        return Ok(());
    }

    let pb = ProgressBar::new(branch_names.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim} [{pos}/{len}] {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );

    let futures: Vec<_> = branch_names
        .iter()
        .map(|branch| {
            let pb = pb.clone();
            let base = base.clone();
            async move {
                pb.set_message(branch.to_string());
                let cmp = compare_branches(client, owner, name, &base, branch).await;
                pb.inc(1);
                cmp.map(|c| (branch.clone(), c.behind, c.ahead))
            }
        })
        .collect();

    let results = join_all(futures).await;
    pb.finish_and_clear();

    let mut rows: Vec<(String, u64, u64)> = vec![];
    for res in results {
        rows.push(res?);
    }

    // sort by behind descending, then ahead descending
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)));

    if behind_only {
        rows.retain(|(_, behind, _)| *behind > 0);
    }

    if rows.is_empty() {
        println!("All branches are up to date with '{base}'.");
        return Ok(());
    }

    if output_json {
        let out: Vec<Value> = rows
            .iter()
            .map(|(branch, behind, ahead)| {
                json!({ "branch": branch, "base": base, "behind": behind, "ahead": ahead })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!(
        "{} {} (base: {})\n",
        "→".dimmed(),
        format!("{owner}/{name}").bold(),
        base.cyan()
    );

    let name_width = rows.iter().map(|(n, _, _)| n.len()).max().unwrap_or(20);

    for (branch, behind, ahead) in &rows {
        let status = format_status(*behind, *ahead);
        println!("  {:<width$}  {status}", branch.bold(), width = name_width);
    }
    println!();

    Ok(())
}

fn parse_repo(repo: &str) -> Result<(&str, &str)> {
    let mut parts = repo.splitn(2, '/');
    let owner = parts.next().unwrap_or("");
    let name = parts.next().unwrap_or("");
    if owner.is_empty() || name.is_empty() {
        anyhow::bail!("Repo must be in 'owner/name' format");
    }
    Ok((owner, name))
}

fn format_status(behind: u64, ahead: u64) -> String {
    match (behind, ahead) {
        (0, 0) => "✓ up to date".green().to_string(),
        (b, 0) => format!("↓ {b} behind").yellow().to_string(),
        (0, a) => format!("↑ {a} ahead").cyan().to_string(),
        (b, a) => format!("↓ {b} behind  ↑ {a} ahead").yellow().to_string(),
    }
}
