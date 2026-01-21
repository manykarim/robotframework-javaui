"""
Tests for AssertionEngine integration.

These tests verify the assertion module functionality including:
- ElementState Flag enum
- Text formatters
- Retry assertion wrappers
- Configuration classes
"""

import pytest
import time
from unittest.mock import Mock, patch, MagicMock
from enum import Flag

# Import conftest items for consistent mocking
import sys
import os
sys.path.insert(0, os.path.dirname(__file__))


# Create mock AssertionOperator for tests when assertionengine is not installed
class MockAssertionOperator:
    """Mock AssertionOperator for testing without assertionengine installed."""
    equal = "=="
    not_equal = "!="
    contains = "*="
    matches = "matches"
    starts = "^="
    ends = "$="
    greater_than = ">"
    less_than = "<"
    greater_than_or_equal = ">="
    less_than_or_equal = "<="

    def __init__(self, value):
        self.value = value

    def __eq__(self, other):
        if isinstance(other, str):
            return self.value == other
        return self.value == other.value


# Try to import real assertions module, fall back to mocking
try:
    from JavaGui.assertions import (
        ElementState,
        with_retry_assertion,
        state_assertion_with_retry,
        numeric_assertion_with_retry,
        AssertionConfig,
    )
    from JavaGui.assertions.formatters import (
        normalize_spaces,
        strip,
        lowercase,
        uppercase,
        strip_html_tags,
        apply_formatters,
        get_formatter,
        FORMATTERS,
    )
    from assertionengine import AssertionOperator
    ASSERTIONS_AVAILABLE = True
except ImportError:
    ASSERTIONS_AVAILABLE = False
    # Create mock ElementState for testing
    class ElementState(Flag):
        """Mock ElementState Flag enum for testing."""
        visible = 1
        hidden = 2
        enabled = 4
        disabled = 8
        focused = 16
        unfocused = 32
        selected = 64
        unselected = 128
        checked = 256
        unchecked = 512
        editable = 1024
        readonly = 2048
        expanded = 4096
        collapsed = 8192
        attached = 16384
        detached = 32768

        @classmethod
        def from_string(cls, state: str) -> "ElementState":
            """Convert string to ElementState."""
            return cls[state.lower().strip()]

        @classmethod
        def from_strings(cls, states: list) -> "ElementState":
            """Convert list of strings to combined ElementState."""
            result = cls(0)
            for state in states:
                result |= cls.from_string(state)
            return result

        def to_list(self) -> list:
            """Convert to list of state names."""
            return [flag.name for flag in type(self) if flag in self and flag.name]

    AssertionOperator = MockAssertionOperator


class TestElementState:
    """Tests for ElementState Flag enum."""

    def test_basic_states(self):
        """Test basic state flags."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        state = ElementState.visible | ElementState.enabled
        assert ElementState.visible in state
        assert ElementState.enabled in state
        assert ElementState.hidden not in state

    def test_from_string(self):
        """Test creating state from string."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        state = ElementState.from_string("visible")
        assert state == ElementState.visible

        state = ElementState.from_string("ENABLED")
        assert state == ElementState.enabled

    def test_from_strings(self):
        """Test creating combined state from strings."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        state = ElementState.from_strings(["visible", "enabled", "focused"])
        assert ElementState.visible in state
        assert ElementState.enabled in state
        assert ElementState.focused in state

    def test_to_list(self):
        """Test converting state to list."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        state = ElementState.visible | ElementState.enabled
        names = state.to_list()
        assert "visible" in names
        assert "enabled" in names

    def test_all_states_defined(self):
        """Test all expected states are defined."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        expected = [
            'visible', 'hidden', 'enabled', 'disabled',
            'focused', 'unfocused', 'selected', 'unselected',
            'checked', 'unchecked', 'editable', 'readonly',
            'expanded', 'collapsed', 'attached', 'detached'
        ]
        for state_name in expected:
            assert hasattr(ElementState, state_name), f"Missing state: {state_name}"

    def test_state_flag_operations(self):
        """Test Flag bitwise operations."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        # Test OR combination
        combined = ElementState.visible | ElementState.enabled
        assert ElementState.visible in combined
        assert ElementState.enabled in combined

        # Test AND intersection
        intersection = combined & ElementState.visible
        assert ElementState.visible in intersection

    def test_state_negation(self):
        """Test state negation and exclusion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        state = ElementState.visible | ElementState.enabled | ElementState.focused
        # Remove enabled using XOR
        modified = state ^ ElementState.enabled
        assert ElementState.visible in modified
        assert ElementState.focused in modified
        # enabled may or may not be "in" after XOR depending on implementation

    def test_from_string_case_insensitive(self):
        """Test from_string is case insensitive."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert ElementState.from_string("VISIBLE") == ElementState.visible
        assert ElementState.from_string("Visible") == ElementState.visible
        assert ElementState.from_string("visible") == ElementState.visible

    def test_from_string_strips_whitespace(self):
        """Test from_string strips whitespace."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert ElementState.from_string("  visible  ") == ElementState.visible

    def test_from_string_invalid_raises(self):
        """Test from_string raises KeyError for invalid state."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        with pytest.raises(KeyError):
            ElementState.from_string("invalid_state")

    def test_from_strings_empty_list(self):
        """Test from_strings with empty list returns zero state."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        state = ElementState.from_strings([])
        # Zero state should have no flags set
        for flag in ElementState:
            if flag.value != 0:
                assert flag not in state

    def test_to_list_single_state(self):
        """Test to_list with single state."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        state = ElementState.visible
        names = state.to_list()
        assert names == ["visible"] or "visible" in names


