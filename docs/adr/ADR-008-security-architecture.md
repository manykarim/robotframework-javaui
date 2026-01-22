# ADR-008: Security Architecture

| ADR ID | ADR-008 |
|--------|---------|
| Title | Security Architecture for Keyword API Modernization |
| Status | Proposed |
| Date | 2026-01-19 |
| Authors | Security Architecture Team |

## Context

The robotframework-swing library connects to Java applications via JSON-RPC over TCP, executes UI automation commands, and handles potentially sensitive application data. As part of the keyword API modernization effort, a comprehensive security architecture is required to address identified vulnerabilities and establish secure-by-default patterns.

### Current Architecture Security Surface

```
+-------------------+        JSON-RPC/TCP        +------------------+
|  Python Library   |  <--------------------->   |   Java Agent     |
|  (Robot Framework)|                            |  (Target JVM)    |
+-------------------+                            +------------------+
        |                                                |
        v                                                v
   Locator Parsing                                 UI Component
   (Rust Core)                                     Access
        |                                                |
        v                                                v
   RPC Command                                    Sensitive Data
   Construction                                   (UI Tree, Screenshots)
```

### Identified Security Concerns

| Category | Risk Level | Description |
|----------|------------|-------------|
| RPC Command Injection | HIGH | Malicious locators could inject RPC commands |
| Locator Injection | HIGH | Specially crafted locators could exploit parsing |
| Session Security | MEDIUM | Session tokens/connections could be hijacked |
| Data Exposure | MEDIUM | UI tree dumps may contain sensitive data |
| Screenshot PII | MEDIUM | Screenshots may capture personal information |
| Resource Exhaustion | LOW | Unbounded operations could exhaust resources |

### Decision Drivers

- Protect against injection attacks through locators and assertions
- Prevent unauthorized access to target applications
- Minimize exposure of sensitive data in logs and screenshots
- Maintain backwards compatibility with existing tests
- Keep security controls transparent to test authors
- Enable security auditing for compliance requirements

## Decision

We will implement a **Defense-in-Depth Security Architecture** with multiple layers of protection across the entire data flow, with secure defaults that require no additional configuration for most use cases.

### 1. Threat Model

#### 1.1 Trust Boundaries

```
                    UNTRUSTED                    TRUSTED
                  +-----------+               +-----------+
                  |           |               |           |
  User Input ---->|  Locator  |--[Validated]->|   RPC     |----> Java Agent
  (Test Files)    |  Validator|               |  Client   |
                  |           |               |           |
                  +-----------+               +-----------+
                        |                           |
                        v                           v
                  Input from Robot               Commands to
                  Framework Keywords             Target JVM
```

#### 1.2 Attack Vectors

| Vector | Entry Point | Potential Impact | Mitigation |
|--------|-------------|------------------|------------|
| Locator Injection | `find_element()`, `click()`, etc. | RPC command manipulation | Input validation, allowlist |
| XPath Injection | XPath locators | Query manipulation | Grammar-based parsing, sanitization |
| Assertion Code Execution | Custom assertions | Arbitrary code execution | Safe evaluation, no `eval()` |
| Session Hijacking | TCP connection | Unauthorized access | Session tokens, IP binding |
| Data Exfiltration | UI tree dumps, screenshots | PII/sensitive data leak | Data sanitization, masking |
| DoS via Recursion | Deep locator chaining | Stack exhaustion | Bounded recursion limits |
| Resource Exhaustion | Large screenshots, tree dumps | Memory exhaustion | Size limits, streaming |

### 2. Security Controls

#### 2.1 Input Validation for Locators

All locator inputs must pass through a validation layer before reaching the RPC client.

