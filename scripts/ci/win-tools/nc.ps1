$ErrorActionPreference = "Stop"

if ($args.Count -lt 2) {
  Write-Error "Usage: nc -z -w1 <host> <port>"
  exit 2
}

$port = [int]$args[-1]
$host = $args[-2]

$tnc = Test-NetConnection -ComputerName $host -Port $port -WarningAction SilentlyContinue
if ($tnc.TcpTestSucceeded) {
  exit 0
}

exit 1
