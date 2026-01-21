"""
Tests for RCP (Eclipse Rich Client Platform) assertion keywords.

These tests verify RCP-specific library keywords including view/editor
assertions, perspective assertions, dirty state assertions, and workbench
assertions with AssertionEngine integration.
"""

import pytest
import time
from unittest.mock import Mock, MagicMock, patch
from typing import Dict, Any, List, Optional


# Mock RCP Element/Widget
class MockRcpWidget:
    """Mock RCP widget for testing."""

    def __init__(
        self,
        id: int = 1,
        class_name: str = "org.eclipse.swt.widgets.Button",
        name: Optional[str] = "testBtn",
        text: Optional[str] = "Click Me",
        visible: bool = True,
        enabled: bool = True,
        properties: Dict[str, Any] = None,
    ):
        self.id = id
        self.class_name = class_name
        self.name = name
        self.text = text
        self.is_visible = visible
        self.is_enabled = enabled
        self._properties = properties or {}

    def get_property(self, name: str) -> Any:
        if name == "text":
            return self.text
        if name == "visible":
            return self.is_visible
        if name == "enabled":
            return self.is_enabled
        return self._properties.get(name)


# Mock RCP Library
class MockRcpLibrary:
    """Mock RCP Library for testing assertion keywords."""

    def __init__(self, timeout: float = 10.0):
        self.timeout = timeout
        self._connected = False
        self._widgets: Dict[str, MockRcpWidget] = {}

        # RCP-specific state
        self._active_perspective = "org.eclipse.jdt.ui.JavaPerspective"
        self._available_perspectives = [
            "org.eclipse.jdt.ui.JavaPerspective",
            "org.eclipse.debug.ui.DebugPerspective",
            "org.eclipse.team.ui.TeamSynchronizingPerspective",
        ]
        self._open_views = [
            {"id": "org.eclipse.jdt.ui.PackageExplorer", "title": "Package Explorer", "visible": True},
            {"id": "org.eclipse.ui.views.ProblemView", "title": "Problems", "visible": True},
            {"id": "org.eclipse.ui.console.ConsoleView", "title": "Console", "visible": False},
        ]
        self._open_editors = [
            {"id": "1", "title": "Main.java", "dirty": False, "path": "/project/src/Main.java"},
            {"id": "2", "title": "*Config.xml", "dirty": True, "path": "/project/config.xml"},
            {"id": "3", "title": "README.md", "dirty": False, "path": "/project/README.md"},
        ]
        self._active_editor = "Main.java"

        self._setup_default_widgets()

    def _setup_default_widgets(self) -> None:
        """Set up default mock widgets for testing."""
        self._widgets = {
            "Button#submitBtn": MockRcpWidget(
                id=1, name="submitBtn", text="Submit",
                class_name="org.eclipse.swt.widgets.Button"
            ),
            "Text#searchField": MockRcpWidget(
                id=2, name="searchField", text="",
                class_name="org.eclipse.swt.widgets.Text"
            ),
        }

    def connect_to_application(
        self, app: str, host: str = "localhost", port: int = 5679, timeout: float = None
    ):
        self._connected = True

    def connect_to_swt_application(
        self, app: str, host: str = "localhost", port: int = 5679, timeout: float = None
    ):
        self._connected = True

    def disconnect(self):
        self._connected = False

    def is_connected(self) -> bool:
        return self._connected

    def set_timeout(self, timeout: float) -> float:
        old_timeout = self.timeout
        self.timeout = timeout
        return old_timeout

    # Widget Methods
    def find_widget(self, locator: str) -> MockRcpWidget:
        if locator in self._widgets:
            return self._widgets[locator]
        for key, elem in self._widgets.items():
            if locator in key or (elem.name and locator.endswith(f"#{elem.name}")):
                return elem
        raise Exception(f"Widget not found: {locator}")

    def find_widgets(self, locator: str) -> List[MockRcpWidget]:
        results = []
        for key, elem in self._widgets.items():
            if locator in key:
                results.append(elem)
        return results

    def get_widget_text(self, locator: str) -> str:
        return self.find_widget(locator).text or ""

    # Perspective Methods
    def get_active_perspective(self) -> str:
        """Get the currently active perspective ID."""
        return self._active_perspective

    def get_available_perspectives(self) -> List[str]:
        """Get list of available perspective IDs."""
        return self._available_perspectives.copy()

    def open_perspective(self, perspective_id: str) -> None:
        """Open a perspective by ID."""
        if perspective_id in self._available_perspectives:
            self._active_perspective = perspective_id
        else:
            raise Exception(f"Perspective not found: {perspective_id}")

    def reset_perspective(self) -> None:
        """Reset the current perspective to its default layout."""
        pass

    # View Methods
    def get_open_views(self) -> List[Dict[str, Any]]:
        """Get list of open views."""
        return [v.copy() for v in self._open_views]

    def show_view(self, view_id: str, secondary_id: Optional[str] = None) -> None:
        """Show a view by ID."""
        for view in self._open_views:
            if view["id"] == view_id:
                view["visible"] = True
                return
        # Add new view
        self._open_views.append({
            "id": view_id,
            "title": view_id.split(".")[-1],
            "visible": True
        })

    def close_view(self, view_id: str, secondary_id: Optional[str] = None) -> None:
        """Close a view by ID."""
        self._open_views = [v for v in self._open_views if v["id"] != view_id]

    def activate_view(self, view_id: str) -> None:
        """Activate a view."""
        for view in self._open_views:
            if view["id"] == view_id:
                view["visible"] = True
                return
        raise Exception(f"View not found: {view_id}")

    def view_should_be_visible(self, view_id: str) -> None:
        """Assert view is visible."""
        for view in self._open_views:
            if view["id"] == view_id and view["visible"]:
                return
        raise AssertionError(f"View '{view_id}' is not visible")

    def get_view_widget(self, view_id: str, locator: str) -> MockRcpWidget:
        """Get a widget within a view."""
        # Simplified: return any matching widget
        return self.find_widget(locator)

    def get_view_count(self) -> int:
        """Get count of open views."""
        return len(self._open_views)

    def get_visible_view_count(self) -> int:
        """Get count of visible views."""
        return len([v for v in self._open_views if v["visible"]])

    # Editor Methods
    def get_open_editors(self) -> List[Dict[str, Any]]:
        """Get list of open editors."""
        return [e.copy() for e in self._open_editors]

    def get_active_editor(self) -> Optional[str]:
        """Get active editor title."""
        return self._active_editor

    def open_editor(self, file_path: str) -> None:
        """Open an editor for a file."""
        title = file_path.split("/")[-1]
        editor = {
            "id": str(len(self._open_editors) + 1),
            "title": title,
            "dirty": False,
            "path": file_path
        }
        self._open_editors.append(editor)
        self._active_editor = title

    def close_editor(self, title: str, save: bool = False) -> None:
        """Close an editor by title."""
        self._open_editors = [e for e in self._open_editors if e["title"] != title and e["title"] != f"*{title}"]
        if self._active_editor == title or self._active_editor == f"*{title}":
            if self._open_editors:
                self._active_editor = self._open_editors[0]["title"]
            else:
                self._active_editor = None

    def close_all_editors(self, save: bool = False) -> bool:
        """Close all editors."""
        self._open_editors = []
        self._active_editor = None
        return True

    def save_editor(self, title: Optional[str] = None) -> None:
        """Save an editor."""
        target = title or self._active_editor
        for editor in self._open_editors:
            if editor["title"] == target or editor["title"] == f"*{target}":
                editor["dirty"] = False
                editor["title"] = editor["title"].lstrip("*")
                return

    def save_all_editors(self) -> None:
        """Save all editors."""
        for editor in self._open_editors:
            editor["dirty"] = False
            editor["title"] = editor["title"].lstrip("*")

    def activate_editor(self, title: str) -> None:
        """Activate an editor by title."""
        for editor in self._open_editors:
            if editor["title"] == title or editor["title"] == f"*{title}":
                self._active_editor = editor["title"]
                return
        raise Exception(f"Editor not found: {title}")

    def is_editor_dirty(self, file_path_or_title: str) -> bool:
        """Check if an editor has unsaved changes."""
        for editor in self._open_editors:
            if (editor["path"] == file_path_or_title or
                editor["title"] == file_path_or_title or
                editor["title"] == f"*{file_path_or_title}"):
                return editor["dirty"]
        raise Exception(f"Editor not found: {file_path_or_title}")

    def editor_should_be_dirty(self, file_path: str) -> None:
        """Assert editor has unsaved changes."""
        if not self.is_editor_dirty(file_path):
            raise AssertionError(f"Editor '{file_path}' should be dirty but is not")

    def editor_should_not_be_dirty(self, file_path: str) -> None:
        """Assert editor has no unsaved changes."""
        if self.is_editor_dirty(file_path):
            raise AssertionError(f"Editor '{file_path}' should not be dirty but is")

    def get_editor_widget(self, title: str, locator: str) -> MockRcpWidget:
        """Find a widget within an editor."""
        return self.find_widget(locator)

    def get_editor_count(self) -> int:
        """Get count of open editors."""
        return len(self._open_editors)

    def get_dirty_editor_count(self) -> int:
        """Get count of dirty (unsaved) editors."""
        return len([e for e in self._open_editors if e["dirty"]])

    # Command Methods
    def execute_command(self, command_id: str) -> None:
        """Execute an Eclipse command."""
        pass  # Mock implementation

    def get_available_commands(self, category: Optional[str] = None) -> List[str]:
        """Get available commands."""
        return [
            "org.eclipse.ui.file.save",
            "org.eclipse.ui.file.saveAll",
            "org.eclipse.ui.file.close",
            "org.eclipse.ui.edit.copy",
            "org.eclipse.ui.edit.paste",
        ]

    # Workbench Methods
    def get_workbench_info(self) -> Dict[str, Any]:
        """Get workbench information."""
        return {
            "title": "Eclipse IDE",
            "active_perspective": self._active_perspective,
            "view_count": len(self._open_views),
            "editor_count": len(self._open_editors),
        }

    def wait_for_workbench(self, timeout: Optional[float] = None) -> None:
        """Wait for workbench to be ready."""
        pass  # Mock implementation

    # Table Methods
    def get_table_row_count(self, locator: str) -> int:
        return 10

    def get_table_cell(self, locator: str, row: int, col: int) -> str:
        return f"Cell[{row},{col}]"