```rust
/// Security-focused locator validator
pub struct SecureLocatorValidator {
    /// Maximum locator length (prevents buffer overflow attacks)
    max_length: usize,
    /// Maximum nesting depth (prevents recursion attacks)
    max_depth: usize,
    /// Allowed attribute names (prevents injection via unknown attributes)
    allowed_attributes: HashSet<String>,
    /// Blocked patterns (regex injection, path traversal, etc.)
    blocked_patterns: Vec<Regex>,
}

impl SecureLocatorValidator {
    pub const DEFAULT_MAX_LENGTH: usize = 4096;
    pub const DEFAULT_MAX_DEPTH: usize = 20;

    /// Core allowed attributes - cannot be extended without security review
    pub const CORE_ATTRIBUTES: &[&str] = &[
        "name", "text", "class", "id", "type", "visible", "enabled",
        "selected", "tooltip", "index", "row", "column", "label",
    ];

    /// Validate a locator before processing
    pub fn validate(&self, locator: &str) -> Result<ValidatedLocator, SecurityError> {
        // 1. Length check
        if locator.len() > self.max_length {
            return Err(SecurityError::LocatorTooLong {
                length: locator.len(),
                max: self.max_length,
            });
        }

        // 2. Empty/whitespace check
        let trimmed = locator.trim();
        if trimmed.is_empty() {
            return Err(SecurityError::EmptyLocator);
        }

        // 3. Blocked pattern check
        for pattern in &self.blocked_patterns {
            if pattern.is_match(locator) {
                return Err(SecurityError::BlockedPattern {
                    locator: Self::sanitize_for_log(locator),
                    pattern: pattern.to_string(),
                });
            }
        }

        // 4. Parse and validate structure
        let parsed = parse_locator(locator)?;

        // 5. Depth check
        if parsed.depth() > self.max_depth {
            return Err(SecurityError::ExcessiveDepth {
                depth: parsed.depth(),
                max: self.max_depth,
            });
        }

        // 6. Attribute validation
        for attr in parsed.attributes() {
            if !self.allowed_attributes.contains(&attr.to_lowercase()) {
                return Err(SecurityError::UnknownAttribute {
                    attribute: attr.to_string(),
                });
            }
        }

        Ok(ValidatedLocator::new(parsed, locator.to_string()))
    }

    /// Sanitize locator for safe logging (no sensitive data exposure)
    fn sanitize_for_log(locator: &str) -> String {
        // Truncate and mask potentially sensitive values
        let truncated = if locator.len() > 100 {
            format!("{}...[truncated]", &locator[..100])
        } else {
            locator.to_string()
        };
        // Mask quoted values that might contain sensitive data
        truncated.replace(|c| c == '\'' || c == '"', "*")
    }
}

/// Blocked patterns that could indicate injection attempts
pub const DEFAULT_BLOCKED_PATTERNS: &[&str] = &[
    // Path traversal
    r"\.\./",
    r"\.\.\\",
    // Shell metacharacters
    r"[;|&`$]",
    // Null bytes
    r"\x00",
    // Unicode direction override (text spoofing)
    r"[\u202A-\u202E\u2066-\u2069]",
    // CRLF injection
    r"[\r\n]",
];
```

#### 2.2 Safe Assertion Evaluation

Assertions must never execute arbitrary code. All assertion evaluation uses a safe, restricted evaluator.

```rust
/// Safe assertion evaluator - NO eval(), NO code execution
pub struct SafeAssertionEvaluator {
    /// Allowed comparison operators
    allowed_operators: HashSet<ComparisonOp>,
    /// Maximum assertion complexity
    max_terms: usize,
}

/// Supported comparison operations (whitelist approach)
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ComparisonOp {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Matches,  // Regex match with resource limits
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    IsTrue,
    IsFalse,
    IsEmpty,
    IsNotEmpty,
    IsNull,
    IsNotNull,
}

impl SafeAssertionEvaluator {
    /// Evaluate an assertion safely
    pub fn evaluate(
        &self,
        actual: &str,
        expected: &str,
        operator: ComparisonOp,
    ) -> Result<bool, AssertionError> {
        // Validate operator is allowed
        if !self.allowed_operators.contains(&operator) {
            return Err(AssertionError::OperatorNotAllowed(operator));
        }

        match operator {
            ComparisonOp::Equals => Ok(actual == expected),
            ComparisonOp::NotEquals => Ok(actual != expected),
            ComparisonOp::Contains => Ok(actual.contains(expected)),
            ComparisonOp::NotContains => Ok(!actual.contains(expected)),
            ComparisonOp::StartsWith => Ok(actual.starts_with(expected)),
            ComparisonOp::EndsWith => Ok(actual.ends_with(expected)),
            ComparisonOp::Matches => self.safe_regex_match(actual, expected),
            ComparisonOp::GreaterThan => self.safe_numeric_compare(actual, expected, |a, b| a > b),
            ComparisonOp::LessThan => self.safe_numeric_compare(actual, expected, |a, b| a < b),
            ComparisonOp::GreaterOrEqual => self.safe_numeric_compare(actual, expected, |a, b| a >= b),
            ComparisonOp::LessOrEqual => self.safe_numeric_compare(actual, expected, |a, b| a <= b),
            ComparisonOp::IsTrue => Ok(actual.to_lowercase() == "true"),
            ComparisonOp::IsFalse => Ok(actual.to_lowercase() == "false"),
            ComparisonOp::IsEmpty => Ok(actual.is_empty()),
            ComparisonOp::IsNotEmpty => Ok(!actual.is_empty()),
            ComparisonOp::IsNull => Ok(actual.is_empty()),  // Null represented as empty
            ComparisonOp::IsNotNull => Ok(!actual.is_empty()),
        }
    }

    /// Safe regex matching with resource limits
    fn safe_regex_match(&self, text: &str, pattern: &str) -> Result<bool, AssertionError> {
        // Use regex with size limits to prevent ReDoS
        let regex = regex::RegexBuilder::new(pattern)
            .size_limit(1024 * 100)  // 100KB compiled size limit
            .dfa_size_limit(1024 * 100)  // 100KB DFA limit
            .build()
            .map_err(|e| AssertionError::InvalidRegex(e.to_string()))?;

        // Use with timeout via match_indices limited iteration
        Ok(regex.is_match(text))
    }
}
```

#### 2.3 Session Token Management

Secure session management for RPC connections.

```rust
/// Secure session manager for RPC connections
pub struct SecureSessionManager {
    /// Active sessions with metadata
    sessions: HashMap<SessionId, SessionInfo>,
    /// Session token generator
    token_generator: TokenGenerator,
    /// Maximum concurrent sessions per host
    max_sessions_per_host: usize,
    /// Session timeout
    session_timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// Unique session identifier
    pub id: SessionId,
    /// Session token (cryptographically random)
    pub token: SessionToken,
    /// Client IP address (for binding)
    pub client_addr: IpAddr,
    /// Creation time
    pub created_at: Instant,
    /// Last activity time
    pub last_activity: Instant,
    /// Target application info
    pub target: TargetInfo,
}

