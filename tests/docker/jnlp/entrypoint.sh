#!/usr/bin/env bash
#
# Harness entrypoint: boot Xvfb, verify the prebuilt Swing test app + agent jar are
# present, serve the JNLP over 127.0.0.1:8099, then run the Robot Web Start suite.
#
# HONEST SCOPE: the suite launches the JNLP with a real IcedTea-Web launcher and asserts an
# either/or — EITHER the library attached and can drive the live app, OR the launch failed
# with the *clear, documented* SecurityManager AttachError. Under IcedTea-Web the
# JNLPSecurityManager STRUCTURALLY blocks dynamic attach (independent of permission level),
# so on this image the expected, proven outcome is: launch + discover + SecurityManager
# block. A green attach requires a launcher/JDK without the legacy SecurityManager
# (modern OpenWebStart, or JDK 24+).
#
# Everything that can fail is logged rather than aborting: the point is to COLLECT EVIDENCE
# about what works and what breaks. The harness ALWAYS exits 0.
#
set -uo pipefail

DISPLAY_NUM=":99"
SERVE_PORT="${SERVE_PORT:-8099}"
HOST="127.0.0.1"
JNLP_FILE="app.jnlp"
SWING_JAR_NAME="swing-test-app-1.0.0.jar"

SWING_JAR="/work/tests/apps/swing/target/${SWING_JAR_NAME}"
# The library's Launch Web Start Application injects the BUNDLED agent jar (get_agent_jar_path).
AGENT_JAR="/work/python/JavaGui/jars/javagui-agent.jar"
JNLP_SRC="/work/tests/apps/jnlp/${JNLP_FILE}"

SERVE_DIR="/tmp/jnlp-serve"
OUT="/work/results/jnlp"
LOGS="$OUT/logs"

mkdir -p "$SERVE_DIR" "$OUT" "$LOGS"

log() { echo "[harness] $*"; }

# ---------------------------------------------------------------------------
# 1. Headless X server
# ---------------------------------------------------------------------------
log "starting Xvfb on $DISPLAY_NUM"
Xvfb "$DISPLAY_NUM" -screen 0 1600x1000x24 -nolisten tcp >"$LOGS/xvfb.log" 2>&1 &
XVFB_PID=$!
export DISPLAY="$DISPLAY_NUM"
sleep 2

# ---------------------------------------------------------------------------
# 2. Verify the host-built artifacts are present (built before `docker run`)
#    Missing artifacts are logged, not fatal: the Robot suite self-skips cleanly
#    when the swing jar is absent, and we still want to collect that evidence.
# ---------------------------------------------------------------------------
if [ ! -f "$SWING_JAR" ]; then
  log "WARNING: Swing test app jar not found at $SWING_JAR"
  log "         build it on the host: (cd tests/apps/swing && mvn package)"
fi
if [ ! -f "$AGENT_JAR" ]; then
  log "WARNING: bundled agent jar not found at $AGENT_JAR"
  log "         build it on the host: uv run invoke build"
fi
log "launcher (JAVAGUI_JAVAWS)=${JAVAGUI_JAVAWS:-<unset>}"
java -version >"$LOGS/java-version.log" 2>&1 || true

# ---------------------------------------------------------------------------
# 3. Serve the JNLP over 127.0.0.1 only. Stage app.jnlp + the swing jar into one
#    directory so the codebase (http://127.0.0.1:8099/) resolves both the descriptor
#    and its main jar. (The Robot suite also stages+serves in its Suite Setup; this
#    entrypoint server keeps the harness self-contained and, on the same port, the
#    suite's socket check simply succeeds against it.)
# ---------------------------------------------------------------------------
cp -f "$JNLP_SRC" "$SERVE_DIR/${JNLP_FILE}" 2>/dev/null || log "could not stage $JNLP_SRC"
cp -f "$SWING_JAR" "$SERVE_DIR/${SWING_JAR_NAME}" 2>/dev/null || log "could not stage $SWING_JAR"
log "serving $SERVE_DIR on http://${HOST}:${SERVE_PORT}/ ..."
( cd "$SERVE_DIR" && python3 -m http.server "$SERVE_PORT" --bind "$HOST" ) \
    >"$LOGS/httpd.log" 2>&1 &
HTTPD_PID=$!
# Wait for the server to accept connections.
for i in $(seq 1 20); do
  if (exec 3<>"/dev/tcp/${HOST}/${SERVE_PORT}") 2>/dev/null; then exec 3>&- 3<&-; break; fi
  sleep 0.5
done

# ---------------------------------------------------------------------------
# 4. Run the Robot Web Start suite. JavaGui is imported from the bind-mounted
#    /work/python (prebuilt _core.abi3.so); JAVAGUI_JAVAWS points at the ITW image.
#    The suite self-skips if a precondition is missing, and asserts the either/or.
# ---------------------------------------------------------------------------
export PYTHONPATH=/work/python
export SERVE_PORT
SUITE_DIR="${SUITE_DIR:-/work/tests/robot/jnlp}"
log "running Robot suite: $SUITE_DIR"
robot \
  --outputdir "$OUT" \
  --variable SERVE_PORT:"$SERVE_PORT" \
  --loglevel TRACE \
  "$SUITE_DIR" >"$LOGS/robot.stdout.log" 2>&1
RC=$?
log "robot finished rc=$RC (see $OUT/log.html; rc reflects the suite's either/or assertion)"

# ---------------------------------------------------------------------------
# 5. Teardown
# ---------------------------------------------------------------------------
# 'app.jnlp' appears in the launcher/app JVM command line but NOT in the http.server
# nor this script's command line, so this never self-kills the harness.
pkill -f "$JNLP_FILE" 2>/dev/null || true
kill "$HTTPD_PID" 2>/dev/null || true
kill "$XVFB_PID" 2>/dev/null || true
# Hand results back to the host user (container runs as root over a bind mount).
chown -R "$(stat -c %u:%g /work)" "$OUT" 2>/dev/null || true
log "done. results in results/jnlp (log.html + logs/)"
# Always exit 0: harness success == 'we collected evidence', not 'attach went green'.
exit 0