class RcpNotFoundError(Exception):
    """RCP element not found error."""
    pass


# Try to import assertion-related modules
try:
    from JavaGui.assertions import (
        AssertionConfig,
        with_retry_assertion,
        numeric_assertion_with_retry,
        state_assertion_with_retry,
        ElementState,
    )
    from assertionengine import AssertionOperator
    ASSERTIONS_AVAILABLE = True
except ImportError:
    ASSERTIONS_AVAILABLE = False

    # Mock AssertionOperator for tests
    class MockAssertionOperator:
        equal = "=="
        not_equal = "!="
        contains = "*="
        not_contains = "not_contains"
        greater_than = ">"
        less_than = "<"
        greater_than_or_equal = ">="
        less_than_or_equal = "<="
        starts = "^="
        ends = "$="
        matches = "matches"

        def __getitem__(self, key):
            mapping = {
                "==": self.equal,
                "!=": self.not_equal,
                ">": self.greater_than,
                "<": self.less_than,
                ">=": self.greater_than_or_equal,
                "<=": self.less_than_or_equal,
                "greater than": self.greater_than,
                "less than": self.less_than,
            }
            return mapping.get(key, key)

    AssertionOperator = MockAssertionOperator()


# =============================================================================
# RCP Perspective Assertion Tests
# =============================================================================