/// Cryptographically secure token (256-bit)
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SessionToken([u8; 32]);

impl SessionToken {
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        getrandom::getrandom(&mut bytes).expect("Failed to generate random bytes");
        Self(bytes)
    }

    /// Constant-time comparison to prevent timing attacks
    pub fn verify(&self, other: &SessionToken) -> bool {
        use subtle::ConstantTimeEq;
        self.0.ct_eq(&other.0).into()
    }
}

impl SecureSessionManager {
    /// Create a new authenticated session
    pub fn create_session(
        &mut self,
        client_addr: IpAddr,
        target: TargetInfo,
    ) -> Result<SessionInfo, SessionError> {
        // Check session limits
        let host_sessions = self.sessions.values()
            .filter(|s| s.client_addr == client_addr)
            .count();

        if host_sessions >= self.max_sessions_per_host {
            return Err(SessionError::TooManySessions);
        }

        let session = SessionInfo {
            id: SessionId::generate(),
            token: SessionToken::generate(),
            client_addr,
            created_at: Instant::now(),
            last_activity: Instant::now(),
            target,
        };

        self.sessions.insert(session.id.clone(), session.clone());
        Ok(session)
    }

    /// Validate session and update activity
    pub fn validate_session(
        &mut self,
        session_id: &SessionId,
        token: &SessionToken,
        client_addr: IpAddr,
    ) -> Result<&SessionInfo, SessionError> {
        let session = self.sessions.get_mut(session_id)
            .ok_or(SessionError::SessionNotFound)?;

        // Verify token (constant-time comparison)
        if !session.token.verify(token) {
            return Err(SessionError::InvalidToken);
        }

        // Verify client IP binding
        if session.client_addr != client_addr {
            return Err(SessionError::IpMismatch);
        }

        // Check timeout
        if session.last_activity.elapsed() > self.session_timeout {
            self.sessions.remove(session_id);
            return Err(SessionError::SessionExpired);
        }

        // Update activity
        session.last_activity = Instant::now();

        Ok(self.sessions.get(session_id).unwrap())
    }
}
```

#### 2.4 Secure RPC Communication

Security controls for the JSON-RPC communication layer.

```rust
/// Security-enhanced RPC client
pub struct SecureRpcClient {
    /// Underlying connection
    connection: TcpStream,
    /// Session info
    session: SessionInfo,
    /// Request validator
    validator: RpcRequestValidator,
    /// Rate limiter
    rate_limiter: RateLimiter,
}

impl SecureRpcClient {
    /// Send a request with security controls
    pub fn send_request(
        &mut self,
        method: RpcMethod,
        params: serde_json::Value,
    ) -> SwingResult<serde_json::Value> {
        // 1. Rate limiting
        if !self.rate_limiter.allow_request() {
            return Err(SwingError::RateLimited);
        }

        // 2. Validate method is allowed
        self.validator.validate_method(&method)?;

        // 3. Sanitize parameters
        let sanitized_params = self.validator.sanitize_params(&params)?;

        // 4. Build request with session token
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method.as_str(),
            "params": sanitized_params,
            "id": self.next_request_id(),
            "session_token": hex::encode(&self.session.token.0),
        });

        // 5. Send and receive
        let response = self.send_raw(&request)?;

        // 6. Validate response integrity
        self.validator.validate_response(&response)?;

        Ok(response)
    }
}