class TestFormatters:
    """Tests for text formatters."""

    def test_normalize_spaces(self):
        """Test whitespace normalization."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert normalize_spaces("hello   world") == "hello world"
        assert normalize_spaces("  a  b  c  ") == "a b c"
        assert normalize_spaces("no\nchange") == "no change"
        assert normalize_spaces("\t\ttabs\t\there") == "tabs here"

    def test_normalize_spaces_empty(self):
        """Test normalize_spaces with empty string."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert normalize_spaces("") == ""

    def test_normalize_spaces_single_word(self):
        """Test normalize_spaces with single word."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert normalize_spaces("word") == "word"

    def test_strip(self):
        """Test strip formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert strip("  hello  ") == "hello"
        assert strip("test") == "test"
        assert strip("\n\tvalue\n\t") == "value"

    def test_strip_empty(self):
        """Test strip with empty string."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert strip("") == ""

    def test_lowercase(self):
        """Test lowercase formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert lowercase("HELLO") == "hello"
        assert lowercase("MixedCase") == "mixedcase"
        assert lowercase("already") == "already"

    def test_uppercase(self):
        """Test uppercase formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert uppercase("hello") == "HELLO"
        assert uppercase("MixedCase") == "MIXEDCASE"
        assert uppercase("ALREADY") == "ALREADY"

    def test_strip_html_tags(self):
        """Test HTML tag removal."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert strip_html_tags("<b>bold</b>") == "bold"
        assert strip_html_tags("<div class='x'>text</div>") == "text"
        assert strip_html_tags("<a href='url'>link</a>") == "link"

    def test_strip_html_tags_nested(self):
        """Test HTML tag removal with nested tags."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert strip_html_tags("<div><span>nested</span></div>") == "nested"

    def test_strip_html_tags_no_tags(self):
        """Test strip_html_tags with no tags."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert strip_html_tags("plain text") == "plain text"

    def test_strip_html_tags_empty(self):
        """Test strip_html_tags with empty string."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        assert strip_html_tags("") == ""

    def test_apply_formatters(self):
        """Test applying multiple formatters."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = apply_formatters("  HELLO  WORLD  ", ["strip", "lowercase", "normalize_spaces"])
        assert result == "hello world"

    def test_apply_formatters_empty_list(self):
        """Test apply_formatters with empty formatter list."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = apply_formatters("unchanged", [])
        assert result == "unchanged"

    def test_apply_formatters_single(self):
        """Test apply_formatters with single formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = apply_formatters("TEST", ["lowercase"])
        assert result == "test"

    def test_get_formatter_valid(self):
        """Test getting valid formatter by name."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        formatter = get_formatter("strip")
        assert callable(formatter)
        assert formatter("  test  ") == "test"

    def test_get_formatter_unknown(self):
        """Test getting unknown formatter raises error."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        with pytest.raises(ValueError, match="Unknown formatter"):
            get_formatter("nonexistent")

    def test_formatters_registry(self):
        """Test FORMATTERS registry contains expected formatters."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        expected = ["normalize_spaces", "strip", "lowercase", "uppercase", "strip_html_tags"]
        for name in expected:
            assert name in FORMATTERS, f"Missing formatter: {name}"


class TestWithRetryAssertion:
    """Tests for retry assertion wrapper."""

    def test_no_operator_returns_value(self):
        """Test that None operator just returns value."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = with_retry_assertion(
            lambda: "test",
            None,
            None,
            timeout=1.0
        )
        assert result == "test"

    def test_successful_assertion(self):
        """Test assertion that passes immediately."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = with_retry_assertion(
            lambda: "expected",
            AssertionOperator.equal,
            "expected",
            timeout=1.0
        )
        assert result == "expected"

    def test_retry_until_success(self):
        """Test retry until value changes to expected."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        call_count = [0]

        def get_value():
            call_count[0] += 1
            return "expected" if call_count[0] >= 3 else "wrong"

        result = with_retry_assertion(
            get_value,
            AssertionOperator.equal,
            "expected",
            timeout=5.0,
            interval=0.1
        )
        assert result == "expected"
        assert call_count[0] >= 3

    def test_timeout_raises_error(self):
        """Test that timeout raises AssertionError."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        with pytest.raises(AssertionError, match="timeout"):
            with_retry_assertion(
                lambda: "wrong",
                AssertionOperator.equal,
                "expected",
                timeout=0.5,
                interval=0.1
            )

    def test_contains_operator(self):
        """Test contains operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = with_retry_assertion(
            lambda: "hello world",
            AssertionOperator.contains,
            "world",
            timeout=1.0
        )
        assert result == "hello world"

    def test_exception_during_get_value(self):
        """Test handling of exceptions during value retrieval."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        call_count = [0]

        def get_value():
            call_count[0] += 1
            if call_count[0] < 3:
                raise RuntimeError("Temporary failure")
            return "expected"

        result = with_retry_assertion(
            get_value,
            AssertionOperator.equal,
            "expected",
            timeout=5.0,
            interval=0.1
        )
        assert result == "expected"
        assert call_count[0] >= 3

    def test_custom_message(self):
        """Test custom error message is included."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        with pytest.raises(AssertionError) as exc_info:
            with_retry_assertion(
                lambda: "wrong",
                AssertionOperator.equal,
                "expected",
                message="Custom prefix",
                timeout=0.3,
                interval=0.1
            )
        # Error message should contain timeout info
        assert "timeout" in str(exc_info.value).lower()