class TestRcpPerspectiveAssertions:
    """Tests for RCP perspective-related assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_get_active_perspective(self, mock_lib):
        """Test getting active perspective."""
        perspective = mock_lib.get_active_perspective()
        assert perspective == "org.eclipse.jdt.ui.JavaPerspective"

    def test_get_active_perspective_with_assertion(self, mock_lib):
        """Test getting active perspective with assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            mock_lib.get_active_perspective,
            AssertionOperator.equal,
            "org.eclipse.jdt.ui.JavaPerspective",
            timeout=1.0
        )
        assert result == "org.eclipse.jdt.ui.JavaPerspective"

    def test_get_active_perspective_contains(self, mock_lib):
        """Test active perspective contains substring."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            mock_lib.get_active_perspective,
            AssertionOperator.contains,
            "Java",
            timeout=1.0
        )
        assert "Java" in result

    def test_get_active_perspective_starts(self, mock_lib):
        """Test active perspective starts with."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            mock_lib.get_active_perspective,
            AssertionOperator.starts,
            "org.eclipse",
            timeout=1.0
        )
        assert result.startswith("org.eclipse")

    def test_perspective_assertion_fails(self, mock_lib):
        """Test perspective assertion failure."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError):
            with_retry_assertion(
                mock_lib.get_active_perspective,
                AssertionOperator.equal,
                "org.eclipse.debug.ui.DebugPerspective",
                timeout=0.3,
                interval=0.1
            )

    def test_open_perspective_changes_active(self, mock_lib):
        """Test opening perspective changes active."""
        mock_lib.open_perspective("org.eclipse.debug.ui.DebugPerspective")
        assert mock_lib.get_active_perspective() == "org.eclipse.debug.ui.DebugPerspective"

    def test_get_available_perspectives(self, mock_lib):
        """Test getting available perspectives."""
        perspectives = mock_lib.get_available_perspectives()
        assert len(perspectives) == 3
        assert "org.eclipse.jdt.ui.JavaPerspective" in perspectives

    def test_perspective_switch_with_retry(self, mock_lib):
        """Test perspective switch with retry assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        call_count = [0]
        original_get = mock_lib.get_active_perspective

        def changing_perspective():
            call_count[0] += 1
            if call_count[0] < 3:
                return "org.eclipse.jdt.ui.JavaPerspective"
            return "org.eclipse.debug.ui.DebugPerspective"

        mock_lib.get_active_perspective = changing_perspective

        result = with_retry_assertion(
            mock_lib.get_active_perspective,
            AssertionOperator.equal,
            "org.eclipse.debug.ui.DebugPerspective",
            timeout=5.0,
            interval=0.1
        )
        assert result == "org.eclipse.debug.ui.DebugPerspective"


