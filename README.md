# gitfetchzip

Download a GitHub repository snapshot at a specific commit and extract it into a target directory without running `git clone`.

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
