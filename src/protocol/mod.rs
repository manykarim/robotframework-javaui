//! JSON-RPC protocol for communication with the Java agent

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// Request ID
    pub id: u64,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Result (if success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error (if failure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID
    pub id: u64,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// RPC Methods supported by the Java agent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpcMethod {
    // Discovery
    Ping,
    ListWindows,
    GetComponentTree,

    // Element Location
    FindElement,
    FindElements,
    WaitForElement,

    // Element Inspection
    GetElementProperties,
    GetElementBounds,
    GetElementText,
    GetTableData,
    GetTreeNodes,
    GetListItems,

    // Actions
    Click,
    DoubleClick,
    RightClick,
    TypeText,
    ClearText,
    SelectItem,
    Focus,

    // Table Operations
    SelectTableCell,
    GetTableCellValue,
    SetTableCellValue,
    GetTableRowCount,
    GetTableColumnCount,

    // Tree Operations
    ExpandTreeNode,
    CollapseTreeNode,
    SelectTreeNode,

    // Waits
    WaitUntilEnabled,
    WaitUntilVisible,
    WaitUntilNotVisible,

    // Screenshots
    CaptureScreenshot,
}

impl RpcMethod {
    /// Get the method name string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ping => "ping",
            Self::ListWindows => "listWindows",
            Self::GetComponentTree => "getComponentTree",
            Self::FindElement => "findElement",
            Self::FindElements => "findElements",
            Self::WaitForElement => "waitForElement",
            Self::GetElementProperties => "getElementProperties",
            Self::GetElementBounds => "getElementBounds",
            Self::GetElementText => "getElementText",
            Self::GetTableData => "getTableData",
            Self::GetTreeNodes => "getTreeNodes",
            Self::GetListItems => "getListItems",
            Self::Click => "click",
            Self::DoubleClick => "doubleClick",
            Self::RightClick => "rightClick",
            Self::TypeText => "typeText",
            Self::ClearText => "clearText",
            Self::SelectItem => "selectItem",
            Self::Focus => "focus",
            Self::SelectTableCell => "selectTableCell",
            Self::GetTableCellValue => "getTableCellValue",
            Self::SetTableCellValue => "setTableCellValue",
            Self::GetTableRowCount => "getTableRowCount",
            Self::GetTableColumnCount => "getTableColumnCount",
            Self::ExpandTreeNode => "expandTreeNode",
            Self::CollapseTreeNode => "collapseTreeNode",
            Self::SelectTreeNode => "selectTreeNode",
            Self::WaitUntilEnabled => "waitUntilEnabled",
            Self::WaitUntilVisible => "waitUntilVisible",
            Self::WaitUntilNotVisible => "waitUntilNotVisible",
            Self::CaptureScreenshot => "captureScreenshot",
        }
    }
}

impl JsonRpcRequest {
    /// Create a new request
    pub fn new(method: RpcMethod, params: serde_json::Value, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.as_str().to_string(),
            params: Some(params),
            id,
        }
    }

    /// Create a request with no parameters
    pub fn new_no_params(method: RpcMethod, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.as_str().to_string(),
            params: None,
            id,
        }
    }
}

impl JsonRpcResponse {
    /// Check if this is an error response
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// Get the result or error
    pub fn into_result(self) -> Result<serde_json::Value, JsonRpcError> {
        if let Some(error) = self.error {
            Err(error)
        } else {
            Ok(self.result.unwrap_or(serde_json::Value::Null))
        }
    }
}

/// Standard JSON-RPC error codes
pub mod error_codes {
    /// Parse error
    pub const PARSE_ERROR: i32 = -32700;
    /// Invalid request
    pub const INVALID_REQUEST: i32 = -32600;
    /// Method not found
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid params
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal error
    pub const INTERNAL_ERROR: i32 = -32603;

    // Custom error codes (application-specific)
    /// Element not found
    pub const ELEMENT_NOT_FOUND: i32 = -32000;
    /// Multiple elements found
    pub const MULTIPLE_ELEMENTS: i32 = -32001;
    /// Element not interactable
    pub const NOT_INTERACTABLE: i32 = -32002;
    /// Timeout
    pub const TIMEOUT: i32 = -32003;
    /// Stale element
    pub const STALE_ELEMENT: i32 = -32004;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let req = JsonRpcRequest::new(
            RpcMethod::FindElement,
            serde_json::json!({"locator": "JButton#submit"}),
            1,
        );

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("findElement"));
        assert!(json.contains("JButton#submit"));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","result":{"id":"elem1"},"id":1}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert!(!resp.is_error());
        assert!(resp.result.is_some());
    }

    #[test]
    fn test_error_response() {
        let json = r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"Element not found"},"id":1}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert!(resp.is_error());
        assert_eq!(resp.error.as_ref().unwrap().code, -32000);
    }
}