# =============================================================================
# RCP View Assertion Tests
# =============================================================================


class TestRcpViewAssertions:
    """Tests for RCP view-related assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_get_open_views(self, mock_lib):
        """Test getting open views."""
        views = mock_lib.get_open_views()
        assert len(views) == 3

    def test_get_view_count(self, mock_lib):
        """Test getting view count."""
        count = mock_lib.get_view_count()
        assert count == 3

    def test_get_view_count_with_assertion(self, mock_lib):
        """Test view count with numeric assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            mock_lib.get_view_count,
            AssertionOperator.equal,
            3,
            timeout=1.0
        )
        assert result == 3

    def test_get_view_count_greater_than(self, mock_lib):
        """Test view count > operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            mock_lib.get_view_count,
            AssertionOperator["greater than"],
            0,
            timeout=1.0
        )
        assert result > 0

    def test_get_visible_view_count(self, mock_lib):
        """Test getting visible view count."""
        count = mock_lib.get_visible_view_count()
        assert count == 2  # Two views are visible

    def test_visible_view_count_assertion(self, mock_lib):
        """Test visible view count with assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            mock_lib.get_visible_view_count,
            AssertionOperator[">="],
            1,
            timeout=1.0
        )
        assert result >= 1

    def test_view_should_be_visible_pass(self, mock_lib):
        """Test view visibility assertion passes."""
        mock_lib.view_should_be_visible("org.eclipse.jdt.ui.PackageExplorer")

    def test_view_should_be_visible_fail(self, mock_lib):
        """Test view visibility assertion fails."""
        with pytest.raises(AssertionError):
            mock_lib.view_should_be_visible("org.eclipse.ui.console.ConsoleView")

    def test_show_view_makes_visible(self, mock_lib):
        """Test showing view makes it visible."""
        mock_lib.show_view("org.eclipse.ui.console.ConsoleView")
        mock_lib.view_should_be_visible("org.eclipse.ui.console.ConsoleView")

    def test_close_view_removes_from_list(self, mock_lib):
        """Test closing view removes it from list."""
        initial_count = mock_lib.get_view_count()
        mock_lib.close_view("org.eclipse.jdt.ui.PackageExplorer")
        assert mock_lib.get_view_count() == initial_count - 1

    def test_view_count_after_operations(self, mock_lib):
        """Test view count changes after operations."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        # Initial count
        initial_count = mock_lib.get_view_count()

        # Add a view
        mock_lib.show_view("org.eclipse.ui.views.NewView")

        result = numeric_assertion_with_retry(
            mock_lib.get_view_count,
            AssertionOperator.equal,
            initial_count + 1,
            timeout=1.0
        )
        assert result == initial_count + 1


# =============================================================================
# RCP Editor Assertion Tests
# =============================================================================


class TestRcpEditorAssertions:
    """Tests for RCP editor-related assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_get_open_editors(self, mock_lib):
        """Test getting open editors."""
        editors = mock_lib.get_open_editors()
        assert len(editors) == 3

    def test_get_editor_count(self, mock_lib):
        """Test getting editor count."""
        count = mock_lib.get_editor_count()
        assert count == 3

    def test_get_editor_count_with_assertion(self, mock_lib):
        """Test editor count with numeric assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            mock_lib.get_editor_count,
            AssertionOperator.equal,
            3,
            timeout=1.0
        )
        assert result == 3

    def test_get_editor_count_greater_than(self, mock_lib):
        """Test editor count > operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            mock_lib.get_editor_count,
            AssertionOperator["greater than"],
            0,
            timeout=1.0
        )
        assert result > 0

    def test_get_active_editor(self, mock_lib):
        """Test getting active editor."""
        editor = mock_lib.get_active_editor()
        assert editor == "Main.java"

    def test_get_active_editor_with_assertion(self, mock_lib):
        """Test active editor with assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            mock_lib.get_active_editor,
            AssertionOperator.equal,
            "Main.java",
            timeout=1.0
        )
        assert result == "Main.java"

    def test_get_active_editor_contains(self, mock_lib):
        """Test active editor contains assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            mock_lib.get_active_editor,
            AssertionOperator.contains,
            ".java",
            timeout=1.0
        )
        assert ".java" in result

    def test_activate_editor_changes_active(self, mock_lib):
        """Test activating editor changes active editor."""
        mock_lib.activate_editor("README.md")
        assert mock_lib.get_active_editor() == "README.md"

    def test_close_editor_reduces_count(self, mock_lib):
        """Test closing editor reduces count."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        initial_count = mock_lib.get_editor_count()
        mock_lib.close_editor("README.md")

        result = numeric_assertion_with_retry(
            mock_lib.get_editor_count,
            AssertionOperator.equal,
            initial_count - 1,
            timeout=1.0
        )
        assert result == initial_count - 1

    def test_close_all_editors(self, mock_lib):
        """Test closing all editors."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        mock_lib.close_all_editors()

        result = numeric_assertion_with_retry(
            mock_lib.get_editor_count,
            AssertionOperator.equal,
            0,
            timeout=1.0
        )
        assert result == 0

    def test_open_editor_adds_to_list(self, mock_lib):
        """Test opening editor adds to list."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        initial_count = mock_lib.get_editor_count()
        mock_lib.open_editor("/project/src/NewFile.java")

        result = numeric_assertion_with_retry(
            mock_lib.get_editor_count,
            AssertionOperator.equal,
            initial_count + 1,
            timeout=1.0
        )
        assert result == initial_count + 1


# =============================================================================
# RCP Dirty State Assertion Tests
# =============================================================================


class TestRcpDirtyStateAssertions:
    """Tests for RCP editor dirty state assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_is_editor_dirty_true(self, mock_lib):
        """Test is_editor_dirty returns True for dirty editor."""
        # Config.xml is marked as dirty
        assert mock_lib.is_editor_dirty("/project/config.xml") == True

    def test_is_editor_dirty_false(self, mock_lib):
        """Test is_editor_dirty returns False for clean editor."""
        assert mock_lib.is_editor_dirty("/project/src/Main.java") == False

    def test_editor_should_be_dirty_pass(self, mock_lib):
        """Test editor_should_be_dirty passes for dirty editor."""
        mock_lib.editor_should_be_dirty("/project/config.xml")

    def test_editor_should_be_dirty_fail(self, mock_lib):
        """Test editor_should_be_dirty fails for clean editor."""
        with pytest.raises(AssertionError) as exc_info:
            mock_lib.editor_should_be_dirty("/project/src/Main.java")
        assert "should be dirty" in str(exc_info.value)

    def test_editor_should_not_be_dirty_pass(self, mock_lib):
        """Test editor_should_not_be_dirty passes for clean editor."""
        mock_lib.editor_should_not_be_dirty("/project/src/Main.java")

    def test_editor_should_not_be_dirty_fail(self, mock_lib):
        """Test editor_should_not_be_dirty fails for dirty editor."""
        with pytest.raises(AssertionError) as exc_info:
            mock_lib.editor_should_not_be_dirty("/project/config.xml")
        assert "should not be dirty" in str(exc_info.value)

    def test_get_dirty_editor_count(self, mock_lib):
        """Test getting dirty editor count."""
        count = mock_lib.get_dirty_editor_count()
        assert count == 1  # Only Config.xml is dirty

    def test_dirty_editor_count_assertion(self, mock_lib):
        """Test dirty editor count with assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            mock_lib.get_dirty_editor_count,
            AssertionOperator.equal,
            1,
            timeout=1.0
        )
        assert result == 1

    def test_dirty_count_changes_after_save(self, mock_lib):
        """Test dirty count changes after saving."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        # Save all editors
        mock_lib.save_all_editors()

        result = numeric_assertion_with_retry(
            mock_lib.get_dirty_editor_count,
            AssertionOperator.equal,
            0,
            timeout=1.0
        )
        assert result == 0

    def test_save_editor_clears_dirty(self, mock_lib):
        """Test saving editor clears dirty flag."""
        mock_lib.save_editor("*Config.xml")
        assert mock_lib.is_editor_dirty("/project/config.xml") == False

    def test_dirty_state_with_retry(self, mock_lib):
        """Test dirty state assertion with retry."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        call_count = [0]

        def changing_dirty_count():
            call_count[0] += 1
            if call_count[0] < 3:
                return 1
            return 0

        mock_lib.get_dirty_editor_count = changing_dirty_count

        result = numeric_assertion_with_retry(
            mock_lib.get_dirty_editor_count,
            AssertionOperator.equal,
            0,
            timeout=5.0,
            interval=0.1
        )
        assert result == 0


# =============================================================================
# RCP Timeout and Retry Behavior Tests
# =============================================================================


class TestRcpTimeoutBehavior:
    """Tests for RCP assertion timeout behavior."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_immediate_success_is_fast(self, mock_lib):
        """Test assertion that passes immediately is fast."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        start = time.time()
        result = with_retry_assertion(
            mock_lib.get_active_perspective,
            AssertionOperator.contains,
            "Java",
            timeout=5.0,
            interval=0.1
        )
        elapsed = time.time() - start
        assert elapsed < 0.5
        assert "Java" in result

    def test_timeout_honored_for_views(self, mock_lib):
        """Test timeout is honored for view count assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        start = time.time()
        with pytest.raises(AssertionError):
            numeric_assertion_with_retry(
                mock_lib.get_view_count,
                AssertionOperator.equal,
                100,  # Won't match
                timeout=0.5,
                interval=0.1
            )
        elapsed = time.time() - start
        assert 0.4 < elapsed < 1.0

    def test_timeout_honored_for_editors(self, mock_lib):
        """Test timeout is honored for editor count assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        start = time.time()
        with pytest.raises(AssertionError):
            numeric_assertion_with_retry(
                mock_lib.get_editor_count,
                AssertionOperator.equal,
                100,  # Won't match
                timeout=0.5,
                interval=0.1
            )
        elapsed = time.time() - start
        assert 0.4 < elapsed < 1.0


# =============================================================================
# RCP Custom Error Messages Tests
# =============================================================================


class TestRcpCustomErrorMessages:
    """Tests for custom error messages in RCP assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_perspective_error_includes_context(self, mock_lib):
        """Test perspective assertion error includes context."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError) as exc_info:
            with_retry_assertion(
                mock_lib.get_active_perspective,
                AssertionOperator.equal,
                "NonExistent",
                message="Active perspective",
                timeout=0.2,
                interval=0.1
            )
        error_msg = str(exc_info.value)
        assert "timeout" in error_msg.lower()

    def test_view_count_error_message(self, mock_lib):
        """Test view count error message."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError) as exc_info:
            numeric_assertion_with_retry(
                mock_lib.get_view_count,
                AssertionOperator.equal,
                100,
                message="View count",
                timeout=0.2,
                interval=0.1
            )
        error_msg = str(exc_info.value)
        assert "timeout" in error_msg.lower()


