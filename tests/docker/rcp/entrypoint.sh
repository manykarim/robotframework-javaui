#!/usr/bin/env bash
#
# Harness entrypoint: boot Xvfb, attach the JavaGui agent to DBeaver, launch DBeaver
# headless, wait for the agent RPC port, then run the Robot experiment suite (which
# drives DBeaver and captures framebuffer screenshots as visual confirmation).
#
# Everything that can fail is logged rather than aborting: the point of this harness is
# to COLLECT EVIDENCE about what works and what breaks, not to go green.
#
set -uo pipefail

DISPLAY_NUM=":99"
AGENT_PORT="${AGENT_PORT:-5682}"
AGENT_JAR="/work/agent/target/javagui-agent.jar"
DBEAVER="/opt/dbeaver/dbeaver"
INI="/opt/dbeaver/dbeaver.ini"
WS="/tmp/dbeaver-ws"
OUT="/work/results/dbeaver"
SHOTS="$OUT/shots"
LOGS="$OUT/logs"

mkdir -p "$WS" "$OUT" "$SHOTS" "$LOGS"

log() { echo "[harness] $*"; }

# ---------------------------------------------------------------------------
# 1. Headless X server
# ---------------------------------------------------------------------------
log "starting Xvfb on $DISPLAY_NUM"
Xvfb "$DISPLAY_NUM" -screen 0 1600x1000x24 -nolisten tcp >"$LOGS/xvfb.log" 2>&1 &
XVFB_PID=$!
export DISPLAY="$DISPLAY_NUM"
sleep 2

grab() { # grab NAME  -> whole-framebuffer PNG (visual confirmation)
  local name="$1"
  import -window root "$SHOTS/$name.png" 2>>"$LOGS/screenshot.log" \
    || { xwd -root -silent 2>>"$LOGS/screenshot.log" | convert xwd:- "$SHOTS/$name.png" 2>>"$LOGS/screenshot.log"; }
}

# ---------------------------------------------------------------------------
# 2. Attach the agent to DBeaver via dbeaver.ini
#    The -javaagent line MUST go inside the existing -vmargs section (Eclipse
#    products allow exactly one -vmargs; a second one breaks the launcher).
# ---------------------------------------------------------------------------
if [ ! -f "$AGENT_JAR" ]; then
  log "FATAL: agent jar not found at $AGENT_JAR (build it: mvn -f agent/pom.xml package)"
  exit 2
fi
# 2a. Repoint DBeaver at a FULL JDK 21 (its bundled jlink JRE has no java.instrument
#     module, so a -javaagent cannot load). -vm must appear before -vmargs.
FULL_JAVA="${FULL_JAVA:-/usr/lib/jvm/java-21-openjdk-amd64/bin/java}"
if ! grep -q '^-vm$' "$INI"; then
  sed -i "/^-vmargs/i -vm" "$INI"
  sed -i "\|^-vm\$|a ${FULL_JAVA}" "$INI"
  log "repointed dbeaver.ini -vm -> ${FULL_JAVA}"
fi
# 2b. Attach the agent as a JVM arg (inside the single -vmargs section).
if ! grep -q "javagui-agent" "$INI"; then
  if grep -q '^-vmargs' "$INI"; then
    sed -i "/^-vmargs/a -javaagent:${AGENT_JAR}=port=${AGENT_PORT},toolkit=swt,host=127.0.0.1" "$INI"
  else
    printf '\n-vmargs\n-javaagent:%s=port=%s,toolkit=swt,host=127.0.0.1\n' "$AGENT_JAR" "$AGENT_PORT" >> "$INI"
  fi
  log "patched dbeaver.ini with agent (port $AGENT_PORT, toolkit=swt)"
fi
cp "$INI" "$LOGS/dbeaver.ini.used"

# ---------------------------------------------------------------------------
# 3. Launch DBeaver headless
# ---------------------------------------------------------------------------
log "launching DBeaver ..."
"$DBEAVER" -nosplash -data "$WS" -consoleLog >"$LOGS/dbeaver.log" 2>&1 &
DBEAVER_PID=$!

# ---------------------------------------------------------------------------
# 4. Wait for the agent RPC port to accept connections
# ---------------------------------------------------------------------------
log "waiting for agent on 127.0.0.1:$AGENT_PORT ..."
UP=0
for i in $(seq 1 90); do
  if (exec 3<>"/dev/tcp/127.0.0.1/$AGENT_PORT") 2>/dev/null; then exec 3>&- 3<&-; UP=1; break; fi
  if ! kill -0 "$DBEAVER_PID" 2>/dev/null; then log "DBeaver process exited early (see dbeaver.log)"; break; fi
  sleep 2
done
grab "00_after_launch"
if [ "$UP" = 1 ]; then
  log "agent port is UP after ~$((i*2))s"
else
  log "agent port NEVER came up; running suite anyway to record the failure"
fi

# 4a. The agent RPC port opens at premain, LONG before DBeaver creates its SWT Display
#     and opens the workbench window. Wait for the framebuffer to actually render
#     content (mean pixel > threshold) so we drive a live UI, not a black screen.
log "waiting for DBeaver workbench window to render ..."
RENDERED=0
for i in $(seq 1 60); do
  import -window root /tmp/probe.png 2>/dev/null || true
  MEAN=$(convert /tmp/probe.png -format "%[mean]" info: 2>/dev/null || echo 0)
  # ImageMagick mean is 0..65535; a black screen is 0, a rendered workbench ~30000+
  if awk "BEGIN{exit !(${MEAN:-0} > 1500)}" 2>/dev/null; then RENDERED=1; break; fi
  sleep 2
done
if [ "$RENDERED" = 1 ]; then
  log "workbench rendered after ~$((i*2))s (framebuffer mean=$MEAN)"
else
  log "workbench never rendered (framebuffer stayed near-black); recording anyway"
fi
grab "05_workbench_rendered"

# ---------------------------------------------------------------------------
# 5. Run the Robot experiment suite (driver co-located, so screenshots are local)
# ---------------------------------------------------------------------------
export PYTHONPATH=/work/python
export SHOTS_DIR="$SHOTS"
export AGENT_PORT
SUITE_DIR="${SUITE_DIR:-/work/tests/robot/rcp/real_dbeaver}"
log "running Robot suite: $SUITE_DIR"
robot \
  --outputdir "$OUT" \
  --variable SHOTS:"$SHOTS" \
  --variable PORT:"$AGENT_PORT" \
  --loglevel TRACE \
  "$SUITE_DIR" >"$LOGS/robot.stdout.log" 2>&1
RC=$?
log "robot finished rc=$RC"

grab "99_final"

# ---------------------------------------------------------------------------
# 6. Teardown
# ---------------------------------------------------------------------------
kill "$DBEAVER_PID" 2>/dev/null || true
kill "$XVFB_PID" 2>/dev/null || true
# Hand results back to the host user (container runs as root over a bind mount).
chown -R "$(stat -c %u:%g /work)" "$OUT" 2>/dev/null || true
log "done. results in results/dbeaver (logs + shots/)"
# Always exit 0: harness success == 'we collected evidence', not 'app went green'.
exit 0
