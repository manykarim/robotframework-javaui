$ErrorActionPreference = "Stop"

$SwingAppJar = if ($env:SWING_APP_JAR) { $env:SWING_APP_JAR } else { "tests/apps/swing/target/swing-test-app-1.0.0.jar" }
$AgentJar = if ($env:SWING_AGENT_JAR) { $env:SWING_AGENT_JAR } else { "agent/target/robotframework-swing-agent-1.0.0-all.jar" }
$Port = if ($env:SWING_PORT) { [int]$env:SWING_PORT } else { 5678 }
$LogDir = if ($env:SWING_LOG_DIR) { $env:SWING_LOG_DIR } else { $env:TEMP }
$StdoutLog = Join-Path $LogDir "swing-app-stdout.log"
$StderrLog = Join-Path $LogDir "swing-app-stderr.log"

Write-Host "Checking Swing app jar: $SwingAppJar"
if (-not (Test-Path $SwingAppJar)) {
  throw "Missing Swing app jar: $SwingAppJar"
}

Write-Host "Checking agent jar: $AgentJar"
if (-not (Test-Path $AgentJar)) {
  throw "Missing agent jar: $AgentJar"
}

Write-Host "Starting Swing app (logs: $StdoutLog, $StderrLog)"
$javaArgs = @(
  "-javaagent:$AgentJar=port=$Port",
  "-jar",
  $SwingAppJar
)
$proc = Start-Process -FilePath "java" -ArgumentList $javaArgs `
  -RedirectStandardOutput $StdoutLog -RedirectStandardError $StderrLog -PassThru

try {
  $ready = $false
  for ($i = 0; $i -lt 20; $i++) {
    $tnc = Test-NetConnection -ComputerName "localhost" -Port $Port -WarningAction SilentlyContinue
    if ($tnc.TcpTestSucceeded) {
      $ready = $true
      break
    }
    Start-Sleep -Seconds 1
  }

  if (-not $ready) {
    Write-Host "Port $Port never opened."
    Write-Host "---- Swing stdout ----"
    if (Test-Path $StdoutLog) { Get-Content -Tail 200 $StdoutLog }
    Write-Host "---- Swing stderr ----"
    if (Test-Path $StderrLog) { Get-Content -Tail 200 $StderrLog }
    throw "Swing app failed to open port $Port"
  }

  Write-Host "Swing app started successfully."
}
finally {
  if ($proc -and -not $proc.HasExited) {
    Write-Host "Stopping Swing app (pid=$($proc.Id))"
    Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
  }
}
