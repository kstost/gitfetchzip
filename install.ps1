#
# GITFETCHZIP Installer for Windows
# Usage: irm https://raw.githubusercontent.com/kstost/gitfetchzip/refs/heads/main/install.ps1 | iex
#

$ErrorActionPreference = "Stop"

$BINARY_NAME = "gitfetchzip"
$BASE_URL = "https://raw.githubusercontent.com/kstost/gitfetchzip/refs/heads/main/dist_beta"

function Info($msg) { Write-Host "→ $msg" -ForegroundColor Blue }
function Success($msg) { Write-Host "✓ $msg" -ForegroundColor Green }
function Warn($msg) { Write-Host "! $msg" -ForegroundColor Yellow }
function Error($msg) { Write-Host "✗ $msg" -ForegroundColor Red; throw $msg }

# Detect architecture
function Detect-Arch {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        default { Error "Unsupported architecture: $arch" }
    }
}

# Get install directory
function Get-InstallDir {
    $dir = Join-Path $env:LOCALAPPDATA "gitfetchzip"
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
    return $dir
}

# Add directory to user PATH
function Add-ToPath($dir) {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$dir*") {
        [Environment]::SetEnvironmentVariable("Path", "$dir;$currentPath", "User")
        $env:Path = "$dir;$env:Path"
        Success "Added $dir to PATH"
    }
}

function Main {
    $arch = Detect-Arch

    Info "Downloading gitfetchzip (windows-$arch)..."

    $filename = "${BINARY_NAME}-windows-${arch}.exe"
    $url = "${BASE_URL}/${filename}"

    $installDir = Get-InstallDir
    $installPath = Join-Path $installDir "${BINARY_NAME}.exe"

    # Stop running instances to release file lock
    if (Test-Path $installPath) {
        Stop-Process -Name $BINARY_NAME -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 1
    }

    # Download
    try {
        Invoke-WebRequest -Uri $url -OutFile $installPath -UseBasicParsing
    } catch {
        Error "Download failed: $_`n  If gitfetchzip is running, close it first and try again."
    }

    # Verify
    if (Test-Path $installPath) {
        Add-ToPath $installDir
        Success "Installed!"

        Success "Run 'gitfetchzip' to start."
    } else {
        Error "Installation failed"
    }
}

try { Main } catch { }
