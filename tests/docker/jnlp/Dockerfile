# Reproducible Docker harness for automating a REAL Java Web Start (JNLP) launch with
# the robotframework-javaui runtime dynamic-attach flow, headless under Xvfb.
#
# What it PROVES (honest scope): launch a JNLP app with a real Web Start launcher
# (portable IcedTea-Web 1.8.8), discover the running JVM, and attempt runtime attach.
# Under IcedTea-Web a JNLPSecurityManager STRUCTURALLY blocks dynamic attach — regardless
# of the app's permission level (a signed all-permissions JNLP was still blocked). So this
# harness proves launch + discover + the *clear, documented SecurityManager AttachError*,
# NOT a successful attach. A green attach needs a launcher/JDK WITHOUT the legacy
# SecurityManager (modern OpenWebStart, or JDK 24+). The Robot suite asserts that either/or.
#
# Self-contained: Xvfb + a full JDK 17 (has jdk.attach for injection) + portable
# IcedTea-Web + the Robot Framework driver all run in one container. The repository is
# bind-mounted at /work so the already-built _core.abi3.so, the agent jar
# (python/JavaGui/jars/javagui-agent.jar) and the Swing test app jar are reused (host +
# image are both Ubuntu 24.04, so the abi3 extension is glibc-compatible).
#
# Build:  docker build -t jnlp-harness tests/docker/jnlp
# Run:    docker run --rm -v "$PWD":/work jnlp-harness
#
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# System deps:
#  - xvfb + x11-utils: headless X server
#  - X libraries + fontconfig: AWT/Swing native rendering under Xvfb
#  - fonts: so the launched Swing app renders real glyphs (not boxes)
#  - python3 + venv: the Robot Framework driver
#  - unzip/curl/ca-certificates: fetch + extract portable IcedTea-Web
#  - openjdk-17-jdk (FULL, not -headless): has the jdk.attach module used to inject
#    the agent into the running Web Start JVM, plus AWT/Swing native libs
RUN apt-get update && apt-get install -y --no-install-recommends \
      xvfb x11-utils \
      libx11-6 libxext6 libxrender1 libxtst6 libxi6 \
      libxrandr2 libxcomposite1 libxdamage1 libxfixes3 libxcursor1 \
      libxkbcommon0 libpango-1.0-0 libcairo2 libfreetype6 \
      libgtk-3-0 libglib2.0-0 fontconfig fonts-dejavu-core \
      ca-certificates curl unzip \
      python3 python3-pip python3-venv \
      openjdk-17-jdk \
    && rm -rf /var/lib/apt/lists/*

# Portable IcedTea-Web 1.8.8 (no root install needed — just unzip the distribution).
# It extracts to a top-level `icedtea-web-image/` directory; we place it at
# /opt/icedtea-web-image and point JAVAGUI_JAVAWS at that dir. resolve_webstart_launcher()
# treats a directory as an ITW image and launches via
#   java @bin/itw-modularjdk.args -cp <javaws.jar:linux-deps-runtime/*:jsobject.jar> \
#        net.sourceforge.jnlp.runtime.Boot <jnlp-url>
ARG ITW_URL=https://github.com/AdoptOpenJDK/IcedTea-Web/releases/download/icedtea-web-1.8.8/icedtea-web-1.8.8.portable.bin.zip
RUN curl -fsSL "$ITW_URL" -o /tmp/itw.zip \
    && unzip -q /tmp/itw.zip -d /opt \
    && rm /tmp/itw.zip \
    && test -f /opt/icedtea-web-image/share/icedtea-web/javaws.jar
ENV JAVAGUI_JAVAWS=/opt/icedtea-web-image

# IcedTea-Web deployment config: accept unsigned apps and suppress the interactive trust /
# grant dialogs so the launch runs unattended under Xvfb. Boot reads this from
# $HOME/.config/icedtea-web/deployment.properties (container runs as root -> /root).
RUN mkdir -p /root/.config/icedtea-web \
    && printf '%s\n' \
        'deployment.security.level=ALLOW_UNSIGNED' \
        'deployment.security.askgrantdialog.show=false' \
        'deployment.security.askgrantdialog.notinca=false' \
        'deployment.security.notinca.warning=false' \
        'deployment.security.https.dialog.hide=true' \
        'deployment.trust.autosign=true' \
        > /root/.config/icedtea-web/deployment.properties

# Robot Framework driver deps (JavaGui itself is used from the bind-mounted /work/python
# via PYTHONPATH, reusing the prebuilt _core.abi3.so and the bundled agent jar).
RUN python3 -m venv /venv \
    && /venv/bin/pip install --no-cache-dir \
        robotframework robotframework-assertion-engine docutils
ENV PATH=/venv/bin:$PATH

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
