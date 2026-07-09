#!/usr/bin/env bash
#
# Build the real Eclipse RCP test application (tests/apps/rcp/plugins/com.testapp.rcp)
# against a downloaded Eclipse platform, install it, and launch it headless with the
# JavaGui agent attached — so the RCP keywords can be validated against a REAL Eclipse
# workbench (not the MockRcpApplication simulation).
#
# This proves the rcp-real-eclipse-validation capability. It does NOT use Tycho: it
# compiles the bundle directly against the Eclipse platform's plugin jars and installs
# it via the dropins directory, which is simpler and fully reproducible in CI.
#
# Requirements: JDK 17+, curl, tar, xvfb (for headless), network access to
# download.eclipse.org on first run.
#
# Env vars:
#   ECLIPSE_VERSION   Eclipse platform version (default 4.30)
#   ECLIPSE_DROP      drops4 build id (default R-4.30-202312010110)
#   WORKDIR           where to place the eclipse runtime + build (default: $PWD/.rcp-real)
#   AGENT_PORT        agent RPC port (default 5682)
#
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PLUGIN="$ROOT/tests/apps/rcp/plugins/com.testapp.rcp"
AGENT_JAR="$ROOT/agent/target/javagui-agent.jar"

ECLIPSE_VERSION="${ECLIPSE_VERSION:-4.30}"
ECLIPSE_DROP="${ECLIPSE_DROP:-R-4.30-202312010110}"
WORKDIR="${WORKDIR:-$ROOT/tests/apps/rcp/.rcp-real}"
AGENT_PORT="${AGENT_PORT:-5682}"
ECL="$WORKDIR/eclipse"

mkdir -p "$WORKDIR"

# 1. Download + extract the Eclipse platform runtime (cached).
if [ ! -x "$ECL/eclipse" ]; then
  echo "[real-rcp] downloading Eclipse platform $ECLIPSE_VERSION ..."
  curl -fsSL -o "$WORKDIR/eclipse-platform.tar.gz" \
    "https://download.eclipse.org/eclipse/downloads/drops4/${ECLIPSE_DROP}/eclipse-platform-${ECLIPSE_VERSION}-linux-gtk-x86_64.tar.gz"
  tar xzf "$WORKDIR/eclipse-platform.tar.gz" -C "$WORKDIR"
fi

# 2. Compile the RCP plugin against the Eclipse platform bundles.
echo "[real-rcp] compiling com.testapp.rcp ..."
BIN="$WORKDIR/bin"; rm -rf "$BIN"; mkdir -p "$BIN"
CP="$(ls "$ECL"/plugins/*.jar | tr '\n' ':')"
find "$PLUGIN/src" -name '*.java' > "$WORKDIR/sources.txt"
javac -nowarn -d "$BIN" -cp "$CP" @"$WORKDIR/sources.txt"

# 3. Assemble the OSGi bundle jar and install it into dropins.
cp -r "$PLUGIN/META-INF" "$BIN/"
cp "$PLUGIN/plugin.xml" "$BIN/"
[ -d "$PLUGIN/icons" ] && cp -r "$PLUGIN/icons" "$BIN/" || true
sed -i 's/1.0.0.qualifier/1.0.0/' "$BIN/META-INF/MANIFEST.MF"
mkdir -p "$ECL/dropins"
( cd "$BIN" && jar cfm "$ECL/dropins/com.testapp.rcp_1.0.0.jar" META-INF/MANIFEST.MF com plugin.xml $( [ -d icons ] && echo icons ) )

# 4. Attach the JavaGui agent (SWT/RCP mode) via eclipse.ini.
if ! grep -q 'javagui-agent' "$ECL/eclipse.ini"; then
  echo "-javaagent:${AGENT_JAR}=port=${AGENT_PORT},toolkit=swt" >> "$ECL/eclipse.ini"
fi

# 5. Launch headless. The workbench comes up; connect on $AGENT_PORT and drive it
#    with the RCP library. Caller is responsible for connecting/asserting.
echo "[real-rcp] launching Eclipse RCP app headless on agent port ${AGENT_PORT} ..."
rm -rf "$WORKDIR/ws"; mkdir -p "$WORKDIR/ws"
exec xvfb-run -a "$ECL/eclipse" \
  -application com.testapp.rcp.application \
  -data "$WORKDIR/ws" -consoleLog
