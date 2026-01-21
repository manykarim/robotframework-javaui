"""
Invoke tasks for robotframework-javagui development.

Run `invoke --list` to see all available tasks.
Run `invoke <task> --help` for task-specific help.

Example:
    invoke build
    invoke test
    invoke release-test
    invoke release-prod
"""

import os
import shutil
import sys
from pathlib import Path

from invoke import task, Context

PTY = os.name != "nt"

ROOT = Path(__file__).parent.absolute()
PYTHON_SRC = ROOT / "python"
JAVAGUI_DIR = PYTHON_SRC / "JavaGui"
JARS_DIR = JAVAGUI_DIR / "jars"
DOCS_DIR = ROOT / "docs"
KEYWORDS_DIR = DOCS_DIR / "keywords"
AGENT_DIR = ROOT / "agent"
TESTS_DIR = ROOT / "tests"
OUTPUT_DIR = TESTS_DIR / "robot" / "output"
WHEELS_DIR = ROOT / "target" / "wheels"

# JAR files
AGENT_JAR_SOURCE = AGENT_DIR / "target" / "javagui-agent.jar"
AGENT_JAR_DEST = JARS_DIR / "javagui-agent.jar"


# =============================================================================
# Build Tasks
# =============================================================================


@task
def build_java(ctx: Context):
    """Build the Java agent JAR file."""
    print("Building Java agent...")
    with ctx.cd(str(AGENT_DIR)):
        ctx.run("mvn clean package -DskipTests", pty=PTY)

    if AGENT_JAR_SOURCE.exists():
        print(f"Agent JAR built: {AGENT_JAR_SOURCE}")
        print(f"Size: {AGENT_JAR_SOURCE.stat().st_size:,} bytes")
    else:
        print(f"ERROR: Agent JAR not found at {AGENT_JAR_SOURCE}")
        sys.exit(1)


@task
def copy_jars(ctx: Context):
    """Copy Java agent JAR to Python package for wheel bundling."""
    print("Copying JAR files to Python package...")

    if not AGENT_JAR_SOURCE.exists():
        print(f"ERROR: Source JAR not found: {AGENT_JAR_SOURCE}")
        print("Run 'invoke build-java' first.")
        sys.exit(1)

    JARS_DIR.mkdir(parents=True, exist_ok=True)
    shutil.copy2(AGENT_JAR_SOURCE, AGENT_JAR_DEST)
    print(f"Copied: {AGENT_JAR_SOURCE.name} -> {AGENT_JAR_DEST}")
    print(f"Agent JAR size: {AGENT_JAR_DEST.stat().st_size:,} bytes")


@task
def build_rust(ctx: Context, release: bool = True):
    """Build the Rust extension (development mode)."""
    print("Building Rust extension...")
    mode = "--release" if release else ""
    ctx.run(f"maturin develop {mode}", pty=PTY)
    print("Rust extension built and installed!")


@task
def build_wheel(ctx: Context):
    """Build the Python wheel for distribution."""
    print("Building Python wheel...")
    ctx.run("maturin build --release --strip", pty=PTY)

    if WHEELS_DIR.exists():
        wheels = list(WHEELS_DIR.glob("*.whl"))
        print(f"\nBuilt {len(wheels)} wheel(s):")
        for wheel in wheels:
            size = wheel.stat().st_size
            print(f"  - {wheel.name} ({size:,} bytes)")


@task(pre=[build_java, copy_jars, build_wheel])
def build(ctx: Context):
    """Full build: Java agent + copy JARs + Python wheel."""
    print("\n" + "=" * 60)
    print("FULL BUILD COMPLETE!")
    print("=" * 60)

    wheels = list(WHEELS_DIR.glob("*.whl"))
    if wheels:
        print("\nWheel contents:")
        ctx.run(f"unzip -l {wheels[0]}", warn=True)


@task
def build_dev(ctx: Context):
    """Development build: Java agent + Rust extension (no wheel)."""
    print("Development build...")
    build_java(ctx)
    copy_jars(ctx)
    build_rust(ctx, release=True)
    print("\nDevelopment build complete!")