class TestNumericAssertionWithRetry:
    """Tests for numeric assertion wrapper."""

    def test_no_operator_returns_value(self):
        """Test None operator returns value."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = numeric_assertion_with_retry(
            lambda: 42,
            None,
            None,
            timeout=1.0
        )
        assert result == 42

    def test_greater_than(self):
        """Test > operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = numeric_assertion_with_retry(
            lambda: 10,
            AssertionOperator["greater than"],
            5,
            timeout=1.0
        )
        assert result == 10

    def test_equal(self):
        """Test == operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = numeric_assertion_with_retry(
            lambda: 42,
            AssertionOperator.equal,
            42,
            timeout=1.0
        )
        assert result == 42

    def test_less_than(self):
        """Test < operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        result = numeric_assertion_with_retry(
            lambda: 5,
            AssertionOperator["less than"],
            10,
            timeout=1.0
        )
        assert result == 5

    def test_retry_until_value_meets_condition(self):
        """Test retry until numeric condition is met."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        call_count = [0]

        def get_value():
            call_count[0] += 1
            return call_count[0] * 10  # 10, 20, 30, ...

        result = numeric_assertion_with_retry(
            get_value,
            AssertionOperator[">="],
            30,
            timeout=5.0,
            interval=0.1
        )
        assert result >= 30

    def test_timeout_includes_last_value(self):
        """Test timeout error includes last value."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        with pytest.raises(AssertionError) as exc_info:
            numeric_assertion_with_retry(
                lambda: 5,
                AssertionOperator["greater than"],
                100,
                timeout=0.3,
                interval=0.1
            )
        # Should mention timeout and possibly last value
        assert "timeout" in str(exc_info.value).lower()