/// Rate limiter to prevent DoS attacks
pub struct RateLimiter {
    /// Maximum requests per window
    max_requests: u32,
    /// Window size
    window: Duration,
    /// Request timestamps
    requests: VecDeque<Instant>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            requests: VecDeque::with_capacity(max_requests as usize),
        }
    }

    pub fn allow_request(&mut self) -> bool {
        let now = Instant::now();

        // Remove old requests outside window
        while let Some(front) = self.requests.front() {
            if now.duration_since(*front) > self.window {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        if self.requests.len() < self.max_requests as usize {
            self.requests.push_back(now);
            true
        } else {
            false
        }
    }
}
```

#### 2.5 Data Sanitization

Sanitization of sensitive data in logs and outputs.

```rust
/// Data sanitizer for logs, screenshots, and UI dumps
pub struct DataSanitizer {
    /// Patterns to detect sensitive data
    sensitive_patterns: Vec<SensitivePattern>,
    /// Masking strategy
    masking_strategy: MaskingStrategy,
}

#[derive(Debug, Clone)]
pub struct SensitivePattern {
    /// Pattern name for audit logging
    pub name: String,
    /// Regex pattern
    pub pattern: Regex,
    /// Whether to completely redact or mask
    pub redact_completely: bool,
}

impl DataSanitizer {
    /// Default sensitive patterns
    pub fn default_patterns() -> Vec<SensitivePattern> {
        vec![
            // Credit card numbers
            SensitivePattern {
                name: "credit_card".to_string(),
                pattern: Regex::new(r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b").unwrap(),
                redact_completely: true,
            },
            // Social Security Numbers
            SensitivePattern {
                name: "ssn".to_string(),
                pattern: Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap(),
                redact_completely: true,
            },
            // Email addresses
            SensitivePattern {
                name: "email".to_string(),
                pattern: Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap(),
                redact_completely: false,
            },
            // Passwords (common patterns in UI)
            SensitivePattern {
                name: "password_field".to_string(),
                pattern: Regex::new(r"(?i)(password|passwd|pwd)[:=]\s*\S+").unwrap(),
                redact_completely: true,
            },
            // API keys and tokens
            SensitivePattern {
                name: "api_key".to_string(),
                pattern: Regex::new(r"(?i)(api[_-]?key|token|secret)[:=]\s*[A-Za-z0-9+/=_-]{20,}").unwrap(),
                redact_completely: true,
            },
        ]
    }

    /// Sanitize a string for safe logging
    pub fn sanitize_for_log(&self, input: &str) -> String {
        let mut output = input.to_string();

        for pattern in &self.sensitive_patterns {
            if pattern.redact_completely {
                output = pattern.pattern.replace_all(&output, "[REDACTED]").to_string();
            } else {
                // Partial masking (keep first/last few chars)
                output = pattern.pattern.replace_all(&output, |caps: &regex::Captures| {
                    let matched = caps.get(0).unwrap().as_str();
                    self.partial_mask(matched)
                }).to_string();
            }
        }

        output
    }

    /// Sanitize UI tree for export
    pub fn sanitize_ui_tree(&self, tree: &str) -> String {
        let mut output = self.sanitize_for_log(tree);

        // Additional UI-specific sanitization
        // Remove text content from password fields
        let password_field_pattern = Regex::new(
            r#"(JPasswordField|PasswordField)\[.*?text=['"](.*?)['"]"#
        ).unwrap();
        output = password_field_pattern.replace_all(&output, |caps: &regex::Captures| {
            format!("{}[text='[MASKED]'", caps.get(1).unwrap().as_str())
        }).to_string();

        output
    }

    /// Partial mask - show first and last chars only
    fn partial_mask(&self, value: &str) -> String {
        let len = value.len();
        if len <= 4 {
            return "*".repeat(len);
        }

        let visible_chars = 2.min(len / 4);
        format!(
            "{}{}{}",
            &value[..visible_chars],
            "*".repeat(len - visible_chars * 2),
            &value[len - visible_chars..]
        )
    }
}

/// Screenshot security settings
pub struct ScreenshotSecurityConfig {
    /// Enable PII detection and masking
    pub mask_pii: bool,
    /// Regions to always mask (coordinates)
    pub masked_regions: Vec<MaskedRegion>,
    /// Maximum screenshot dimensions
    pub max_width: u32,
    pub max_height: u32,
    /// Strip EXIF metadata
    pub strip_metadata: bool,
}

#[derive(Debug, Clone)]
pub struct MaskedRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: String,  // For audit trail
}
```

### 3. Secure-by-Default Patterns

#### 3.1 Locator Whitelist/Validation

```rust
/// Locator validation configuration (secure defaults)
pub struct LocatorSecurityConfig {
    /// Maximum locator string length
    pub max_length: usize,
    /// Maximum selector nesting depth
    pub max_depth: usize,
    /// Maximum number of compound selectors
    pub max_compounds: usize,
    /// Allowed pseudo-selectors
    pub allowed_pseudos: HashSet<String>,
    /// Blocked characters/patterns
    pub blocked_chars: HashSet<char>,
}

impl Default for LocatorSecurityConfig {
    fn default() -> Self {
        Self {
            max_length: 4096,
            max_depth: 20,
            max_compounds: 50,
            allowed_pseudos: [
                "enabled", "disabled", "visible", "hidden", "checked",
                "selected", "focused", "first", "last", "first-child",
                "last-child", "nth-child", "nth-of-type", "contains",
                "not", "has", "empty", "root",
            ].iter().map(|s| s.to_string()).collect(),
            blocked_chars: ['`', '$', '|', ';', '\0', '\r', '\n'].into_iter().collect(),
        }
    }
}
```

#### 3.2 Safe String Interpolation

```rust
/// Safe parameter builder that prevents injection
pub struct SafeParamBuilder {
    params: HashMap<String, serde_json::Value>,
}

impl SafeParamBuilder {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Add a string parameter with automatic escaping
    pub fn add_string(&mut self, key: &str, value: &str) -> &mut Self {
        // Escape any potentially dangerous characters
        let escaped = Self::escape_string(value);
        self.params.insert(key.to_string(), serde_json::Value::String(escaped));
        self
    }

    /// Add a locator parameter with validation
    pub fn add_locator(&mut self, key: &str, locator: &ValidatedLocator) -> &mut Self {
        // Only accept pre-validated locators
        self.params.insert(
            key.to_string(),
            serde_json::Value::String(locator.to_string())
        );
        self
    }

    /// Add an integer parameter (no escaping needed)
    pub fn add_int(&mut self, key: &str, value: i64) -> &mut Self {
        self.params.insert(
            key.to_string(),
            serde_json::Value::Number(value.into())
        );
        self
    }

    /// Escape potentially dangerous characters in strings
    fn escape_string(input: &str) -> String {
        input
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\0', "")  // Remove null bytes entirely
    }

    pub fn build(self) -> serde_json::Value {
        serde_json::Value::Object(
            self.params.into_iter()
                .map(|(k, v)| (k, v))
                .collect()
        )
    }
}
```

#### 3.3 Bounded Recursion in Locator Chaining

```rust
/// Locator evaluator with bounded recursion
pub struct BoundedLocatorEvaluator {
    /// Maximum recursion depth
    max_depth: usize,
    /// Current depth (thread-local for reentrancy)
    depth_counter: std::cell::RefCell<usize>,
}

impl BoundedLocatorEvaluator {
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            depth_counter: std::cell::RefCell::new(0),
        }
    }

    /// Evaluate locator with depth tracking
    pub fn evaluate<F, T>(&self, f: F) -> Result<T, SwingError>
    where
        F: FnOnce() -> Result<T, SwingError>,
    {
        // Increment depth
        {
            let mut depth = self.depth_counter.borrow_mut();
            if *depth >= self.max_depth {
                return Err(SwingError::Internal {
                    message: format!(
                        "Maximum locator recursion depth ({}) exceeded",
                        self.max_depth
                    ),
                });
            }
            *depth += 1;
        }

        // Execute
        let result = f();

        // Decrement depth
        {
            let mut depth = self.depth_counter.borrow_mut();
            *depth = depth.saturating_sub(1);
        }

        result
    }
}
```

#### 3.4 Timeout Enforcement

```rust
/// Timeout enforcer for all blocking operations
pub struct TimeoutEnforcer {
    /// Default timeout for operations
    default_timeout: Duration,
    /// Maximum allowed timeout (prevents infinite waits)
    max_timeout: Duration,
    /// Minimum timeout (prevents busy-waiting)
    min_timeout: Duration,
}

