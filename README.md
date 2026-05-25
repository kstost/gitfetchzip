# gitfetchzip

Download a GitHub repository snapshot at a specific commit and extract it into a target directory without running `git clone`.

## Install

### macOS / Linux

Paste this into your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/kstost/gitfetchzip/main/install.sh | bash
```

- The installer automatically downloads the binary that matches your OS and CPU architecture.
- It installs `gitfetchzip` into `/usr/local/bin` when possible, otherwise into `~/.local/bin`.
- After installation, you can run `gitfetchzip` from anywhere.

### Windows

Open PowerShell and paste this command. Administrator permission is not required.

```powershell
irm https://raw.githubusercontent.com/kstost/gitfetchzip/main/install.ps1 | iex
```

- The installer downloads the matching Windows binary.
- It installs into `%LOCALAPPDATA%\gitfetchzip\` and adds that directory to your user PATH.
- After installation, close and reopen PowerShell if the `gitfetchzip` command is not found immediately.

### Verify installation

```bash
gitfetchzip --help
```

## Usage

```bash
gitfetchzip <repo-url> <commit|index> <target-dir>
```

Examples:

```bash
gitfetchzip https://github.com/kstost/cokacdir 0 ~/Downloads/cokacdir
gitfetchzip https://github.com/kstost/cokacdir 3 ~/Downloads/cokacdir-old
gitfetchzip https://github.com/kstost/cokacdir 4b592f250f784e259a9a41dc18bb4fcbc2074dbc ~/Downloads/cokacdir
```

`index` is zero-based from the latest commit. `0` means the latest commit, and `3` means the fourth commit from the latest.

The target directory must either not exist or be empty.

## Build

```bash
cargo build --release
```

The project also includes the cross-platform build system adopted from
`kstost/cokacdircode`:

```bash
python3 build.py --status
python3 build.py --native
python3 build.py --all
python3 build.py --windows
```

Build outputs are written to `dist_beta/` with names such as
`gitfetchzip-linux-x86_64` and `gitfetchzip-windows-x86_64.exe`.

## MVP Scope

- Public GitHub repositories
- Commit SHA or latest-relative commit index
- ZIP download and extraction
- Top-level archive directory removal
- Linux, macOS, and Windows compatible Rust binary

---

## Disclaimer

This software is provided **as is**, without warranty of any kind, express or implied. This includes, but is not limited to, warranties of merchantability, fitness for a particular purpose, and non-infringement.

In no event shall the authors, copyright holders, or contributors be liable for any claim, damages, or other liability arising from the use of or inability to use this software. This includes, but is not limited to, data loss or corruption, system malfunction, security issues, financial loss, and direct, indirect, incidental, special, punitive, or consequential damages.

Use of this software is entirely at your own risk and responsibility.
