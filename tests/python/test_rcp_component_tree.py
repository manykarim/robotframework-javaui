"""
Tests for RCP Component Tree functionality (Phase 6).

Tests the RCP-specific tree traversal that exposes:
- Workbench Windows
- Perspectives
- Views (ViewParts)
- Editors (EditorParts)
- Underlying SWT widgets

Verifies that RCP components inherit all SWT operations.

NOTE: These tests require a real Eclipse RCP application and are skipped in CI.
"""

import json
import pytest
from JavaGui import SwingLibrary

pytestmark = pytest.mark.skip(reason="RCP tests require a real Eclipse RCP application - not available in CI")


@pytest.fixture
def rcp_library():
    """Create a SwingLibrary instance configured for RCP testing."""
    lib = SwingLibrary()
    # For RCP, we might need different connection parameters
    # The agent should auto-detect RCP vs plain SWT
    return lib


class TestRcpComponentTree:
    """Test RCP component tree retrieval and structure."""

    def test_get_rcp_component_tree_available(self, rcp_library):
        """Test that RCP component tree can be retrieved."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=5, format="json")
        tree = json.loads(tree_json)

        assert "type" in tree
        assert tree["type"] == "RcpWorkbench"
        assert "available" in tree

        # If RCP is not available, that's okay - just verify the response structure
        if not tree.get("available", False):
            pytest.skip("RCP not available in test environment")

    def test_get_rcp_component_tree_structure(self, rcp_library):
        """Test RCP component tree has expected structure."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=3, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False):
            pytest.skip("RCP not available")

        # Verify workbench structure
        assert "windows" in tree
        assert isinstance(tree["windows"], list)

        if len(tree["windows"]) > 0:
            window = tree["windows"][0]
            assert "type" in window
            assert window["type"] == "WorkbenchWindow"

            # Verify window has SWT shell reference
            assert "swtShellId" in window
            assert isinstance(window["swtShellId"], int)

    def test_rcp_workbench_window_structure(self, rcp_library):
        """Test workbench window has pages and SWT shell."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=3, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False) or len(tree.get("windows", [])) == 0:
            pytest.skip("No RCP windows available")

        window = tree["windows"][0]

        # Verify pages
        assert "pages" in window
        assert isinstance(window["pages"], list)
        assert "pageCount" in window

        # Verify SWT shell access
        assert "swtShellId" in window
        assert "swtClass" in window
        assert window["swtClass"] == "org.eclipse.swt.widgets.Shell"

    def test_rcp_page_structure(self, rcp_library):
        """Test page has perspective, views, and editors."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=3, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False) or len(tree.get("windows", [])) == 0:
            pytest.skip("No RCP windows available")

        window = tree["windows"][0]
        if len(window.get("pages", [])) == 0:
            pytest.skip("No pages in window")

        page = window["pages"][0]

        assert "type" in page
        assert page["type"] == "WorkbenchPage"

        # Verify perspective
        if "perspective" in page:
            persp = page["perspective"]
            assert "type" in persp
            assert persp["type"] == "Perspective"
            assert "id" in persp
            assert "label" in persp

        # Verify views array
        assert "views" in page
        assert isinstance(page["views"], list)
        assert "viewCount" in page

        # Verify editors array
        assert "editors" in page
        assert isinstance(page["editors"], list)
        assert "editorCount" in page

    def test_rcp_view_properties(self, rcp_library):
        """Test view has ID, name, title, and SWT widget access."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=2, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False):
            pytest.skip("RCP not available")

        # Find a view in the tree
        view = None
        for window in tree.get("windows", []):
            for page in window.get("pages", []):
                if len(page.get("views", [])) > 0:
                    view = page["views"][0]
                    break
            if view:
                break

        if not view:
            pytest.skip("No views found in RCP tree")

        # Verify view properties
        assert "type" in view
        assert view["type"] == "ViewPart"
        assert "id" in view
        assert "name" in view
        assert "title" in view

        # Secondary ID is optional
        # fastView is a boolean

        # SWT control ID should be available if part is created
        if view.get("partCreated", False):
            # May have swtControlId or swtShellId
            assert "swtControlId" in view or "swtShellId" in view

    def test_rcp_editor_properties(self, rcp_library):
        """Test editor has ID, name, title, dirty flag, and SWT widget access."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=2, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False):
            pytest.skip("RCP not available")

        # Find an editor in the tree
        editor = None
        for window in tree.get("windows", []):
            for page in window.get("pages", []):
                if len(page.get("editors", [])) > 0:
                    editor = page["editors"][0]
                    break
            if editor:
                break

        if not editor:
            pytest.skip("No editors found in RCP tree")

        # Verify editor properties
        assert "type" in editor
        assert editor["type"] == "EditorPart"
        assert "id" in editor
        assert "name" in editor
        assert "title" in editor
        assert "dirty" in editor
        assert isinstance(editor["dirty"], bool)

        # File path is optional (depends on editor type)
        # Tooltip is optional

    def test_rcp_swt_widget_inheritance(self, rcp_library):
        """Test that RCP components expose underlying SWT widgets."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=3, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False) or len(tree.get("windows", [])) == 0:
            pytest.skip("No RCP windows available")

        window = tree["windows"][0]

        # Window should have SWT shell
        assert "swtShellId" in window

        # If maxDepth > 0, should include SWT widget tree
        if "swtWidgetTree" in window:
            swt_tree = window["swtWidgetTree"]
            assert "id" in swt_tree
            assert "class" in swt_tree

            # Verify it's a valid SWT widget node
            # Should have children (composites)
            if "children" in swt_tree:
                assert isinstance(swt_tree["children"], list)


class TestRcpViewsAndEditors:
    """Test retrieving all RCP views and editors."""

    def test_get_all_rcp_views_basic(self, rcp_library):
        """Test retrieving all RCP views without SWT widgets."""
        views_json = rcp_library.get_all_rcp_views(include_swt_widgets=False)
        views = json.loads(views_json)

        assert isinstance(views, list)

        # If no views, that's okay - just verify structure
        if len(views) > 0:
            view = views[0]
            assert "type" in view
            assert view["type"] == "ViewPart"
            assert "id" in view

    def test_get_all_rcp_views_with_swt_widgets(self, rcp_library):
        """Test retrieving all RCP views with SWT widget information."""
        views_json = rcp_library.get_all_rcp_views(include_swt_widgets=True)
        views = json.loads(views_json)

        assert isinstance(views, list)

        # If there are views, check if SWT widgets are included
        if len(views) > 0:
            view = views[0]

            # If partCreated, should have SWT widget info
            if view.get("partCreated", False):
                # May have swtWidgetTree or swtControlId
                has_swt = ("swtWidgetTree" in view or
                          "swtControlId" in view or
                          "swtShellId" in view)
                assert has_swt, "View should have SWT widget information when partCreated=true"

    def test_get_all_rcp_editors_basic(self, rcp_library):
        """Test retrieving all RCP editors without SWT widgets."""
        editors_json = rcp_library.get_all_rcp_editors(include_swt_widgets=False)
        editors = json.loads(editors_json)

        assert isinstance(editors, list)

        # If no editors, that's okay - just verify structure
        if len(editors) > 0:
            editor = editors[0]
            assert "type" in editor
            assert editor["type"] == "EditorPart"
            assert "id" in editor
            assert "dirty" in editor

    def test_get_all_rcp_editors_with_swt_widgets(self, rcp_library):
        """Test retrieving all RCP editors with SWT widget information."""
        editors_json = rcp_library.get_all_rcp_editors(include_swt_widgets=True)
        editors = json.loads(editors_json)

        assert isinstance(editors, list)

        # If there are editors, check if SWT widgets are included
        if len(editors) > 0:
            editor = editors[0]

            # If partCreated, should have SWT widget info
            if editor.get("partCreated", False):
                # May have swtWidgetTree or swtControlId
                has_swt = ("swtWidgetTree" in editor or
                          "swtControlId" in editor or
                          "swtShellId" in editor)
                assert has_swt, "Editor should have SWT widget information when partCreated=true"


class TestRcpOutputFormats:
    """Test RCP component tree output formats."""

    def test_rcp_tree_json_format(self, rcp_library):
        """Test JSON output format for RCP tree."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=2, format="json")

        # Should be valid JSON
        tree = json.loads(tree_json)
        assert isinstance(tree, dict)

    def test_rcp_tree_text_format(self, rcp_library):
        """Test text output format for RCP tree."""
        tree_text = rcp_library.get_rcp_component_tree(max_depth=2, format="text")

        # Should be a string
        assert isinstance(tree_text, str)

        # Should have some content (even if RCP not available)
        assert len(tree_text) > 0

    def test_rcp_tree_yaml_format(self, rcp_library):
        """Test YAML output format for RCP tree."""
        tree_yaml = rcp_library.get_rcp_component_tree(max_depth=2, format="yaml")

        # Should be a string
        assert isinstance(tree_yaml, str)
        assert len(tree_yaml) > 0

    def test_rcp_tree_invalid_format(self, rcp_library):
        """Test that invalid format raises error."""
        with pytest.raises(Exception) as exc_info:
            rcp_library.get_rcp_component_tree(max_depth=2, format="invalid")

        assert "format" in str(exc_info.value).lower()