impl TimeoutEnforcer {
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
    pub const MAX_TIMEOUT: Duration = Duration::from_secs(300);  // 5 minutes max
    pub const MIN_TIMEOUT: Duration = Duration::from_millis(100);

    /// Validate and clamp a timeout value
    pub fn validate_timeout(&self, timeout: Option<Duration>) -> Duration {
        match timeout {
            Some(t) => t.max(self.min_timeout).min(self.max_timeout),
            None => self.default_timeout,
        }
    }

    /// Execute with timeout
    pub async fn with_timeout<F, T>(
        &self,
        timeout: Option<Duration>,
        future: F,
    ) -> Result<T, SwingError>
    where
        F: std::future::Future<Output = Result<T, SwingError>>,
    {
        let timeout = self.validate_timeout(timeout);

        tokio::time::timeout(timeout, future)
            .await
            .map_err(|_| SwingError::WaitTimeout {
                condition: "operation".to_string(),
                timeout_ms: timeout.as_millis() as u64,
            })?
    }
}
```

#### 3.5 Resource Cleanup Guarantees

```rust
/// RAII guard for resource cleanup
pub struct ResourceGuard<T, F>
where
    F: FnOnce(&T),
{
    resource: T,
    cleanup: Option<F>,
}

impl<T, F> ResourceGuard<T, F>
where
    F: FnOnce(&T),
{
    pub fn new(resource: T, cleanup: F) -> Self {
        Self {
            resource,
            cleanup: Some(cleanup),
        }
    }

    pub fn get(&self) -> &T {
        &self.resource
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.resource
    }
}

impl<T, F> Drop for ResourceGuard<T, F>
where
    F: FnOnce(&T),
{
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup(&self.resource);
        }
    }
}

/// Connection cleanup guard
pub type ConnectionGuard = ResourceGuard<SwingConnection, fn(&SwingConnection)>;

impl SwingConnection {
    /// Create a connection with guaranteed cleanup
    pub fn with_cleanup(self) -> ConnectionGuard {
        ResourceGuard::new(self, |conn| {
            // Best-effort cleanup - don't panic in drop
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // conn.disconnect() would need interior mutability
                // This is just for illustration
            }));
        })
    }
}
```

### 4. Security Hooks Integration

#### 4.1 Pre-Command Validation Hooks

```rust
/// Security hook interface
pub trait SecurityHook: Send + Sync {
    /// Called before command execution
    fn pre_command(
        &self,
        context: &CommandContext,
    ) -> Result<HookDecision, SecurityError>;

