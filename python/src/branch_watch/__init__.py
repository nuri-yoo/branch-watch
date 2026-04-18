import os
import platform
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
from pathlib import Path

VERSION = "0.1.0"
BIN_DIR = Path.home() / ".branch-watch" / "bin"
BIN_PATH = BIN_DIR / "bw"


def _target() -> str:
    machine = platform.machine().lower()
    system = sys.platform
    if system == "darwin":
        return "aarch64-apple-darwin" if machine == "arm64" else "x86_64-apple-darwin"
    elif system.startswith("linux"):
        return (
            "aarch64-unknown-linux-gnu"
            if machine == "aarch64"
            else "x86_64-unknown-linux-gnu"
        )
    raise RuntimeError(f"Unsupported platform: {system}/{machine}")


def _ensure_binary() -> Path:
    if BIN_PATH.exists():
        return BIN_PATH
    target = _target()
    url = (
        f"https://github.com/nuri-yoo/branch-watch/releases/download/"
        f"v{VERSION}/branch-watch-v{VERSION}-{target}.tar.gz"
    )
    BIN_DIR.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(suffix=".tar.gz", delete=False) as tmp:
        print(f"Downloading branch-watch {VERSION} for {target}...", file=sys.stderr)
        urllib.request.urlretrieve(url, tmp.name)
        with tarfile.open(tmp.name) as tar:
            tar.extract("bw", BIN_DIR)
    os.unlink(tmp.name)
    BIN_PATH.chmod(0o755)
    return BIN_PATH


def main() -> None:
    binary = _ensure_binary()
    result = subprocess.run([str(binary)] + sys.argv[1:])
    sys.exit(result.returncode)
