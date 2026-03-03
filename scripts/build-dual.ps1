param(
    [string]$WindowsTarget = "x86_64-pc-windows-gnu",
    [string]$LinuxTarget = "x86_64-unknown-linux-gnu",
    [ValidateSet("cargo", "cross")]
    [string]$LinuxBuilder = "cross",
    [switch]$Release,
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
if (-not (Test-Path (Join-Path $projectRoot "Cargo.toml"))) {
    throw "Cargo.toml not found from script location. Expected project root at: $projectRoot"
}

function Test-Tool([string]$Name) {
    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Ensure-RustTarget([string]$Target) {
    $installed = rustup target list --installed
    if ($installed -notcontains $Target) {
        Write-Host "[setup] Installing missing target: $Target"
        rustup target add $Target | Out-Host
    }
}

$profileArgs = @()
if ($Release) {
    $profileArgs += "--release"
}

if (-not (Test-Tool "cargo")) {
    throw "cargo not found in PATH."
}

Ensure-RustTarget -Target $WindowsTarget
Ensure-RustTarget -Target $LinuxTarget

if ($LinuxBuilder -eq "cross" -and -not (Test-Tool "cross")) {
    Write-Warning "cross is not installed. Falling back to cargo for Linux build."
    Write-Warning "Install with: cargo install cross"
    $LinuxBuilder = "cargo"
}

$logsDir = Join-Path $projectRoot "target\build-logs"
New-Item -ItemType Directory -Path $logsDir -Force | Out-Null

$windowsArgs = @("build", "--target", $WindowsTarget) + $profileArgs
$linuxArgs = @("build", "--target", $LinuxTarget) + $profileArgs

$windowsCmd = "cargo " + ($windowsArgs -join " ")
$linuxCmd = "$LinuxBuilder " + ($linuxArgs -join " ")

Write-Host "[windows] $windowsCmd"
Write-Host "[linux]   $linuxCmd"
Write-Host "[mode]    parallel"

if ($DryRun) {
    Write-Host "Dry run enabled: no build started."
    exit 0
}

$timeTag = Get-Date -Format "yyyyMMdd-HHmmss"
$winLog = Join-Path $logsDir "windows-$timeTag.log"
$linLog = Join-Path $logsDir "linux-$timeTag.log"

$runner = {
    param([string]$Root, [string]$Exe, [string[]]$Args, [string]$LogPath, [string]$Label)

    Set-Location $Root

    & $Exe @Args *>&1 | Tee-Object -FilePath $LogPath | Out-Host
    $exitCode = $LASTEXITCODE

    [pscustomobject]@{
        Label = $Label
        ExitCode = $exitCode
        Log = $LogPath
    }
}

$winJob = Start-Job -Name "build-windows" -ScriptBlock $runner -ArgumentList @(
    $projectRoot,
    "cargo",
    $windowsArgs,
    $winLog,
    "windows"
)

$linJob = Start-Job -Name "build-linux" -ScriptBlock $runner -ArgumentList @(
    $projectRoot,
    $LinuxBuilder,
    $linuxArgs,
    $linLog,
    "linux"
)

Wait-Job -Job @($winJob, $linJob) | Out-Null

$results = @()
$results += Receive-Job -Job $winJob
$results += Receive-Job -Job $linJob

Remove-Job -Job @($winJob, $linJob) -Force

$failed = $results | Where-Object { $_.ExitCode -ne 0 }

Write-Host ""
Write-Host "=== Build Summary ==="
foreach ($r in $results) {
    if ($r.ExitCode -eq 0) {
        Write-Host ("[{0}] OK (log: {1})" -f $r.Label, $r.Log)
    } else {
        Write-Host ("[{0}] FAILED (exit {1}, log: {2})" -f $r.Label, $r.ExitCode, $r.Log)
    }
}

if ($failed.Count -gt 0) {
    exit 1
}

exit 0
