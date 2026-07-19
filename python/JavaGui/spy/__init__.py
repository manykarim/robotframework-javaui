"""javagui-spy — scan Java GUIs (Swing/SWT/RCP) and generate unique Robot Framework locators.

Thin surfaces over one SpyCore: an agentic CLI today (`javagui-spy`), a web GUI later.
"""
from .core import SpyCore, SpyError
from . import generator

__all__ = ["SpyCore", "SpyError", "generator"]