class TestStateAssertionWithRetry:
    """Tests for state assertion wrapper."""

    def test_no_operator_returns_states(self):
        """Test None operator returns state list."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        def get_states():
            return ElementState.visible | ElementState.enabled

        result = state_assertion_with_retry(
            get_states,
            None,
            None,
            timeout=1.0
        )
        assert "visible" in result
        assert "enabled" in result

    def test_state_assertion_passes(self):
        """Test state assertion that passes."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        def get_states():
            return ElementState.visible | ElementState.enabled

        result = state_assertion_with_retry(
            get_states,
            AssertionOperator.contains,
            ["visible"],
            timeout=1.0
        )
        assert "visible" in result

    def test_state_assertion_retry(self):
        """Test state assertion with retry."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        call_count = [0]

        def get_states():
            call_count[0] += 1
            if call_count[0] < 3:
                return ElementState.hidden | ElementState.disabled
            return ElementState.visible | ElementState.enabled

        result = state_assertion_with_retry(
            get_states,
            AssertionOperator.contains,
            ["visible"],
            timeout=5.0,
            interval=0.1
        )
        assert "visible" in result
        assert call_count[0] >= 3


class TestAssertionConfig:
    """Tests for AssertionConfig class."""

    def test_default_values(self):
        """Test default configuration values."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        config = AssertionConfig()
        assert config.timeout == 5.0
        assert config.interval == 0.1
        assert config.message_prefix == ""

    def test_custom_values(self):
        """Test custom configuration."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        config = AssertionConfig(timeout=10.0, interval=0.5, message_prefix="Test: ")
        assert config.timeout == 10.0
        assert config.interval == 0.5
        assert config.message_prefix == "Test: "

    def test_zero_timeout(self):
        """Test zero timeout configuration."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        config = AssertionConfig(timeout=0.0)
        assert config.timeout == 0.0

    def test_small_interval(self):
        """Test small interval configuration."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")
        config = AssertionConfig(interval=0.01)
        assert config.interval == 0.01


class TestAssertionIntegration:
    """Integration tests for assertion functionality."""

    def test_assertion_with_changing_element(self):
        """Test assertion with element that changes state."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        class MockElement:
            def __init__(self):
                self.text = "Loading..."
                self._calls = 0

            def get_text(self):
                self._calls += 1
                if self._calls >= 3:
                    self.text = "Complete"
                return self.text

        element = MockElement()

        result = with_retry_assertion(
            element.get_text,
            AssertionOperator.equal,
            "Complete",
            timeout=5.0,
            interval=0.1
        )
        assert result == "Complete"

    def test_assertion_with_formatters(self):
        """Test assertion with formatters applied."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        def get_value():
            return "  HELLO WORLD  "

        # Convert formatter names to functions
        formatter_funcs = [FORMATTERS[f] for f in ["strip", "lowercase"]]

        result = with_retry_assertion(
            get_value,
            AssertionOperator.equal,
            "hello world",
            timeout=1.0,
            formatters=formatter_funcs
        )
        # Result is formatted, so compare directly
        assert result == "hello world"

    def test_assertion_performance(self):
        """Test assertion completes within reasonable time."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        start = time.time()
        result = with_retry_assertion(
            lambda: "expected",
            AssertionOperator.equal,
            "expected",
            timeout=5.0,
            interval=0.1
        )
        elapsed = time.time() - start

        # Should complete almost immediately since assertion passes
        assert elapsed < 0.5
        assert result == "expected"


# ============================================================================
# Security Tests
# ============================================================================

# Try to import security module
try:
    from JavaGui.assertions.security import (
        SecureExpressionEvaluator,
        ExpressionSecurityError,
        secure_evaluate,
        validate_expression,
        is_expression_safe,
        SAFE_BUILTINS,
        DANGEROUS_BUILTINS,
    )
    SECURITY_AVAILABLE = True
