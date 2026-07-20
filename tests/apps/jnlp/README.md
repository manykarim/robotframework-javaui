# Java Web Start (JNLP) test harness

A deterministic, opt-in harness that exercises `Launch Web Start Application` against a
real Java Web Start launcher. It **reuses the already-built Swing test app**
(`tests/apps/swing/target/swing-test-app-1.0.0.jar`, `Main-Class testapp.SwingTestApp`) —
no separate app is authored or maintained here.

Contents:

| File | Purpose |
|------|---------|
| `app.jnlp` | Sandbox (default-permission) JNLP skeleton, codebase `http://127.0.0.1:8099/`. |
| `make_signed_jnlp.sh` | Builds a **signed all-permissions** variant into a target dir (keytool + jarsigner, storepass `changeit`). |
| `../../robot/jnlp/01_webstart.robot` | Self-skipping Robot suite that serves + launches + attaches. |

## The honest constraint (read this first)

Under **IcedTea-Web**, a JNLP app installs a `JNLPSecurityManager` that **structurally
blocks runtime dynamic attach** — it cannot classify the attach-loaded agent's foreign
code, so agent init is denied. This was verified even for a **signed all-permissions**
JNLP: *still blocked*. It is **independent of the app's permission level**.

Therefore:

- A **fully-green attach** requires a launcher/JDK **without the legacy SecurityManager**:
  modern **OpenWebStart**, or **JDK 24+** (where the SecurityManager is removed).
- On a legacy IcedTea-Web `javaws`, `Launch Web Start Application` raises a clear
  `AttachError` that names the SecurityManager. The Robot suite treats that specific error
  as an expected, documented outcome — and treats any *other* failure as a real defect.

`make_signed_jnlp.sh` signs to all-permissions only to clear IcedTea-Web's **trust
prompts** so the app actually launches — signing does **not** unblock attach.

## Recipe

### 1. Build the Swing test app (once)

```bash
mvn -f tests/apps/swing/pom.xml package
# => tests/apps/swing/target/swing-test-app-1.0.0.jar  (Main-Class testapp.SwingTestApp)
```

### 2a. Sandbox JNLP (no signing)

`app.jnlp` in this directory is the skeleton. Its shape:

```xml
<jnlp spec="1.0+" codebase="http://127.0.0.1:8099/" href="app.jnlp">
  <information> ... </information>
  <resources>
    <j2se version="1.7+"/>
    <jar href="swing-test-app-1.0.0.jar" main="true"/>
  </resources>
  <application-desc main-class="testapp.SwingTestApp"/>
</jnlp>
```

Stage `app.jnlp` and the swing jar into a single served directory:

```bash
mkdir -p /tmp/jnlp-serve
cp tests/apps/jnlp/app.jnlp                              /tmp/jnlp-serve/
cp tests/apps/swing/target/swing-test-app-1.0.0.jar      /tmp/jnlp-serve/
```

### 2b. Signed all-permissions JNLP

```bash
tests/apps/jnlp/make_signed_jnlp.sh [TARGET_DIR] [CODEBASE_URL]
# defaults: TARGET_DIR=tests/apps/jnlp/signed   CODEBASE_URL=http://127.0.0.1:8099/
```

It copies the jar, stamps JNLP security attributes into its manifest
(`Permissions: all-permissions`, `Codebase: *`), self-signs it with a throwaway keystore
(`keytool -genkeypair` + `jarsigner`, storepass/keypass `changeit`, alias `javagui`), and
writes an all-permissions `app.jnlp` (`<security><all-permissions/></security>`) into the
target dir. Everything needed to serve is left in `TARGET_DIR`.

### 3. Serve over 127.0.0.1

Bind to loopback only (never `0.0.0.0`):

```bash
cd /tmp/jnlp-serve          # or the signed TARGET_DIR
python3 -m http.server 8099 --bind 127.0.0.1
```

The port must match the `codebase` in `app.jnlp` (`8099` by default).

### 4. Launch with a Web Start launcher

With `javaws` on PATH (IcedTea-Web) under a headless display:

```bash
xvfb-run -a javaws http://127.0.0.1:8099/app.jnlp
```

With a **portable IcedTea-Web image** (no root needed — unzip the ITW distribution
anywhere) or **OpenWebStart**, point `JAVAGUI_JAVAWS` at the launcher binary **or** the
image directory:

```bash
export JAVAGUI_JAVAWS=/path/to/icedtea-web-image        # a dir => treated as an ITW image
# or
export JAVAGUI_JAVAWS=/opt/openwebstart/bin/javaws      # a binary launcher
```

`resolve_webstart_launcher()` accepts either: a directory is launched via the image's
`share/icedtea-web/javaws.jar` + `net.sourceforge.jnlp.runtime.Boot`; a file is run
directly. Requires a JDK 17+ with `jdk.attach` on this host to inject the agent (set
`JAVAGUI_JAVA` if your default `java` is a trimmed JRE).

### 5. Run the Robot suite

```bash
JAVAGUI_JAVAWS=/path/to/icedtea-web-image \
  xvfb-run -a uv run robot -d results/jnlp tests/robot/jnlp/
```

The suite self-skips unless a launcher (`JAVAGUI_JAVAWS` or `javaws` on PATH) **and** a
`DISPLAY` are present, and the swing jar is built. It:

1. stages `app.jnlp` + the swing jar into `${OUTPUT DIR}/jnlp-serve`,
2. starts `python3 -m http.server 8099 --bind 127.0.0.1` (Suite Setup),
3. runs `Launch Web Start Application http://127.0.0.1:8099/app.jnlp`,
4. asserts **either** a live connection (and locates JButtons to prove it) **or** that the
   failure is the documented SecurityManager `AttachError` — never success unconditionally,
5. tears down the launcher (`pkill -f app.jnlp`) and the http server (Suite Teardown).

## Environment notes

- **No sudo needed.** A portable IcedTea-Web zip works without root; unzip and point
  `JAVAGUI_JAVAWS` at the image dir.
- **JDK 17** has `jdk.attach`, which is what performs the runtime agent injection.
- To get a **green attach**, use a launcher/JDK without the legacy SecurityManager
  (modern OpenWebStart, JDK 24+).
