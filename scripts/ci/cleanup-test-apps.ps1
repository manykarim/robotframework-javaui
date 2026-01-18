$ErrorActionPreference = "Stop"

$patterns = @(
  "swing-test-app-1.0.0.jar",
  "swt-test-app-1.0.0-all.jar",
  "rcp-mock-test-app-1.0.0-all.jar"
)

$procs = Get-CimInstance Win32_Process -Filter "Name='java.exe'" -ErrorAction SilentlyContinue
foreach ($proc in $procs) {
  foreach ($pattern in $patterns) {
    if ($proc.CommandLine -and $proc.CommandLine -like "*$pattern*") {
      try {
        Stop-Process -Id $proc.ProcessId -Force -ErrorAction SilentlyContinue
      } catch {
        # ignore cleanup failures
      }
      break
    }
  }
}