@task
def verify(ctx: Context):
    """Verify the build by testing imports."""
    print("Verifying build...")
    ctx.run('''python -c "
from JavaGui import Swing, Swt, Rcp, get_agent_jar_path, __version__
import os
print(f'Version: {__version__}')
print(f'Swing: {Swing}')
print(f'Swt: {Swt}')
print(f'Rcp: {Rcp}')
jar_path = get_agent_jar_path()
print(f'Agent JAR: {jar_path}')
if os.path.exists(jar_path):
    size = os.path.getsize(jar_path)
    print(f'Agent JAR size: {size:,} bytes')
    print('All imports successful!')
else:
    print('ERROR: Agent JAR not found!')
    exit(1)
"''', pty=PTY)


# =============================================================================
# Release Tasks
# =============================================================================


@task(pre=[build])
def release_test(ctx: Context):
    """Release to Test PyPI."""
    print("\n" + "=" * 60)
    print("RELEASING TO TEST PYPI")
    print("=" * 60)

    wheels = list(WHEELS_DIR.glob("*.whl"))
    if not wheels:
        print("ERROR: No wheels found.")
        sys.exit(1)

    print(f"Uploading {len(wheels)} wheel(s) to Test PyPI...")
    ctx.run("maturin upload --repository testpypi target/wheels/*", pty=PTY)

    print("\nTest installation:")
    print("  pip install --index-url https://test.pypi.org/simple/ robotframework-javagui")


@task(pre=[build])
def release_prod(ctx: Context):
    """Release to Production PyPI."""
    print("\n" + "=" * 60)
    print("RELEASING TO PRODUCTION PYPI")
    print("=" * 60)

    wheels = list(WHEELS_DIR.glob("*.whl"))
    if not wheels:
        print("ERROR: No wheels found.")
        sys.exit(1)

    release_check(ctx)

    response = input("\nAre you sure you want to release to PyPI? [y/N]: ")
    if response.lower() != 'y':
        print("Release cancelled.")
        return

    ctx.run("maturin upload target/wheels/*", pty=PTY)

    print("\nInstall with:")
    print("  pip install robotframework-javagui")


@task
def release_check(ctx: Context):
    """Check release readiness."""
    print("Checking release readiness...\n")

    try:
        import tomllib
        with open(ROOT / "pyproject.toml", "rb") as f:
            pyproject = tomllib.load(f)
        version = pyproject["project"]["version"]
        print(f"Version in pyproject.toml: {version}")

        with open(ROOT / "Cargo.toml", "rb") as f:
            cargo = tomllib.load(f)
        cargo_version = cargo["package"]["version"]
        print(f"Version in Cargo.toml: {cargo_version}")

        if version != cargo_version:
            print("WARNING: Version mismatch!")
    except Exception as e:
        print(f"Could not read version: {e}")

    if AGENT_JAR_SOURCE.exists():
        print(f"Java agent JAR: OK ({AGENT_JAR_SOURCE.stat().st_size:,} bytes)")
    else:
        print("Java agent JAR: MISSING")

    if AGENT_JAR_DEST.exists():
        print(f"Bundled agent JAR: OK ({AGENT_JAR_DEST.stat().st_size:,} bytes)")
    else:
        print("Bundled agent JAR: MISSING")

    print("\nGit status:")
    ctx.run("git status --short", warn=True)


# =============================================================================
# Documentation Tasks
# =============================================================================


@task
def docs(ctx: Context):
    """Generate keyword documentation using Libdoc."""
    KEYWORDS_DIR.mkdir(parents=True, exist_ok=True)

    libraries = [
        ("JavaGui.Swing", "Swing"),
        ("JavaGui.Swt", "Swt"),
        ("JavaGui.Rcp", "Rcp"),
    ]

    for lib_path, lib_name in libraries:
        output = KEYWORDS_DIR / f"{lib_name}.html"
        print(f"Generating documentation for {lib_name}...")
        ctx.run(f"python -m robot.libdoc {lib_path} {output}", pty=PTY, warn=True)


