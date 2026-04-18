# branch-watch

[![GitHub Marketplace](https://img.shields.io/badge/Marketplace-branch--watch-blue?logo=github)](https://github.com/marketplace/actions/branch-watch)
[![Release](https://img.shields.io/github/v/release/nuri-yoo/branch-watch)](https://github.com/nuri-yoo/branch-watch/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Track branch and fork sync status across your GitHub repositories from the command line.

## Overview

`branch-watch` (`bw`) is a CLI tool that uses the [GitHub REST API](https://docs.github.com/en/rest) to give you a real-time view of how your branches and forks compare to their base or upstream. Whether you manage a monorepo with many feature branches, maintain forks of multiple open-source projects, or want to keep tabs on open pull requests — `bw` surfaces the information in one place.

```
$ bw forks

Forked repositories

  alice/rust          rust-lang/rust        ↓ 42 behind
  alice/tokio         tokio-rs/tokio        ↓ 7 behind   ↑ 2 ahead
  alice/serde         serde-rs/serde        ✓ in sync
  acme/axum           tokio-rs/axum         ↓ 15 behind  ↑ 8 ahead
  acme/reqwest        seanmonstar/reqwest   ↑ 3 ahead
```

```
$ bw branches owner/repo

→ owner/repo (base: main)

  feature/auth        ↓ 3 behind  ↑ 2 ahead
  feat/dashboard      ↓ 14 behind
  fix/login           ✓ up to date
```

## Features

- **Fork sync status** — lists all your forked repositories and shows how many commits each fork is behind or ahead of its upstream default branch
- **Branch status** — for any repository, shows how every branch compares to the default branch (main/master/etc.)
- **Open PR list** — shows open pull requests with author, head → base branch, draft status, and pending reviewers
- **Works with any GitHub account** — personal accounts, organization forks, company repositories
- **Single binary** — no runtime dependencies

## GitHub API Usage

`branch-watch` integrates with the following GitHub REST API endpoints:

| Endpoint | Purpose |
|----------|---------|
| `GET /user/repos?type=fork` | List authenticated user's forked repositories |
| `GET /repos/{owner}/{repo}` | Fetch repository metadata including upstream parent info |
| `GET /repos/{owner}/{repo}/branches` | List all branches in a repository |
| `GET /repos/{owner}/{repo}/compare/{base}...{head}` | Compare two refs to get ahead/behind commit counts |
| `GET /repos/{owner}/{repo}/pulls?state=open` | List open pull requests |

Authentication is handled via a [GitHub Personal Access Token (PAT)](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens) with `repo` scope.

## Installation

### Homebrew (macOS / Linux)

```sh
brew tap nuri-yoo/tap
brew install branch-watch
```

### Pre-built binaries

Download the latest binary for your platform from the [releases page](https://github.com/nuri-yoo/branch-watch/releases):

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `branch-watch-*-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `branch-watch-*-x86_64-apple-darwin.tar.gz` |
| Linux (x86_64) | `branch-watch-*-x86_64-unknown-linux-gnu.tar.gz` |
| Linux (ARM64) | `branch-watch-*-aarch64-unknown-linux-gnu.tar.gz` |

```sh
tar xzf branch-watch-*.tar.gz
sudo mv bw /usr/local/bin/
```

### From source

Requires [Rust](https://rustup.rs) 1.80+.

```sh
git clone https://github.com/nuri-yoo/branch-watch
cd branch-watch
cargo install --path .
```

The binary is installed as `bw`.

## Use in GitHub Actions

Add branch-watch to your CI workflow to surface stale branches automatically:

```yaml
- uses: nuri-yoo/branch-watch@v1
  with:
    command: branches
    repo: ${{ github.repository }}
```

## Authentication

Generate a GitHub Personal Access Token with `repo` scope at [github.com/settings/tokens](https://github.com/settings/tokens), then:

```sh
bw auth <your-token>
```

The token is saved to `~/.branch-watch.toml`. Alternatively, set the `GITHUB_TOKEN` environment variable:

```sh
export GITHUB_TOKEN=<your-token>
bw forks
```

## Usage

### Fork sync status

Show all your forked repositories and their sync status against upstream:

```sh
bw forks
```

Output columns: `your-fork` · `upstream` · sync status (↓ behind / ↑ ahead / ✓ in sync)

### Branch status

Show how every branch in a repository compares to its default branch:

```sh
bw branches owner/repo
```

Useful for reviewing stale branches, checking which feature branches need rebasing, or auditing a team repository.

### Open pull requests

List open pull requests with branch and reviewer info:

```sh
bw prs owner/repo
```

### Save authentication token

```sh
bw auth ghp_xxxxxxxxxxxxxxxxxxxx
```

## Use cases

**Personal open-source contributor** — maintain forks of multiple upstream projects and quickly see which ones have drifted without opening GitHub in a browser.

**Team lead** — run `bw branches owner/company-repo` to spot feature branches that have fallen behind `main` before they become hard to merge.

**Release manager** — use `bw prs owner/repo` to get a clean list of open PRs without GitHub's UI noise.

## License

MIT
