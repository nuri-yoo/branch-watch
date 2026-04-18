const { execSync } = require("child_process");
const fs = require("fs");
const https = require("https");
const os = require("os");
const path = require("path");
const zlib = require("zlib");

const VERSION = "0.1.0";
const BIN_DIR = path.join(os.homedir(), ".branch-watch", "bin");
const BIN_PATH = path.join(BIN_DIR, "bw");

function target() {
  const arch = os.arch();
  const platform = os.platform();
  if (platform === "darwin") {
    return arch === "arm64"
      ? "aarch64-apple-darwin"
      : "x86_64-apple-darwin";
  }
  if (platform === "linux") {
    return arch === "arm64"
      ? "aarch64-unknown-linux-gnu"
      : "x86_64-unknown-linux-gnu";
  }
  throw new Error(`Unsupported platform: ${platform}/${arch}`);
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https
      .get(url, (res) => {
        if (res.statusCode === 301 || res.statusCode === 302) {
          file.close();
          return download(res.headers.location, dest).then(resolve).catch(reject);
        }
        res.pipe(file);
        file.on("finish", () => file.close(resolve));
      })
      .on("error", (err) => {
        fs.unlink(dest, () => {});
        reject(err);
      });
  });
}

async function main() {
  if (fs.existsSync(BIN_PATH)) return;

  const t = target();
  const archive = `branch-watch-v${VERSION}-${t}.tar.gz`;
  const url = `https://github.com/nuri-yoo/branch-watch/releases/download/v${VERSION}/${archive}`;
  const tmp = path.join(os.tmpdir(), archive);

  console.log(`Downloading branch-watch v${VERSION} for ${t}...`);
  fs.mkdirSync(BIN_DIR, { recursive: true });
  await download(url, tmp);

  execSync(`tar xzf ${tmp} -C ${BIN_DIR}`);
  fs.chmodSync(BIN_PATH, 0o755);
  fs.unlinkSync(tmp);
  console.log(`Installed bw to ${BIN_PATH}`);
}

main().catch((err) => {
  console.error("Failed to install branch-watch:", err.message);
  process.exit(1);
});