# =============================================================================
# RCP Edge Cases Tests
# =============================================================================


class TestRcpEdgeCases:
    """Tests for edge cases in RCP assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_no_open_editors(self, mock_lib):
        """Test handling when no editors are open."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        mock_lib.close_all_editors()

        result = numeric_assertion_with_retry(
            mock_lib.get_editor_count,
            AssertionOperator.equal,
            0,
            timeout=1.0
        )
        assert result == 0

    def test_no_dirty_editors(self, mock_lib):
        """Test handling when no editors are dirty."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        mock_lib.save_all_editors()

        result = numeric_assertion_with_retry(
            mock_lib.get_dirty_editor_count,
            AssertionOperator.equal,
            0,
            timeout=1.0
        )
        assert result == 0

    def test_active_editor_is_none(self, mock_lib):
        """Test handling when active editor is None."""
        mock_lib.close_all_editors()
        assert mock_lib.get_active_editor() is None

    def test_non_existent_view(self, mock_lib):
        """Test assertion for non-existent view."""
        with pytest.raises(AssertionError):
            mock_lib.view_should_be_visible("non.existent.View")

    def test_non_existent_editor_dirty_check(self, mock_lib):
        """Test dirty check for non-existent editor."""
        with pytest.raises(Exception):
            mock_lib.is_editor_dirty("nonexistent.file")

    def test_empty_perspective_id(self, mock_lib):
        """Test assertion with empty perspective ID."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            mock_lib.get_active_perspective,
            AssertionOperator.inequal,  # correct attribute name
            "",
            timeout=1.0
        )
        assert result != ""

    def test_special_characters_in_editor_title(self, mock_lib):
        """Test handling special characters in editor title."""
        mock_lib.open_editor("/project/src/Test<File>.java")
        editors = mock_lib.get_open_editors()
        titles = [e["title"] for e in editors]
        assert "Test<File>.java" in titles

    def test_none_operator_returns_value(self, mock_lib):
        """Test None operator returns value without assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            mock_lib.get_active_perspective,
            None,
            None,
            timeout=1.0
        )
        assert result == "org.eclipse.jdt.ui.JavaPerspective"


# =============================================================================
# RCP Workbench Info Tests
# =============================================================================


class TestRcpWorkbenchInfo:
    """Tests for RCP workbench information assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_get_workbench_info(self, mock_lib):
        """Test getting workbench info."""
        info = mock_lib.get_workbench_info()
        assert "title" in info
        assert "active_perspective" in info
        assert "view_count" in info
        assert "editor_count" in info

    def test_workbench_view_count(self, mock_lib):
        """Test workbench info view count matches get_view_count."""
        info = mock_lib.get_workbench_info()
        assert info["view_count"] == mock_lib.get_view_count()

    def test_workbench_editor_count(self, mock_lib):
        """Test workbench info editor count matches get_editor_count."""
        info = mock_lib.get_workbench_info()
        assert info["editor_count"] == mock_lib.get_editor_count()


