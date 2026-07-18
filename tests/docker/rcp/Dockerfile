# Reproducible Docker harness for automating a REAL public Eclipse RCP application
# (DBeaver Community Edition) with the robotframework-javaui agent, headless under Xvfb,
# with visual (framebuffer screenshot) confirmation.
#
# Self-contained: Xvfb + DBeaver (bundled JDK) + the Java agent + the Robot Framework
# driver all run in one container. The repository is bind-mounted at /work so the
# already-built _core.abi3.so and agent jar are reused (host + image are both
# Ubuntu 24.04, so the abi3 extension is glibc-compatible).
#
# Build:  docker build -t rcp-dbeaver-harness tests/docker/rcp
# Run:    docker run --rm -v "$PWD":/work rcp-dbeaver-harness
#
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# System deps:
#  - xvfb + x11-utils: headless X server + framebuffer tools
#  - imagemagick: `import` for whole-screen PNG capture (visual confirmation)
#  - GTK3 + X libraries: DBeaver / SWT native rendering
#  - python3 + venv: the Robot Framework driver
#  - fonts: so rendered screenshots are not blank/boxes
RUN apt-get update && apt-get install -y --no-install-recommends \
      xvfb x11-utils imagemagick \
      libgtk-3-0 libglib2.0-0 \
      libx11-6 libxext6 libxrender1 libxtst6 libxi6 \
      libxrandr2 libxcomposite1 libxdamage1 libxfixes3 libxcursor1 \
      libxkbcommon0 libpango-1.0-0 libcairo2 libfreetype6 \
      ca-certificates curl \
      python3 python3-pip python3-venv \
      fonts-dejavu-core \
      openjdk-21-jdk-headless \
    && rm -rf /var/lib/apt/lists/*

# DBeaver ships a trimmed jlink JRE that OMITS the java.instrument module, so a
# -javaagent cannot attach to it (libinstrument.so is absent). We install a FULL
# JDK 21 and point DBeaver's -vm at it in the entrypoint. This is representative of
# the real obstacle to automating packaged RCP products.
ENV FULL_JAVA=/usr/lib/jvm/java-21-openjdk-amd64/bin/java

# DBeaver Community Edition (bundled OpenJDK 21). Pinned for reproducibility;
# override with --build-arg DBEAVER_URL=... to test another version.
ARG DBEAVER_URL=https://github.com/dbeaver/dbeaver/releases/download/26.1.2/dbeaver-ce-26.1.2-linux-x86_64.tar.gz
RUN curl -fsSL "$DBEAVER_URL" -o /tmp/dbeaver.tar.gz \
    && mkdir -p /opt \
    && tar xzf /tmp/dbeaver.tar.gz -C /opt \
    && rm /tmp/dbeaver.tar.gz \
    && test -x /opt/dbeaver/dbeaver

# Allow ImageMagick to read the X root window and write PNGs (its default policy on
# some distros restricts this). We only ever capture our own Xvfb, so this is safe.
RUN if [ -f /etc/ImageMagick-6/policy.xml ]; then \
      sed -i 's/rights="none" pattern="\(PNG\|XWD\)"/rights="read|write" pattern="\1"/g' /etc/ImageMagick-6/policy.xml || true; \
    fi

# Robot Framework driver deps (JavaGui itself is used from the bind-mounted /work/python
# via PYTHONPATH, reusing the prebuilt _core.abi3.so).
RUN python3 -m venv /venv \
    && /venv/bin/pip install --no-cache-dir \
        robotframework robotframework-assertion-engine docutils
ENV PATH=/venv/bin:$PATH

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
