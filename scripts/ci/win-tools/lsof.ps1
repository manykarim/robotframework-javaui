$ErrorActionPreference = "Stop"

$portToken = $args | Where-Object { $_ -match "^:(\d+)$" } | Select-Object -First 1
if (-not $portToken) {
  Write-Error "Usage: lsof -i :<port>"
  exit 2
}

$port = [int]($portToken -replace "^:", "")

$conn = Get-NetTCPConnection -LocalPort $port -State Listen -ErrorAction SilentlyContinue
if ($null -ne $conn) {
  exit 0
}

exit 1
