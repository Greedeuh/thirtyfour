use serde::Serialize;
use serde_json::Value;

/// Common trait for all testing-library option types.
/// 
/// This trait provides standard methods for serialization and construction
/// that are shared across all option types.
pub trait TestingLibraryOptions: Serialize + Default {
    /// Create a new instance with default values.
    fn new() -> Self 
    where 
        Self: Sized 
    {
        Self::default()
    }
    
    /// Serialize this options struct to a JSON string.
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    /// Serialize this options struct to a JSON value.
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}