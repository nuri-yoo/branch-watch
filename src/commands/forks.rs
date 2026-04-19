use anyhow::Result;
use colored::Colorize;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use octocrab::Octocrab;
use serde_json::{json, Value};

use crate::config;
use crate::github::{compare_branches, upstream_info};

pub async fn run(
    client: &Octocrab,
    behind_only: bool,
    output_json: bool,
    org: Option<&str>,
) -> Result<()> {
    let cfg = config::load()?;
    let fork_list = fetch_all_forks(client, org).await?;

    let fork_list: Vec<(String, String)> = fork_list
        .into_iter()
        .filter(|(owner, name)| {
            let full = format!("{owner}/{name}");
            !cfg.ignore.iter().any(|ig| ig == &full)
        })
        .collect();

    if fork_list.is_empty() {
        println!("No forked repositories found.");
        return Ok(());
    }

    let pb = ProgressBar::new(fork_list.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim} [{pos}/{len}] {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );

    let futures: Vec<_> = fork_list
        .iter()
        .map(|(owner, name)| {
            let pb = pb.clone();
            async move {
                pb.set_message(format!("{owner}/{name}"));
                let result = async {
                    if let Some((up_owner, up_repo, up_branch)) =
                        upstream_info(client, owner, name).await?
                    {
                        let cmp = compare_branches(
                            client,
                            owner,
                            name,
                            &format!("{up_owner}:{up_branch}"),
                            "HEAD",
                        )
                        .await
                        .unwrap_or(crate::github::CompareResult {
                            behind: 0,
                            ahead: 0,
                        });
                        Ok::<_, anyhow::Error>(Some((
                            format!("{owner}/{name}"),
                            format!("{up_owner}/{up_repo}"),
                            cmp.behind,
                            cmp.ahead,
                        )))
                    } else {
                        Ok(None)
                    }
                }
                .await;
                pb.inc(1);
                result
            }
        })
        .collect();

    let results = join_all(futures).await;
    pb.finish_and_clear();

    let mut rows: Vec<(String, String, u64, u64)> = vec![];
    for res in results {
        if let Some(row) = res? {
            rows.push(row);
        }
    }

    // sort by behind descending, then ahead descending
    rows.sort_by(|a, b| b.2.cmp(&a.2).then(b.3.cmp(&a.3)));

    if behind_only {
        rows.retain(|(_, _, behind, _)| *behind > 0);
    }

    if rows.is_empty() {
        println!("All forks are in sync with upstream.");
        return Ok(());
    }

    if output_json {
        let out: Vec<Value> = rows
            .iter()
            .map(|(repo, upstream, behind, ahead)| {
                json!({ "repo": repo, "upstream": upstream, "behind": behind, "ahead": ahead })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("{}\n", "Forked repositories".bold());

    let name_width = rows.iter().map(|(n, _, _, _)| n.len()).max().unwrap_or(30);
    let up_width = rows.iter().map(|(_, u, _, _)| u.len()).max().unwrap_or(30);

    for (repo, upstream, behind, ahead) in &rows {
        let status = format_status(*behind, *ahead);
        println!(
            "  {:<nw$}  {:<uw$}  {status}",
            repo.bold(),
            upstream.dimmed(),
            nw = name_width,
            uw = up_width,
        );
    }
    println!();

    Ok(())
}

async fn fetch_all_forks(
    client: &Octocrab,
    org: Option<&str>,
) -> Result<Vec<(String, String)>> {
    let mut all: Vec<(String, String)> = vec![];
    let mut page: u32 = 1;

    loop {
        let url = match org {
            Some(o) => format!(
                "/orgs/{o}/repos?type=fork&per_page=100&page={page}"
            ),
            None => format!("/user/repos?type=fork&per_page=100&page={page}"),
        };

        let repos: Value = client.get(&url, None::<&()>).await?;

        let batch: Vec<(String, String)> = repos
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|r| {
                let owner = r["owner"]["login"].as_str()?.to_string();
                let name = r["name"].as_str()?.to_string();
                Some((owner, name))
            })
            .collect();

        if batch.is_empty() {
            break;
        }

        all.extend(batch);
        page += 1;
    }

    Ok(all)
}

fn format_status(behind: u64, ahead: u64) -> String {
    match (behind, ahead) {
        (0, 0) => "✓ in sync".green().to_string(),
        (b, 0) => format!("↓ {b} behind").yellow().to_string(),
        (0, a) => format!("↑ {a} ahead").cyan().to_string(),
        (b, a) => format!("↓ {b} behind  ↑ {a} ahead").yellow().to_string(),
    }
}