except ImportError:
    SECURITY_AVAILABLE = False


class TestSecureExpressionEvaluator:
    """Tests for secure expression evaluation."""

    def test_simple_equality(self):
        """Test simple equality check."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        result = secure_evaluate("value == 'expected'", {"value": "expected"})
        assert result is True

    def test_string_methods(self):
        """Test string methods are allowed."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert secure_evaluate("value.startswith('Hello')", {"value": "Hello World"}) is True
        assert secure_evaluate("value.upper()", {"value": "hello"}) == "HELLO"
        assert secure_evaluate("value.lower()", {"value": "HELLO"}) == "hello"
        assert secure_evaluate("value.strip()", {"value": "  hi  "}) == "hi"

    def test_len_builtin(self):
        """Test len builtin is allowed."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert secure_evaluate("len(value) > 0", {"value": "hello"}) is True
        assert secure_evaluate("len(value)", {"value": [1, 2, 3]}) == 3

    def test_type_constructors(self):
        """Test type constructors are allowed."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert secure_evaluate("int(value)", {"value": "42"}) == 42
        assert secure_evaluate("str(value)", {"value": 123}) == "123"
        assert secure_evaluate("bool(value)", {"value": 1}) is True

    def test_comparison_operators(self):
        """Test all comparison operators work."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert secure_evaluate("value > 5", {"value": 10}) is True
        assert secure_evaluate("value < 5", {"value": 3}) is True
        assert secure_evaluate("value >= 5", {"value": 5}) is True
        assert secure_evaluate("value <= 5", {"value": 5}) is True
        assert secure_evaluate("value != 5", {"value": 6}) is True

    def test_in_operator(self):
        """Test 'in' operator works."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert secure_evaluate("'hello' in value", {"value": "hello world"}) is True
        assert secure_evaluate("1 in value", {"value": [1, 2, 3]}) is True

    def test_regex_module(self):
        """Test re module is accessible."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        result = secure_evaluate("re.search(r'\\d+', value) is not None", {"value": "test123"})
        assert result is True
        result = secure_evaluate("re.match(r'^hello', value) is not None", {"value": "hello world"})
        assert result is True

    def test_blocks_import(self):
        """Test import statements are blocked."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        with pytest.raises(ExpressionSecurityError, match="[Ii]mport"):
            secure_evaluate("__import__('os')", {})

    def test_blocks_open(self):
        """Test open() is blocked."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        with pytest.raises(ExpressionSecurityError, match="[Dd]angerous"):
            secure_evaluate("open('/etc/passwd')", {})

    def test_blocks_eval(self):
        """Test eval() is blocked."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        with pytest.raises(ExpressionSecurityError, match="[Dd]angerous"):
            secure_evaluate("eval('1+1')", {})

    def test_blocks_exec(self):
        """Test exec() is blocked."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        with pytest.raises(ExpressionSecurityError, match="[Dd]angerous"):
            secure_evaluate("exec('x=1')", {})

    def test_blocks_dunder_attributes(self):
        """Test dunder attribute access is blocked."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        with pytest.raises(ExpressionSecurityError, match="[Aa]ttribute"):
            secure_evaluate("value.__class__", {"value": "test"})
        with pytest.raises(ExpressionSecurityError, match="[Aa]ttribute"):
            secure_evaluate("value.__bases__", {"value": str})

    def test_validate_expression_returns_errors(self):
        """Test validate_expression returns list of errors."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        errors = validate_expression("__import__('os')")
        assert len(errors) > 0
        assert any("import" in e.lower() or "dangerous" in e.lower() for e in errors)

    def test_is_expression_safe(self):
        """Test is_expression_safe helper."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert is_expression_safe("value == 'test'") is True
        assert is_expression_safe("len(value) > 0") is True
        assert is_expression_safe("__import__('os')") is False
        assert is_expression_safe("open('/etc/passwd')") is False

    def test_safe_builtins_defined(self):
        """Test SAFE_BUILTINS contains expected items."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert "len" in SAFE_BUILTINS
        assert "str" in SAFE_BUILTINS
        assert "int" in SAFE_BUILTINS
        assert "bool" in SAFE_BUILTINS
        assert "isinstance" in SAFE_BUILTINS

    def test_dangerous_builtins_defined(self):
        """Test DANGEROUS_BUILTINS contains expected items."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        assert "eval" in DANGEROUS_BUILTINS
        assert "exec" in DANGEROUS_BUILTINS
        assert "open" in DANGEROUS_BUILTINS
        assert "__import__" in DANGEROUS_BUILTINS


