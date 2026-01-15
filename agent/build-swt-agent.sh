#!/bin/bash
# Build script for Robot Framework SWT/RCP Agent
# Requires JDK 17 or higher

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check for Java 17+
check_java_version() {
    local java_cmd="${JAVA_HOME:-}/bin/java"
    if [ ! -x "$java_cmd" ]; then
        java_cmd="java"
    fi

    local version=$("$java_cmd" -version 2>&1 | head -1 | cut -d'"' -f2 | cut -d'.' -f1)
    if [ "$version" -lt 17 ]; then
        echo "Error: Java 17 or higher is required. Found version $version"
        echo "Please set JAVA_HOME to a JDK 17+ installation."
        echo ""
        echo "Tip: If you have DBeaver installed, its bundled JRE is Java 21:"
        echo "  export JAVA_HOME=/path/to/dbeaver/jre"
        exit 1
    fi
    echo "Using Java version: $version"
}

# Try to find a suitable JDK
find_jdk() {
    # Check common locations for JDK 17+
    local jdk_paths=(
        "/usr/lib/jvm/java-17-openjdk-amd64"
        "/usr/lib/jvm/java-21-openjdk-amd64"
        "/usr/lib/jvm/temurin-17"
        "/usr/lib/jvm/temurin-21"
        "$HOME/.sdkman/candidates/java/current"
    )

    for path in "${jdk_paths[@]}"; do
        if [ -x "$path/bin/javac" ]; then
            export JAVA_HOME="$path"
            echo "Found JDK at: $JAVA_HOME"
            return 0
        fi
    done

    # Check if current Java is sufficient
    if command -v javac &> /dev/null; then
        local version=$(javac -version 2>&1 | cut -d' ' -f2 | cut -d'.' -f1)
        if [ "$version" -ge 17 ]; then
            echo "Using system javac (version $version)"
            return 0
        fi
    fi

    return 1
}

echo "=== Robot Framework SWT/RCP Agent Build ==="
echo ""

# Try to find JDK
if [ -z "$JAVA_HOME" ]; then
    echo "JAVA_HOME not set, searching for JDK 17+..."
    if ! find_jdk; then
        echo ""
        echo "No suitable JDK found. Please install JDK 17+ or set JAVA_HOME."
        echo ""
        echo "Install options:"
        echo "  Ubuntu/Debian: sudo apt install openjdk-17-jdk"
        echo "  SDKMAN: sdk install java 17.0.9-tem"
        exit 1
    fi
fi

check_java_version

# Check for Maven
if ! command -v mvn &> /dev/null; then
    echo "Error: Maven is required but not found in PATH"
    exit 1
fi

echo ""
echo "Building SWT agent..."
mvn clean package -Pswt -DskipTests

echo ""
echo "=== Build Complete ==="
echo "Agent JAR: target/robotframework-swt-agent-1.0.0-all.jar"
echo ""
echo "Usage with DBeaver:"
echo "  ./dbeaver -vmargs -javaagent:/path/to/robotframework-swt-agent-1.0.0-all.jar"
