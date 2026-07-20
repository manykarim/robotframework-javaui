#!/usr/bin/env bash
#
# make_signed_jnlp.sh — build a signed, all-permissions Java Web Start harness that
# reuses the repo's Swing test app.
#
# It:
#   1. copies tests/apps/swing/target/swing-test-app-1.0.0.jar into a target dir,
#   2. stamps JNLP security attributes into its manifest (Permissions/Codebase),
#   3. self-signs it with a throwaway keystore (keytool + jarsigner, storepass changeit),
#   4. writes an all-permissions app.jnlp pointing at a local codebase URL.
#
# The result is meant to be served over 127.0.0.1 with `python3 -m http.server` and
# launched with `javaws` / a portable IcedTea-Web image (see README.md).
#
# NOTE (honest): signing to all-permissions clears IcedTea-Web's *trust* prompts so the
# app actually launches — it does NOT unblock runtime dynamic attach. IcedTea-Web installs
# a JNLPSecurityManager that structurally denies the attach-loaded agent's foreign code,
# regardless of permission level. A fully-green attach needs a launcher/JDK WITHOUT the
# legacy SecurityManager (modern OpenWebStart, JDK 24+). See README.md.
#
# Usage:
#   ./make_signed_jnlp.sh [TARGET_DIR] [CODEBASE_URL]
#     TARGET_DIR    default: <repo>/tests/apps/jnlp/signed
#     CODEBASE_URL  default: http://127.0.0.1:8099/
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"

SRC_JAR="${REPO_ROOT}/tests/apps/swing/target/swing-test-app-1.0.0.jar"
JAR_NAME="swing-test-app-1.0.0.jar"
MAIN_CLASS="testapp.SwingTestApp"

TARGET_DIR="${1:-${SCRIPT_DIR}/signed}"
CODEBASE="${2:-http://127.0.0.1:8099/}"
CODEBASE="${CODEBASE%/}/"   # normalize to a single trailing slash

KEYSTORE="${TARGET_DIR}/harness-keystore.jks"
STOREPASS="changeit"
ALIAS="javagui"

# --- preconditions ---------------------------------------------------------
command -v keytool  >/dev/null 2>&1 || { echo "ERROR: keytool not found (install a JDK)";  exit 1; }
command -v jarsigner >/dev/null 2>&1 || { echo "ERROR: jarsigner not found (install a JDK)"; exit 1; }
if [ ! -f "${SRC_JAR}" ]; then
  echo "ERROR: ${SRC_JAR} not found."
  echo "       Build it first:  mvn -f tests/apps/swing/pom.xml package"
  exit 1
fi

mkdir -p "${TARGET_DIR}"
cp -f "${SRC_JAR}" "${TARGET_DIR}/${JAR_NAME}"

# --- 1) stamp JNLP security manifest attributes (before signing) -----------
# Permissions/Codebase let IcedTea-Web and modern plugins accept the signed app
# without extra prompts. Harmless when the launcher ignores them.
if command -v jar >/dev/null 2>&1; then
  MF_TMP="$(mktemp)"
  {
    echo "Permissions: all-permissions"
    echo "Codebase: *"
    echo "Application-Name: Swing Test App"
    echo "Trusted-Only: false"
  } > "${MF_TMP}"
  jar ufm "${TARGET_DIR}/${JAR_NAME}" "${MF_TMP}"
  rm -f "${MF_TMP}"
else
  echo "WARN: 'jar' tool not found — skipping manifest security attributes."
fi

# --- 2) self-signed key (idempotent) ---------------------------------------
if [ ! -f "${KEYSTORE}" ]; then
  keytool -genkeypair \
    -alias "${ALIAS}" -keyalg RSA -keysize 2048 -validity 3650 \
    -dname "CN=robotframework-javaui harness, OU=test, O=javagui, L=n-a, ST=n-a, C=US" \
    -keystore "${KEYSTORE}" -storepass "${STOREPASS}" -keypass "${STOREPASS}"
fi

# --- 3) sign the jar -------------------------------------------------------
jarsigner \
  -keystore "${KEYSTORE}" -storepass "${STOREPASS}" -keypass "${STOREPASS}" \
  "${TARGET_DIR}/${JAR_NAME}" "${ALIAS}"

# --- 4) write the all-permissions JNLP -------------------------------------
cat > "${TARGET_DIR}/app.jnlp" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<jnlp spec="1.0+" codebase="${CODEBASE}" href="app.jnlp">
  <information>
    <title>Swing Test App (Web Start, signed all-permissions)</title>
    <vendor>robotframework-javaui</vendor>
    <description>Signed all-permissions JNLP test harness reusing the Swing test app.</description>
    <description kind="short">Signed Swing test app launched via Java Web Start.</description>
  </information>
  <security>
    <all-permissions/>
  </security>
  <resources>
    <j2se version="1.7+"/>
    <jar href="${JAR_NAME}" main="true"/>
  </resources>
  <application-desc main-class="${MAIN_CLASS}"/>
</jnlp>
EOF

echo
echo "Signed JNLP harness written to: ${TARGET_DIR}"
echo "  jar:      ${TARGET_DIR}/${JAR_NAME}"
echo "  jnlp:     ${TARGET_DIR}/app.jnlp"
echo "  keystore: ${KEYSTORE} (storepass/keypass: ${STOREPASS})"
echo
echo "Serve it:"
echo "  ( cd \"${TARGET_DIR}\" && python3 -m http.server 8099 --bind 127.0.0.1 )"
echo "Launch it (under xvfb for CI):"
echo "  xvfb-run -a javaws \"${CODEBASE}app.jnlp\""
echo "  # or point at a portable IcedTea-Web image:"
echo "  JAVAGUI_JAVAWS=/path/to/icedtea-web-image xvfb-run -a \\"
echo "      uv run robot -d results/jnlp tests/robot/jnlp/"