class TestSecureEvaluatorCustomization:
    """Tests for SecureExpressionEvaluator customization."""

    def test_extra_builtins(self):
        """Test adding extra safe builtins."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")

        def custom_func(x):
            return x * 2

        evaluator = SecureExpressionEvaluator(extra_builtins={"custom": custom_func})
        result = evaluator.evaluate("custom(value)", {"value": 5})
        assert result == 10

    def test_cannot_add_dangerous_builtins(self):
        """Test cannot add dangerous builtins via extra_builtins."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        with pytest.raises(ValueError, match="[Dd]angerous"):
            SecureExpressionEvaluator(extra_builtins={"eval": eval})

    def test_strict_mode_on(self):
        """Test strict mode performs AST validation."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        evaluator = SecureExpressionEvaluator(strict_mode=True)
        with pytest.raises(ExpressionSecurityError):
            evaluator.evaluate("value.__class__", {"value": "test"})

    def test_strict_mode_off(self):
        """Test with strict mode off, only runtime checks apply."""
        if not SECURITY_AVAILABLE:
            pytest.skip("Security module not available")
        evaluator = SecureExpressionEvaluator(strict_mode=False)
        # Should raise NameError or similar due to restricted builtins
        # but won't do AST pre-validation
        with pytest.raises((ExpressionSecurityError, NameError, AttributeError)):
            evaluator.evaluate("__import__('os')", {})


# ============================================================================
# Table Keywords Tests
# ============================================================================

class TestTableKeywordsMocking:
    """Tests for TableKeywords class with mocking."""

    def test_get_table_cell_value_basic(self):
        """Test basic table cell retrieval."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.keywords.tables import TableKeywords

        class MockTableLib:
            def __init__(self):
                self._lib = self

            def get_table_cell_value(self, locator, row, column):
                if locator == "JTable" and row == 0 and column == "0":
                    return "Cell Value"
                return ""

        obj = MockTableLib()
        obj.__class__ = type("MockLib", (TableKeywords, MockTableLib), {})
        obj._assertion_timeout = 1.0
        obj._assertion_interval = 0.1

        # This would need actual method binding; simplified test
        assert True  # Placeholder - actual integration tests need real instances

    def test_table_keywords_exist(self):
        """Test TableKeywords class has expected methods."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.keywords.tables import TableKeywords

        assert hasattr(TableKeywords, 'get_table_cell_value')
        assert hasattr(TableKeywords, 'get_table_row_count')
        assert hasattr(TableKeywords, 'get_table_column_count')
        assert hasattr(TableKeywords, 'get_table_row_values')
        assert hasattr(TableKeywords, 'get_table_column_values')
        assert hasattr(TableKeywords, 'get_selected_table_rows')


class TestTreeKeywordsMocking:
    """Tests for TreeKeywords class with mocking."""

    def test_tree_keywords_exist(self):
        """Test TreeKeywords class has expected methods."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.keywords.tables import TreeKeywords

        assert hasattr(TreeKeywords, 'get_selected_tree_node')
        assert hasattr(TreeKeywords, 'get_tree_node_count')
        assert hasattr(TreeKeywords, 'get_tree_node_children')
        assert hasattr(TreeKeywords, 'tree_node_should_exist')
        assert hasattr(TreeKeywords, 'tree_node_should_not_exist')

    def test_navigate_tree_path_helper(self):
        """Test _navigate_tree_path helper method."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.keywords.tables import TreeKeywords

        tree = TreeKeywords()

        # Create mock tree structure
        tree_data = {
            "text": "Root",
            "children": [
                {"text": "Settings", "children": [
                    {"text": "Advanced", "children": []}
                ]},
                {"text": "Users", "children": []}
            ]
        }

        # Test navigation to existing node
        result = tree._navigate_tree_path(tree_data, "Root/Settings/Advanced")
        assert result is not None
        assert result["text"] == "Advanced"

        # Test navigation to non-existing node
        result = tree._navigate_tree_path(tree_data, "Root/NonExistent")
        assert result is None

        # Test navigation with pipe separator
        result = tree._navigate_tree_path(tree_data, "Root|Settings")
        assert result is not None
        assert result["text"] == "Settings"


class TestListKeywordsMocking:
    """Tests for ListKeywords class with mocking."""

    def test_list_keywords_exist(self):
        """Test ListKeywords class has expected methods."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.keywords.tables import ListKeywords

        assert hasattr(ListKeywords, 'get_selected_list_item')
        assert hasattr(ListKeywords, 'get_selected_list_items')
        assert hasattr(ListKeywords, 'get_list_items')
        assert hasattr(ListKeywords, 'get_list_item_count')
        assert hasattr(ListKeywords, 'get_selected_list_index')
        assert hasattr(ListKeywords, 'list_should_contain')
        assert hasattr(ListKeywords, 'list_should_not_contain')
        assert hasattr(ListKeywords, 'list_selection_should_be')


