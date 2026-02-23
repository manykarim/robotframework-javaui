"""Shared validation utilities for JavaGui library."""

from typing import Any, Union


def validate_locator(locator: Union[str, Any]) -> None:
    """Validate that locator is not empty or whitespace.

    Args:
        locator: Locator string or element/widget object to validate.

    Raises:
        ValueError: If locator is empty string or only whitespace.

    """
    # Skip validation for non-string types (e.g., SwingElement/SwtWidget objects)
    if not isinstance(locator, str):
        return

    # Check for empty or whitespace-only strings
    if not locator or not locator.strip():
        raise ValueError("Locator cannot be empty or whitespace")
