# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust core (PyO3 bindings, locator parsing, RPC/connection logic).
- `python/JavaGui/`: Python package that exposes Swing/Swt/Rcp keywords; bundled JARs live in `python/JavaGui/jars/`.
- `agent/`: Java agent (Maven project; built JARs in `agent/target/`).
- `tests/robot/`: Robot Framework suites (numbered `NN_*.robot`), plus toolkit-specific folders.
- `tests/python/` and `tests/unit/`: Pytest suites for Python bindings and smaller unit checks.
- `docs/`, `schemas/`, `scripts/`: supporting docs, schemas, and helper scripts.

## Build, Test, and Development Commands
- `uv pip install -e ".[dev]"`: install dev dependencies (project uses `uv`).
- `invoke build-dev`: build the Java agent (fat JAR), copy it into the Python package, and compile the Rust extension for local use.
- `invoke build`: full build (agent + wheel) for distribution.
- `cd tests/apps && mvn package`: build the Swing/SWT/RCP test applications used by Robot suites.
- `uv run robot tests/robot/`: run all Robot suites (outputs under `tests/robot/output/`).
- `uv run robot tests/robot/02_locators.robot`: run a specific Robot suite.
- `uv run pytest tests/python/`: run Python tests (pytest config lives in `pyproject.toml`).
- `invoke lint`: run `ruff` on Python and `cargo clippy -D warnings` on Rust.
- `invoke format-all`: format Python (`ruff format`) and Rust (`cargo fmt`).

## Coding Style & Naming Conventions
- Python: 4-space indentation, 100-char lines, formatted with `ruff format`, linted by `ruff`, typed with `mypy`.
- Rust: use `cargo fmt`; keep clippy clean (`cargo clippy -D warnings`).
- Robot: `.robot` files use numeric prefixes (`01_connection.robot`) and readable test-case names.

## Testing Guidelines
- Build the test apps in `tests/apps/` before running Robot suites (they exercise Swing/SWT/RCP UIs).
- Robot Framework tests live in `tests/robot/`; run via `uv run robot ...`, outputs default to `tests/robot/output/`.
- Pytest files follow `test_*.py` naming, with tests named `test_*` (see `tests/python/pytest.ini`).
- Add coverage for new keywords/locators with both Robot suites and focused pytest unit tests where possible.

## Commit & Pull Request Guidelines
- Commit messages follow a conventional prefix (examples in history: `fix:`, `docs:`). Use `type: summary`.
- PRs should describe behavior changes, list test commands run, and link related issues when applicable.

## Configuration Notes
- Build metadata is in `Cargo.toml` and `pyproject.toml`; update both if you change versioning.
- Java agent artifact is expected at `agent/target/javagui-agent.jar`.