# =============================================================================
# Testing Tasks
# =============================================================================


@task
def test(ctx: Context, suite: str = "", loglevel: str = "INFO"):
    """Run Robot Framework tests."""
    test_path = TESTS_DIR / "robot"
    if suite:
        test_path = test_path / suite

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    cmd = f"robot --loglevel {loglevel} --outputdir {OUTPUT_DIR} {test_path}"
    ctx.run(cmd, pty=PTY, warn=True)


@task
def test_dryrun(ctx: Context):
    """Run Robot Framework tests in dry-run mode."""
    print("Running Robot tests (dry-run)...\n")

    print("--- Swing Tests ---")
    ctx.run(
        "robot --dryrun --outputdir tests/robot/swing/results tests/robot/*.robot",
        pty=PTY, warn=True
    )

    print("\n--- SWT Tests ---")
    ctx.run(
        "robot --dryrun --outputdir tests/robot/swt/results "
        "tests/robot/dbeaver/controls/*.robot "
        "tests/robot/dbeaver/connection/*.robot "
        "tests/robot/dbeaver/dialogs/*.robot",
        pty=PTY, warn=True
    )

    print("\n--- RCP Tests ---")
    ctx.run(
        "robot --dryrun --outputdir tests/robot/rcp/output tests/robot/dbeaver/rcp/*.robot",
        pty=PTY, warn=True
    )


# =============================================================================
# Linting Tasks
# =============================================================================


@task
def lint(ctx: Context):
    """Run all linting checks."""
    print("Linting Python...")
    ctx.run("ruff check python/", pty=PTY, warn=True)
    print("\nLinting Rust...")
    ctx.run("cargo clippy -- -D warnings", pty=PTY, warn=True)


@task
def format_all(ctx: Context):
    """Format all code (Python and Rust)."""
    ctx.run("ruff format python/", pty=PTY)
    ctx.run("cargo fmt", pty=PTY)


# =============================================================================
# Cleaning Tasks
# =============================================================================


@task
def clean(ctx: Context):
    """Clean build artifacts."""
    patterns = ["target", "*.egg-info", "__pycache__", ".pytest_cache", "dist", "build"]

    for pattern in patterns:
        for path in ROOT.glob(f"**/{pattern}"):
            if path.is_dir():
                print(f"Removing: {path}")
                shutil.rmtree(path, ignore_errors=True)

    if JARS_DIR.exists():
        print(f"Removing: {JARS_DIR}")
        shutil.rmtree(JARS_DIR, ignore_errors=True)


# =============================================================================
# Version Tasks
# =============================================================================


@task
def version(ctx: Context, set_version: str = ""):
    """Show or set the project version."""
    if set_version:
        print(f"Setting version to {set_version}...")
        ctx.run(f'sed -i \'s/^version = .*/version = "{set_version}"/\' pyproject.toml')
        ctx.run(f'sed -i \'s/^version = .*/version = "{set_version}"/\' Cargo.toml')
        ctx.run(f'sed -i \'s/__version__ = .*/__version__ = "{set_version}"/\' python/JavaGui/__init__.py')
        print(f"Version updated to {set_version}")
    else:
        try:
            from JavaGui import __version__
            print(f"Current version: {__version__}")
        except ImportError:
            ctx.run("grep '^version' pyproject.toml", warn=True)


# =============================================================================
# Development Tasks
# =============================================================================


@task
def dev(ctx: Context):
    """Set up development environment."""
    print("Setting up development environment...\n")
    build_java(ctx)
    copy_jars(ctx)
    build_rust(ctx)
    verify(ctx)
    print("\n✅ Development environment ready!")


@task
def ci(ctx: Context):
    """Run CI checks."""
    print("Running CI checks...\n")
    lint(ctx)
    test_dryrun(ctx)
    print("\n✅ CI checks complete!")


@task(default=True)
def help(ctx: Context):
    """Show available tasks."""
    ctx.run("invoke --list", pty=PTY)