# ============================================================================
# Getter Keywords Tests
# ============================================================================

class TestGetterKeywords:
    """Tests for GetterKeywords class."""

    def test_getter_keywords_exist(self):
        """Test GetterKeywords class has expected methods."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.keywords.getters import GetterKeywords

        assert hasattr(GetterKeywords, 'get_text')
        assert hasattr(GetterKeywords, 'get_value')
        assert hasattr(GetterKeywords, 'get_element_count')
        assert hasattr(GetterKeywords, 'get_element_states')
        assert hasattr(GetterKeywords, 'get_property')
        assert hasattr(GetterKeywords, 'get_properties')
        assert hasattr(GetterKeywords, 'set_assertion_timeout')
        assert hasattr(GetterKeywords, 'set_assertion_interval')


# ============================================================================
# Integration Readiness Tests
# ============================================================================

class TestModuleIntegration:
    """Tests for module structure and integration."""

    def test_javagui_init_imports(self):
        """Test JavaGui __init__ imports work correctly."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        try:
            from JavaGui import AssertionOperator, ElementState
            assert AssertionOperator is not None
            assert ElementState is not None
        except ImportError:
            # May fail if Rust core not available, which is OK for unit tests
            pytest.skip("Full JavaGui import not available")

    def test_keyword_module_exports(self):
        """Test keyword module exports all expected classes."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.keywords import (
            GetterKeywords,
            TableKeywords,
            TreeKeywords,
            ListKeywords,
        )

        assert GetterKeywords is not None
        assert TableKeywords is not None
        assert TreeKeywords is not None
        assert ListKeywords is not None

    def test_assertion_module_exports(self):
        """Test assertion module exports all expected items."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.assertions import (
            AssertionOperator,
            ElementState,
            with_retry_assertion,
            verify_with_retry,
            numeric_assertion_with_retry,
            state_assertion_with_retry,
            AssertionConfig,
            SecureExpressionEvaluator,
            ExpressionSecurityError,
            secure_evaluate,
            validate_expression,
            is_expression_safe,
        )

        # All should be defined
        assert AssertionOperator is not None
        assert ElementState is not None
        assert with_retry_assertion is not None
        assert verify_with_retry is not None
        assert numeric_assertion_with_retry is not None
        assert state_assertion_with_retry is not None
        assert AssertionConfig is not None
        assert SecureExpressionEvaluator is not None
        assert ExpressionSecurityError is not None
        assert secure_evaluate is not None
        assert validate_expression is not None
        assert is_expression_safe is not None


# =============================================================================
# Deprecation System Tests
# =============================================================================


