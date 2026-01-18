#!/usr/bin/env bash
set -euo pipefail

SWING_APP_JAR="${SWING_APP_JAR:-tests/apps/swing/target/swing-test-app-1.0.0.jar}"
AGENT_JAR="${SWING_AGENT_JAR:-agent/target/robotframework-swing-agent-1.0.0-all.jar}"
PORT="${SWING_PORT:-5678}"
LOG_DIR="${SWING_LOG_DIR:-/tmp}"
STDOUT_LOG="${LOG_DIR}/swing-app-stdout.log"
STDERR_LOG="${LOG_DIR}/swing-app-stderr.log"

echo "Checking Swing app jar: ${SWING_APP_JAR}"
if [[ ! -f "${SWING_APP_JAR}" ]]; then
  echo "Missing Swing app jar: ${SWING_APP_JAR}"
  exit 1
fi

echo "Checking agent jar: ${AGENT_JAR}"
if [[ ! -f "${AGENT_JAR}" ]]; then
  echo "Missing agent jar: ${AGENT_JAR}"
  exit 1
fi

if ! command -v nc >/dev/null 2>&1; then
  echo "Missing 'nc' (netcat) for port checks."
  exit 1
fi

SWING_PID=""
cleanup() {
  if [[ -n "${SWING_PID}" ]] && kill -0 "${SWING_PID}" 2>/dev/null; then
    echo "Stopping Swing app (pid=${SWING_PID})"
    kill "${SWING_PID}" || true
    sleep 1
  fi
}
trap cleanup EXIT

echo "Starting Swing app (logs: ${STDOUT_LOG}, ${STDERR_LOG})"
nohup java -javaagent:"${AGENT_JAR}"=port="${PORT}" -jar "${SWING_APP_JAR}" \
  > "${STDOUT_LOG}" 2> "${STDERR_LOG}" &
SWING_PID=$!

echo "Waiting for port ${PORT}..."
for _ in $(seq 1 20); do
  if nc -z localhost "${PORT}"; then
    echo "Port ${PORT} is open."
    break
  fi
  sleep 1
done

if ! nc -z localhost "${PORT}"; then
  echo "Port ${PORT} never opened."
  echo "---- Swing stdout ----"
  tail -n 200 "${STDOUT_LOG}" || true
  echo "---- Swing stderr ----"
  tail -n 200 "${STDERR_LOG}" || true
  exit 1
fi

echo "Swing app started successfully."