    /// Called after command execution
    fn post_command(
        &self,
        context: &CommandContext,
        result: &CommandResult,
    ) -> Result<(), SecurityError>;
}

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub method: RpcMethod,
    pub params: serde_json::Value,
    pub session: SessionId,
    pub timestamp: SystemTime,
    pub source: CommandSource,
}

#[derive(Debug)]
pub enum HookDecision {
    /// Allow the command to proceed
    Allow,
    /// Block the command with reason
    Block { reason: String },
    /// Modify the command (sanitization)
    Modify { new_params: serde_json::Value },
}

/// Pre-command validation hook implementation
pub struct ValidationHook {
    validator: SecureLocatorValidator,
    sanitizer: DataSanitizer,
}

impl SecurityHook for ValidationHook {
    fn pre_command(
        &self,
        context: &CommandContext,
    ) -> Result<HookDecision, SecurityError> {
        // Validate locator parameters
        if let Some(locator) = context.params.get("locator").and_then(|v| v.as_str()) {
            self.validator.validate(locator)?;
        }

        // Sanitize text parameters
        let sanitized = self.sanitizer.sanitize_params(&context.params);

        if sanitized != context.params {
            Ok(HookDecision::Modify { new_params: sanitized })
        } else {
            Ok(HookDecision::Allow)
        }
    }

    fn post_command(
        &self,
        _context: &CommandContext,
        _result: &CommandResult,
    ) -> Result<(), SecurityError> {
        Ok(())
    }
}
```

#### 4.2 Post-Command Audit Logging

```rust
/// Audit logger for security events
pub struct AuditLogger {
    /// Output destination
    output: Box<dyn AuditOutput>,
    /// Minimum log level
    min_level: AuditLevel,
    /// Include sensitive data (should be false in production)
    include_sensitive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditLevel {
    Debug,
    Info,
    Warning,
    Security,
    Critical,
}

#[derive(Debug, Serialize)]
pub struct AuditEvent {
    pub timestamp: String,
    pub level: String,
    pub event_type: String,
    pub session_id: String,
    pub method: String,
    pub success: bool,
    pub duration_ms: u64,
    pub details: serde_json::Value,
}

impl AuditLogger {
    /// Log a command execution
    pub fn log_command(
        &self,
        context: &CommandContext,
        result: &CommandResult,
        duration: Duration,
    ) {
        let event = AuditEvent {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: if result.is_err() { "Warning" } else { "Info" }.to_string(),
            event_type: "command_execution".to_string(),
            session_id: context.session.to_string(),
            method: context.method.as_str().to_string(),
            success: result.is_ok(),
            duration_ms: duration.as_millis() as u64,
            details: self.build_details(context, result),
        };

        self.output.write(&event);
    }

    /// Log a security event
    pub fn log_security_event(
        &self,
        event_type: &str,
        description: &str,
        severity: AuditLevel,
        details: serde_json::Value,
    ) {
        if severity < self.min_level {
            return;
        }

        let event = AuditEvent {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: format!("{:?}", severity),
            event_type: event_type.to_string(),
            session_id: "system".to_string(),
            method: "security".to_string(),
            success: false,
            duration_ms: 0,
            details: serde_json::json!({
                "description": description,
                "context": details,
            }),
        };

        self.output.write(&event);
    }

    fn build_details(
        &self,
        context: &CommandContext,
        result: &CommandResult,
    ) -> serde_json::Value {
        let mut details = serde_json::json!({});

        // Include locator (sanitized) if present
        if let Some(locator) = context.params.get("locator") {
            details["locator"] = if self.include_sensitive {
                locator.clone()
            } else {
                serde_json::Value::String("[redacted]".to_string())
            };
        }

        // Include error message if failed
        if let Err(e) = result {
            details["error"] = serde_json::Value::String(e.to_string());
        }

        details
    }
}
```

#### 4.3 Sensitive Data Detection

```rust
/// Sensitive data detector for real-time scanning
pub struct SensitiveDataDetector {
    patterns: Vec<SensitivePattern>,
    callback: Option<Box<dyn Fn(&SensitiveDataMatch) + Send + Sync>>,
}

#[derive(Debug)]
pub struct SensitiveDataMatch {
    pub pattern_name: String,
    pub location: String,  // e.g., "params.text", "response.ui_tree"
    pub severity: SensitivityLevel,
    pub sample: String,  // Partial match for context (masked)
}

#[derive(Debug, Clone, Copy)]
pub enum SensitivityLevel {
    Low,      // Possibly sensitive (email, phone)
    Medium,   // Likely sensitive (addresses, names)
    High,     // Definitely sensitive (SSN, CC, passwords)
    Critical, // Must never be logged (API keys, tokens)
}

impl SensitiveDataDetector {
    /// Scan a value for sensitive data
    pub fn scan(&self, location: &str, value: &serde_json::Value) -> Vec<SensitiveDataMatch> {
        let mut matches = Vec::new();
        self.scan_recursive(location, value, &mut matches);

        // Invoke callback for high-severity matches
        if let Some(ref callback) = self.callback {
            for m in &matches {
                if m.severity >= SensitivityLevel::High {
                    callback(m);
                }
            }
        }

        matches
    }

