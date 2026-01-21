#!/usr/bin/env bash
set -euo pipefail

SWT_APP_JAR="${SWT_APP_JAR:-tests/apps/swt/target/swt-test-app-1.0.0-all.jar}"
AGENT_JAR="${AGENT_JAR:-agent/target/javagui-agent.jar}"
PORT="${SWT_PORT:-5679}"
LOG_DIR="${SWT_LOG_DIR:-/tmp}"
STDOUT_LOG="${LOG_DIR}/swt-app-stdout.log"
STDERR_LOG="${LOG_DIR}/swt-app-stderr.log"

echo "Checking SWT app jar: ${SWT_APP_JAR}"
if [[ ! -f "${SWT_APP_JAR}" ]]; then
  echo "Missing SWT app jar: ${SWT_APP_JAR}"
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

SWT_PID=""
cleanup() {
  if [[ -n "${SWT_PID}" ]] && kill -0 "${SWT_PID}" 2>/dev/null; then
    echo "Stopping SWT app (pid=${SWT_PID})"
    kill "${SWT_PID}" || true
    sleep 1
  fi
}
trap cleanup EXIT

echo "Starting SWT app (logs: ${STDOUT_LOG}, ${STDERR_LOG})"
VMARG=""
if [[ "$(uname -s)" == "Darwin" ]]; then
  VMARG="-XstartOnFirstThread"
fi
nohup java ${VMARG} -javaagent:"${AGENT_JAR}"=port="${PORT}" -jar "${SWT_APP_JAR}" \
  > "${STDOUT_LOG}" 2> "${STDERR_LOG}" &
SWT_PID=$!

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
  echo "---- SWT stdout ----"
  tail -n 200 "${STDOUT_LOG}" || true
  echo "---- SWT stderr ----"
  tail -n 200 "${STDERR_LOG}" || true
  exit 1
fi

echo "SWT app started successfully."
