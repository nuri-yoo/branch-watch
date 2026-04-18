#!/usr/bin/env node
const { spawnSync } = require("child_process");
const os = require("os");
const path = require("path");

const BIN_PATH = path.join(os.homedir(), ".branch-watch", "bin", "bw");

const result = spawnSync(BIN_PATH, process.argv.slice(2), { stdio: "inherit" });
process.exit(result.status ?? 1);
