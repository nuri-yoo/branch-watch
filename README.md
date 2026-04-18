# branch-watch — GitHub Branch & Fork Sync Status CLI

[![GitHub Marketplace](https://img.shields.io/badge/Marketplace-branch--watch-blue?logo=github)](https://github.com/marketplace/actions/branch-watch)
[![Release](https://img.shields.io/github/v/release/nuri-yoo/branch-watch)](https://github.com/nuri-yoo/branch-watch/releases)
[![PyPI](https://img.shields.io/pypi/v/branch-watch)](https://pypi.org/project/branch-watch/)
[![npm](https://img.shields.io/npm/v/branch-watch)](https://www.npmjs.com/package/branch-watch)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**branch-watch** (`bw`) is a fast, single-binary CLI tool that tells you — at a glance — whether your GitHub branches are behind `main`, how far your forks have drifted from upstream, and what pull requests are open. No browser required. Powered by the GitHub REST API.

> **TL;DR** — Run `bw forks` to see all your forked repos vs. upstream. Run `bw branches owner/repo` to see every branch vs. the default branch. Run `bw prs owner/repo` for open PRs.

---

## Why branch-watch?

Keeping track of GitHub branch and fork sync status manually is tedious:

- You maintain **multiple forks** of open-source projects and need to know which ones are stale
- Your team has **dozens of feature branches** and you want to catch the ones that have drifted before they become impossible to merge
- You want to check open PR status **without leaving the terminal**

branch-watch solves all three with a single command.

---

## Demo

### Check if all your forks are behind upstream

```
$ bw forks

Forked repositories

  alice/rust          rust-lang/rust        ↓ 42 behind
  alice/tokio         tokio-rs/tokio        ↓ 7 behind   ↑ 2 ahead
  alice/serde         serde-rs/serde        ✓ in sync
  acme/axum           tokio-rs/axum         ↓ 15 behind  ↑ 8 ahead
  acme/reqwest        seanmonstar/reqwest   ↑ 3 ahead
```

### Check branch sync status vs. main

```
$ bw branches owner/repo

→ owner/repo (base: main)

  feature/auth        ↓ 3 behind  ↑ 2 ahead
  feat/dashboard      ↓ 14 behind
  fix/login           ✓ up to date
```

### List open pull requests

```
$ bw prs owner/repo

→ owner/repo — 2 open PRs

  #101  Add dark mode support
         feat/dark-mode → main  by @alice · 2 reviewers

  #98   Fix login redirect [draft]
         fix/login → main  by @bob
```

---

## Features

| Feature | Description |
|---------|-------------|
| Fork sync status | Shows behind/ahead commit count for all your forked repos vs. upstream |
| Branch status | Shows every branch vs. the repo's default branch |
| PR overview | Lists open PRs with author, branches, draft status, and reviewer count |
| GitHub Actions support | Use as a CI step to fail builds on stale branches |
| Multi-platform | macOS (Intel + Apple Silicon), Linux (x86_64 + ARM64) |
| Single binary | No runtime, no dependencies — written in Rust |
| Token-based auth | Works with GitHub PAT via env var or config file |

---

## Installation

### Homebrew — recommended for macOS and Linux

```sh
brew install nuri-yoo/tap/branch-watch
```

### pip — for Python users

```sh
pip install branch-watch
```

### npm — for Node.js users

```sh
npm install -g branch-watch
```

### Pre-built binaries — direct download

Download from the [releases page](https://github.com/nuri-yoo/branch-watch/releases):

| Platform | Binary |
|----------|--------|
| macOS Apple Silicon (M1/M2/M3/M4/M5) | `branch-watch-*-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `branch-watch-*-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 | `branch-watch-*-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `branch-watch-*-aarch64-unknown-linux-gnu.tar.gz` |

```sh
tar xzf branch-watch-*.tar.gz
sudo mv bw /usr/local/bin/
```

### Build from source — for Rust developers

Requires [Rust](https://rustup.rs) 1.80+.

```sh
git clone https://github.com/nuri-yoo/branch-watch
cd branch-watch
cargo install --path .
```

---

## Authentication

branch-watch uses the GitHub REST API and requires a Personal Access Token (PAT) with `repo` scope.

**Generate a token**: [github.com/settings/tokens](https://github.com/settings/tokens) → Generate new token → select `repo`

**Option 1 — Save to config file (persistent):**

```sh
bw auth ghp_xxxxxxxxxxxxxxxxxxxx
# Saved to ~/.branch-watch.toml
```

**Option 2 — Environment variable:**

```sh
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxx
bw forks
```

---

## Usage

### `bw forks` — check if your GitHub forks are behind upstream

Lists all repositories you have forked and compares each one to its upstream default branch. Shows how many commits behind or ahead each fork is.

```sh
bw forks
```

**When to use**: After returning from vacation, before syncing forks, or when maintaining many open-source forks simultaneously.

### `bw branches` — check if branches are behind main

Shows the sync status of every branch in a repository relative to the default branch (usually `main` or `master`).

```sh
bw branches owner/repo
```

**When to use**: Before a sprint planning session, during code review, or when cleaning up stale branches.

### `bw prs` — list open pull requests

Lists all open pull requests with author, source → target branch, draft indicator, and number of requested reviewers.

```sh
bw prs owner/repo
```

**When to use**: Daily standup, PR review sessions, or release preparation.

### `bw auth` — save GitHub token

```sh
bw auth <token>
```

---

## Use in GitHub Actions

branch-watch is available on the [GitHub Marketplace](https://github.com/marketplace/actions/branch-watch). Add it to any workflow to automatically check branch sync status in CI:

```yaml
steps:
  - uses: nuri-yoo/branch-watch@v1
    with:
      command: branches          # branches | forks | prs
      repo: ${{ github.repository }}
    env:
      GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Use cases in CI:**
- Fail a build if feature branches are more than N commits behind `main`
- Post a daily Slack summary of fork sync status
- Block merges when a branch is significantly out of date

---

## FAQ

**How do I check if my GitHub fork is behind upstream?**
Run `bw forks`. It lists every fork in your GitHub account and shows how many commits it is behind or ahead of the upstream repository.

**How do I check if a branch is behind main on GitHub?**
Run `bw branches owner/repo`. It compares every branch in the repository against the default branch and shows behind/ahead commit counts.

**Does branch-watch work with private repositories?**
Yes. Any private repository your GitHub token has `repo` scope access to will work.

**Does branch-watch support GitHub Enterprise Server?**
Not yet. Currently supports github.com only.

**What is the difference between branch-watch and `git fetch --prune`?**
`git fetch` requires a local clone of the repository. branch-watch queries the GitHub API remotely — no local clone needed. It also works across all your repositories at once.

**Does branch-watch support GitLab or Bitbucket?**
No. branch-watch is built specifically for the GitHub REST API.

**Is branch-watch free?**
Yes. branch-watch is open-source under the MIT license.

---

## GitHub API Usage

branch-watch integrates with the following GitHub REST API endpoints:

| Method | Endpoint | Purpose |
|--------|----------|---------|
| `GET` | `/user/repos?type=fork` | List authenticated user's forked repositories |
| `GET` | `/repos/{owner}/{repo}` | Fetch repository metadata and upstream parent |
| `GET` | `/repos/{owner}/{repo}/branches` | List all branches |
| `GET` | `/repos/{owner}/{repo}/compare/{base}...{head}` | Get ahead/behind commit counts |
| `GET` | `/repos/{owner}/{repo}/pulls?state=open` | List open pull requests |

---

## License

MIT © [nuri-yoo](https://github.com/nuri-yoo)