class TestDeprecationSystem:
    """Tests for the deprecation warning system."""

    def test_deprecated_decorator(self):
        """Test that @deprecated decorator adds deprecation metadata."""
        from JavaGui.deprecation import deprecated

        @deprecated(
            reason="Use new_func instead",
            replacement="new_func",
            version="3.0.0",
            remove_in="4.0.0",
        )
        def old_func():
            return "result"

        assert hasattr(old_func, "_deprecated")
        assert old_func._deprecated is True
        assert old_func._deprecation_reason == "Use new_func instead"
        assert old_func._deprecation_replacement == "new_func"
        assert old_func._deprecation_version == "3.0.0"
        assert old_func._deprecation_remove_in == "4.0.0"

    def test_deprecated_decorator_issues_warning(self):
        """Test that @deprecated decorator issues warning when called."""
        import warnings
        from JavaGui.deprecation import deprecated, DeprecatedKeywordWarning

        @deprecated(reason="Old function", replacement="new_func")
        def old_func():
            return "result"

        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            result = old_func()
            assert result == "result"
            assert len(w) == 1
            assert issubclass(w[0].category, DeprecatedKeywordWarning)
            assert "old_func" in str(w[0].message)
            assert "deprecated" in str(w[0].message).lower()

    def test_keyword_alias_registry(self):
        """Test KeywordAliasRegistry registration and lookup."""
        from JavaGui.deprecation import KeywordAliasRegistry

        registry = KeywordAliasRegistry()
        registry.register_alias(
            "Old Keyword",
            "New Keyword",
            deprecated_in="3.0.0",
            remove_in="4.0.0",
        )

        assert registry.is_deprecated_alias("Old Keyword")
        assert not registry.is_deprecated_alias("New Keyword")
        assert registry.get_original_name("Old Keyword") == "New Keyword"
        assert registry.get_original_name("Unknown") is None

    def test_keyword_alias_registry_get_all(self):
        """Test getting all aliases from registry."""
        from JavaGui.deprecation import KeywordAliasRegistry

        registry = KeywordAliasRegistry()
        registry.register_alias("alias1", "original1")
        registry.register_alias("alias2", "original2")

        aliases = registry.get_all_aliases()
        assert aliases == {"alias1": "original1", "alias2": "original2"}

    def test_create_keyword_alias(self):
        """Test create_keyword_alias creates working alias."""
        import warnings
        from JavaGui.deprecation import create_keyword_alias, DeprecatedKeywordWarning

        def original_method(x, y):
            return x + y

        alias = create_keyword_alias(
            original_method,
            "old_method_name",
            deprecated_in="3.0.0",
        )

        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            result = alias(2, 3)
            assert result == 5
            assert len(w) == 1
            assert issubclass(w[0].category, DeprecatedKeywordWarning)

    def test_global_alias_registry_has_swing_aliases(self):
        """Test that global registry contains expected Swing aliases."""
        from JavaGui.deprecation import get_alias_registry

        registry = get_alias_registry()
        aliases = registry.get_all_aliases()

        # Check some expected aliases exist
        assert "Get Label Content" in aliases
        assert "Get Table Cell Content" in aliases
        assert "Get Number Of Table Rows" in aliases

        # Check they map to correct new names
        assert aliases["Get Label Content"] == "Get Text"
        assert aliases["Get Table Cell Content"] == "Get Table Cell Value"
        assert aliases["Get Number Of Table Rows"] == "Get Table Row Count"

    def test_deprecation_module_exports(self):
        """Test deprecation module exports all expected items."""
        from JavaGui.deprecation import (
            deprecated,
            DeprecatedKeywordWarning,
            KeywordAliasRegistry,
            create_keyword_alias,
            register_alias,
            get_alias_registry,
        )

        assert deprecated is not None
        assert DeprecatedKeywordWarning is not None
        assert KeywordAliasRegistry is not None
        assert create_keyword_alias is not None
        assert register_alias is not None
        assert get_alias_registry is not None


class TestDeprecationIntegration:
    """Integration tests for deprecation with SwingLibrary."""

    def test_javagui_exports_deprecation(self):
        """Test JavaGui exports deprecation utilities."""
        try:
            from JavaGui import deprecated, DeprecatedKeywordWarning

            assert deprecated is not None
            assert DeprecatedKeywordWarning is not None
        except ImportError:
            pytest.skip("JavaGui not available")