class TestRcpDepthControl:
    """Test depth control for RCP component trees."""

    def test_rcp_tree_depth_0(self, rcp_library):
        """Test depth=0 includes no SWT widget trees."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=0, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False):
            pytest.skip("RCP not available")

        # Should have windows but no SWT widget trees
        if len(tree.get("windows", [])) > 0:
            window = tree["windows"][0]
            assert "swtShellId" in window  # ID should always be there
            assert "swtWidgetTree" not in window  # But not the full tree

    def test_rcp_tree_depth_1(self, rcp_library):
        """Test depth=1 includes one level of SWT widgets."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=1, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False):
            pytest.skip("RCP not available")

        # Should have SWT widget trees with depth 1
        if len(tree.get("windows", [])) > 0:
            window = tree["windows"][0]

            if "swtWidgetTree" in window:
                swt_tree = window["swtWidgetTree"]
                assert "id" in swt_tree

                # Children should exist but be limited
                if "children" in swt_tree:
                    # Depth 1 means we see immediate children
                    assert isinstance(swt_tree["children"], list)

    def test_rcp_tree_depth_5(self, rcp_library):
        """Test depth=5 includes deeper SWT widget hierarchies."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=5, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False):
            pytest.skip("RCP not available")

        # Should have deeper SWT widget trees
        assert isinstance(tree, dict)
        # Exact depth verification would require recursive tree walking


class TestRcpSwtOperations:
    """Test that SWT operations work on RCP widgets."""

    def test_rcp_widget_can_perform_swt_operations(self, rcp_library):
        """
        Test that SWT widget IDs from RCP components can be used
        with standard SWT operations.
        """
        tree_json = rcp_library.get_rcp_component_tree(max_depth=2, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False) or len(tree.get("windows", [])) == 0:
            pytest.skip("No RCP windows available")

        window = tree["windows"][0]
        swt_shell_id = window.get("swtShellId")

        if swt_shell_id is None:
            pytest.skip("No SWT shell ID available")

        # The widget ID from RCP should be usable with getWidgetProperties
        # This proves RCP inherits SWT operations
        props_json = rcp_library.call_method("getWidgetProperties", {"widgetId": swt_shell_id})
        props = json.loads(props_json)

        assert "id" in props
        assert props["id"] == swt_shell_id
        assert "class" in props

    def test_rcp_view_swt_widget_operations(self, rcp_library):
        """Test that view SWT widgets support standard operations."""
        views_json = rcp_library.get_all_rcp_views(include_swt_widgets=True)
        views = json.loads(views_json)

        # Find a view with SWT control
        view_with_widget = None
        for view in views:
            if "swtControlId" in view:
                view_with_widget = view
                break

        if not view_with_widget:
            pytest.skip("No views with SWT control ID found")

        swt_control_id = view_with_widget["swtControlId"]

        # Should be able to get properties of the SWT control
        props_json = rcp_library.call_method("getWidgetProperties", {"widgetId": swt_control_id})
        props = json.loads(props_json)

        assert "id" in props
        assert props["id"] == swt_control_id


class TestRcpPluginMetadata:
    """Test RCP plugin metadata extraction."""

    def test_view_plugin_information(self, rcp_library):
        """Test that view plugin information is captured when available."""
        views_json = rcp_library.get_all_rcp_views(include_swt_widgets=False)
        views = json.loads(views_json)

        # Plugin ID may or may not be available depending on Eclipse version
        # Just verify the structure handles it gracefully
        for view in views:
            assert "id" in view  # View ID should always be there
            # pluginId is optional

    def test_perspective_plugin_information(self, rcp_library):
        """Test that perspective information includes plugin context."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=1, format="json")
        tree = json.loads(tree_json)

        if not tree.get("available", False):
            pytest.skip("RCP not available")

        # Find a perspective
        for window in tree.get("windows", []):
            for page in window.get("pages", []):
                if "perspective" in page:
                    persp = page["perspective"]
                    assert "id" in persp
                    assert "label" in persp
                    # Perspective ID often includes plugin namespace
                    # e.g., "org.eclipse.ui.resourcePerspective"
                    break


class TestRcpErrorHandling:
    """Test error handling for RCP operations."""

    def test_rcp_not_available_graceful_failure(self, rcp_library):
        """Test graceful handling when RCP is not available."""
        tree_json = rcp_library.get_rcp_component_tree(max_depth=2, format="json")
        tree = json.loads(tree_json)

        # Should have available flag
        assert "available" in tree

        # If not available, should have appropriate structure
        if not tree["available"]:
            assert "error" in tree or "windows" in tree

    def test_rcp_component_path_not_implemented(self, rcp_library):
        """Test that component path navigation handles unimplemented features."""
        # This feature is marked as "not yet implemented" in RcpComponentInspector
        result_json = rcp_library.get_rcp_component("window[0]/page[0]/view[test]", max_depth=2)
        result = json.loads(result_json)

        # Should return an error indicating not implemented
        assert "error" in result


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
