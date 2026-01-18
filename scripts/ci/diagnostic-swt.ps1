$ErrorActionPreference = "Stop"

$SwtAppJar = if ($env:SWT_APP_JAR) { $env:SWT_APP_JAR } else { "tests/apps/swt/target/swt-test-app-1.0.0-all.jar" }
$AgentJar = if ($env:SWT_AGENT_JAR) { $env:SWT_AGENT_JAR } else { "agent/target/robotframework-swing-agent-1.0.0-all.jar" }
$Port = if ($env:SWT_PORT) { [int]$env:SWT_PORT } else { 5679 }
$LogDir = if ($env:SWT_LOG_DIR) { $env:SWT_LOG_DIR } else { $env:TEMP }
$StdoutLog = Join-Path $LogDir "swt-app-stdout.log"
$StderrLog = Join-Path $LogDir "swt-app-stderr.log"

Write-Host "Checking SWT app jar: $SwtAppJar"
if (-not (Test-Path $SwtAppJar)) {
  throw "Missing SWT app jar: $SwtAppJar"
}

Write-Host "Checking agent jar: $AgentJar"
if (-not (Test-Path $AgentJar)) {
  throw "Missing agent jar: $AgentJar"
}

Write-Host "Starting SWT app (logs: $StdoutLog, $StderrLog)"
$javaArgs = @(
  "-javaagent:$AgentJar=port=$Port",
  "-jar",
  $SwtAppJar
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
    Write-Host "---- SWT stdout ----"
    if (Test-Path $StdoutLog) { Get-Content -Tail 200 $StdoutLog }
    Write-Host "---- SWT stderr ----"
    if (Test-Path $StderrLog) { Get-Content -Tail 200 $StderrLog }
    throw "SWT app failed to open port $Port"
  }

  Write-Host "SWT app started successfully."
}
finally {
  if ($proc -and -not $proc.HasExited) {
    Write-Host "Stopping SWT app (pid=$($proc.Id))"
    Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
  }
}
