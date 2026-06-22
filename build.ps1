param(
    [ValidateSet("debug", "release")]
    [string]$Mode = "release",
    [switch]$Clean
)

$ErrorActionPreference = "Stop"

Write-Host "=== key-monitor-rust build script ===" -ForegroundColor Cyan
Write-Host "Mode: $Mode" -ForegroundColor Yellow

if ($Clean) {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
    if ($LASTEXITCODE -ne 0) { exit 1 }
    Write-Host "Clean done." -ForegroundColor Green
    return
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: cargo not found. Please install Rust: https://rustup.rs" -ForegroundColor Red
    exit 1
}

$cargoArgs = @("build")
if ($Mode -eq "release") {
    $cargoArgs += "--release"
}

Write-Host "Running: cargo $($cargoArgs -join ' ')" -ForegroundColor Gray
cargo @cargoArgs

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

$profile = if ($Mode -eq "release") { "release" } else { "debug" }
$exePath = "target\$profile\key-monitor-rust.exe"

if (-not (Test-Path $exePath)) {
    Write-Host "Build succeeded but exe not found at expected path." -ForegroundColor Yellow
    exit 1
}

$distDir = "dist"
if (-not (Test-Path $distDir)) {
    New-Item -ItemType Directory -Path $distDir | Out-Null
}

Copy-Item -Path $exePath -Destination $distDir -Force

Write-Host "Build successful!" -ForegroundColor Green
Write-Host "Output: $distDir\key-monitor-rust.exe" -ForegroundColor Green