    fn scan_recursive(
        &self,
        location: &str,
        value: &serde_json::Value,
        matches: &mut Vec<SensitiveDataMatch>,
    ) {
        match value {
            serde_json::Value::String(s) => {
                for pattern in &self.patterns {
                    if pattern.pattern.is_match(s) {
                        matches.push(SensitiveDataMatch {
                            pattern_name: pattern.name.clone(),
                            location: location.to_string(),
                            severity: Self::pattern_severity(&pattern.name),
                            sample: self.mask_sample(s, &pattern.pattern),
                        });
                    }
                }
            }
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let child_loc = format!("{}.{}", location, key);
                    self.scan_recursive(&child_loc, val, matches);
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, val) in arr.iter().enumerate() {
                    let child_loc = format!("{}[{}]", location, i);
                    self.scan_recursive(&child_loc, val, matches);
                }
            }
            _ => {}
        }
    }

    fn pattern_severity(name: &str) -> SensitivityLevel {
        match name {
            "api_key" | "password_field" | "ssn" | "credit_card" => SensitivityLevel::Critical,
            "token" | "secret" => SensitivityLevel::High,
            "email" | "phone" => SensitivityLevel::Medium,
            _ => SensitivityLevel::Low,
        }
    }

    fn mask_sample(&self, value: &str, pattern: &Regex) -> String {
        // Show masked context around the match
        if let Some(m) = pattern.find(value) {
            let start = m.start().saturating_sub(10);
            let end = (m.end() + 10).min(value.len());
            let context = &value[start..end];
            pattern.replace(context, "[***]").to_string()
        } else {
            "[matched]".to_string()
        }
    }
}
```

#### 4.4 Command Rate Limiting

```rust
/// Configurable rate limiter with multiple strategies
pub struct CommandRateLimiter {
    /// Per-method rate limits
    method_limits: HashMap<RpcMethod, RateLimit>,
    /// Global rate limit
    global_limit: RateLimit,
    /// Per-session rate limits
    session_limits: HashMap<SessionId, SessionRateState>,
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum requests
    pub max_requests: u32,
    /// Time window
    pub window: Duration,
    /// Burst allowance (above max for short periods)
    pub burst: u32,
}

impl CommandRateLimiter {
    /// Default rate limits
    pub fn default_limits() -> HashMap<RpcMethod, RateLimit> {
        let mut limits = HashMap::new();

        // High-frequency operations (element finding)
        let high_freq = RateLimit {
            max_requests: 1000,
            window: Duration::from_secs(60),
            burst: 100,
        };
        limits.insert(RpcMethod::FindElement, high_freq.clone());
        limits.insert(RpcMethod::FindElements, high_freq.clone());

        // Medium-frequency operations (actions)
        let med_freq = RateLimit {
            max_requests: 500,
            window: Duration::from_secs(60),
            burst: 50,
        };
        limits.insert(RpcMethod::Click, med_freq.clone());
        limits.insert(RpcMethod::TypeText, med_freq.clone());

        // Low-frequency operations (expensive)
        let low_freq = RateLimit {
            max_requests: 100,
            window: Duration::from_secs(60),
            burst: 10,
        };
        limits.insert(RpcMethod::CaptureScreenshot, low_freq.clone());
        limits.insert(RpcMethod::GetComponentTree, low_freq.clone());

        limits
    }