# =============================================================================
# RCP Command Tests
# =============================================================================


class TestRcpCommands:
    """Tests for RCP command-related functionality."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock RCP library."""
        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")
        return lib

    def test_get_available_commands(self, mock_lib):
        """Test getting available commands."""
        commands = mock_lib.get_available_commands()
        assert len(commands) > 0
        assert "org.eclipse.ui.file.save" in commands

    def test_execute_command(self, mock_lib):
        """Test executing a command (does not raise)."""
        mock_lib.execute_command("org.eclipse.ui.file.saveAll")


# =============================================================================
# Integration Tests
# =============================================================================


class TestRcpAssertionIntegration:
    """Integration tests for RCP assertion functionality."""

    def test_assertion_module_imports(self):
        """Test assertion module can be imported."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.assertions import (
            with_retry_assertion,
            numeric_assertion_with_retry,
            AssertionConfig,
        )
        assert with_retry_assertion is not None
        assert numeric_assertion_with_retry is not None
        assert AssertionConfig is not None

    def test_combined_view_and_editor_assertions(self):
        """Test combined view and editor count assertions."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")

        # Assert both views and editors
        view_count = numeric_assertion_with_retry(
            lib.get_view_count,
            AssertionOperator[">="],
            1,
            timeout=1.0
        )

        editor_count = numeric_assertion_with_retry(
            lib.get_editor_count,
            AssertionOperator[">="],
            1,
            timeout=1.0
        )

        assert view_count >= 1
        assert editor_count >= 1

    def test_combined_perspective_and_dirty_assertions(self):
        """Test combined perspective and dirty state assertions."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockRcpLibrary()
        lib.connect_to_application("TestApp")

        # Assert perspective
        perspective = with_retry_assertion(
            lib.get_active_perspective,
            AssertionOperator.contains,
            "Java",
            timeout=1.0
        )

        # Assert dirty count
        dirty_count = numeric_assertion_with_retry(
            lib.get_dirty_editor_count,
            AssertionOperator[">="],
            0,
            timeout=1.0
        )

        assert "Java" in perspective
        assert dirty_count >= 0