    /// Check if a request should be allowed
    pub fn check(
        &mut self,
        session: &SessionId,
        method: &RpcMethod,
    ) -> Result<(), RateLimitError> {
        // Check global limit
        if !self.global_limit.allow() {
            return Err(RateLimitError::GlobalLimitExceeded);
        }

        // Check method-specific limit
        if let Some(limit) = self.method_limits.get_mut(method) {
            if !limit.allow() {
                return Err(RateLimitError::MethodLimitExceeded {
                    method: method.as_str().to_string(),
                });
            }
        }

        // Check session limit
        let session_state = self.session_limits
            .entry(session.clone())
            .or_insert_with(SessionRateState::new);

        if !session_state.allow() {
            return Err(RateLimitError::SessionLimitExceeded);
        }

        Ok(())
    }
}
```

### 5. Compliance Considerations

#### 5.1 Logging Sensitive Operations

All operations that access, modify, or expose sensitive data must be logged:

| Operation | Log Level | Required Fields |
|-----------|-----------|-----------------|
| Connection established | INFO | session_id, target_app, client_ip |
| Connection closed | INFO | session_id, duration, command_count |
| Screenshot captured | INFO | session_id, size, pii_detected |
| UI tree dumped | INFO | session_id, node_count, depth |
| Text input | DEBUG | session_id, element (masked value) |
| Security violation | SECURITY | session_id, violation_type, details |

#### 5.2 Audit Trail for UI Interactions

```rust
/// Complete audit trail entry for UI interaction
#[derive(Debug, Serialize)]
pub struct InteractionAuditEntry {
    /// Unique interaction ID
    pub interaction_id: Uuid,
    /// Session context
    pub session_id: SessionId,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Type of interaction
    pub interaction_type: InteractionType,
    /// Target element (locator used)
    pub element_locator: String,
    /// Element type
    pub element_type: Option<String>,
    /// Action performed
    pub action: String,
    /// Input data (masked if sensitive)
    pub input_data: Option<String>,
    /// Result
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Duration in milliseconds
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub enum InteractionType {
    Click,
    Input,
    Select,
    Check,
    Navigate,
    Screenshot,
    TreeDump,
    Other(String),
}
```

#### 5.3 Data Handling Policies

| Data Type | Logging | Storage | Transmission |
|-----------|---------|---------|--------------|
| Locators | Allowed (sanitized) | In-memory only | Plain |
| Element text | Masked by default | Not stored | Not transmitted |
| Screenshots | Logged metadata only | Temp directory | Optional TLS |
| UI tree | Logged depth/count | Not stored | Plain (sanitized) |
| Passwords | Never logged | Never stored | Never transmitted |
| Session tokens | Hash only | Secure memory | Encrypted |

### 6. Security Configuration

```rust
/// Complete security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Locator validation settings
    pub locator: LocatorSecurityConfig,
    /// Session management settings
    pub session: SessionSecurityConfig,
    /// Rate limiting settings
    pub rate_limit: RateLimitConfig,
    /// Audit logging settings
    pub audit: AuditConfig,
    /// Data sanitization settings
    pub sanitization: SanitizationConfig,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            locator: LocatorSecurityConfig::default(),
            session: SessionSecurityConfig {
                timeout: Duration::from_secs(3600),
                max_sessions_per_host: 5,
                ip_binding: true,
                token_rotation: true,
            },
            rate_limit: RateLimitConfig {
                enabled: true,
                global_max_rps: 100,
                per_method_limits: true,
            },
            audit: AuditConfig {
                enabled: true,
                min_level: AuditLevel::Info,
                include_params: false,
                file_path: None,
            },
            sanitization: SanitizationConfig {
                mask_pii: true,
                redact_passwords: true,
                sanitize_logs: true,
                sanitize_screenshots: false,
            },
        }
    }
}
```

## Consequences

### Positive

1. **Defense in Depth**: Multiple security layers prevent single-point failures
2. **Secure by Default**: Reasonable defaults require no configuration for safety
3. **Compliance Ready**: Audit logging supports regulatory requirements
4. **Injection Prevention**: Locator validation blocks common attack vectors
5. **Data Protection**: Sensitive data sanitization prevents accidental exposure
6. **Performance**: Rate limiting prevents DoS while allowing normal operation
7. **Transparency**: Security hooks allow custom policies without code changes

### Negative

1. **Implementation Complexity**: Significant new code for security layer
2. **Performance Overhead**: Validation adds latency (estimated <5ms)
3. **Configuration Burden**: Security settings need documentation
4. **Breaking Changes**: Strict validation may reject previously-working locators
5. **Maintenance**: Security patterns require ongoing updates

### Risks

1. **False Positives**: Overly strict validation may block legitimate locators
2. **Bypass**: Determined attackers may find validation gaps
3. **Performance Impact**: Complex validation could slow high-frequency tests
4. **Version Skew**: Agent and library security must stay synchronized

## Alternatives Considered

### Alternative 1: Minimal Security (Current State)

Keep existing validation (empty locator check only).

**Rejected because**:
- Does not address injection risks
- No audit trail for compliance
- No protection against data exposure

### Alternative 2: External Security Layer

Use a separate security proxy between library and agent.

**Rejected because**:
- Additional infrastructure complexity
- Latency impact from extra hop
- Harder to maintain and deploy

### Alternative 3: Java Agent-Only Security

Implement all security controls in the Java agent.

**Rejected because**:
- Cannot validate before network transmission
- Agent updates require application restarts
- Limits flexibility for different security policies

## Implementation Plan

### Phase 1: Foundation (Week 1-2)
1. Implement `SecureLocatorValidator` with default patterns
2. Add `SafeAssertionEvaluator` with comparison operators
3. Create `DataSanitizer` with PII patterns
4. Add basic audit logging infrastructure

### Phase 2: Session Security (Week 3)
1. Implement `SecureSessionManager` with token generation
2. Add IP binding and session timeout
3. Create `SecureRpcClient` wrapper

### Phase 3: Hooks and Rate Limiting (Week 4)
1. Implement `SecurityHook` trait and registry
2. Add pre/post command hooks
3. Create `CommandRateLimiter` with default limits

### Phase 4: Integration and Testing (Week 5-6)
1. Integrate security layer into existing code paths
2. Add comprehensive security tests
3. Performance benchmarking and optimization
4. Documentation and configuration guide

## References

- [OWASP Testing Guide - Injection](https://owasp.org/www-project-web-security-testing-guide/)
- [CWE-78: OS Command Injection](https://cwe.mitre.org/data/definitions/78.html)
- [CWE-79: Cross-site Scripting (XSS)](https://cwe.mitre.org/data/definitions/79.html)
- [ADR-002: Locator Syntax Strategy](./ADR-002-locator-syntax-strategy.md)
- [ADR-005: Error Handling Strategy](./ADR-005-error-handling-strategy.md)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
