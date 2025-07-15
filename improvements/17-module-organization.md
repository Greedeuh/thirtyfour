# 17. Module Organization

## Problem
All option types are separate modules but follow identical patterns - could be unified.

## Current Structure
```
src/
├── lib.rs
├── alt_text.rs       // ByAltTextOptions + impl
├── configure.rs      // Configuration options
├── display_value.rs  // ByDisplayValueOptions + impl  
├── label_text.rs     // ByLabelTextOptions + impl
├── placeholder_text.rs // ByPlaceholderTextOptions + impl
├── role.rs           // ByRoleOptions + impl + TextMatch
├── test_id.rs        // ByTestIdOptions + impl
├── text.rs           // ByTextOptions + impl
└── title.rs          // ByTitleOptions + impl
```

## Issues
- Each module has nearly identical code
- Hard to maintain consistency across modules
- Difficult to add new common functionality
- Scattered related functionality

## Solution: Unified Module Structure

### Option 1: Single options module
```
src/
├── lib.rs
├── options/
│   ├── mod.rs        // All option types + common traits
│   ├── builders.rs   // Builder pattern implementations
│   └── macros.rs     // Code generation macros
├── selectors/
│   ├── mod.rs        // By enum + selector logic
│   └── validation.rs // Input validation
├── query/
│   ├── mod.rs        // QueryExecutor + Screen
│   └── script.rs     // JavaScript generation
└── configure.rs      // Global configuration
```

### Implementation

#### `src/options/mod.rs`
```rust
use serde::Serialize;

// Re-export everything from this module
pub use builders::*;
pub use macros::*;

mod builders;
mod macros;

// Common trait for all option types
pub trait OptionBuilder: Serialize + Default + Clone {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

// All option structs in one place
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByRoleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // ... all role options
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByTextOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByLabelTextOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

// ... all other option types

// Implement common trait for all
impl OptionBuilder for ByRoleOptions {}
impl OptionBuilder for ByTextOptions {}
impl OptionBuilder for ByLabelTextOptions {}
// ... etc

// Unified Options enum for polymorphism
#[derive(Debug, Clone)]
pub enum Options {
    Role(ByRoleOptions),
    Text(ByTextOptions),
    LabelText(ByLabelTextOptions),
    PlaceholderText(ByPlaceholderTextOptions),
    DisplayValue(ByDisplayValueOptions),
    AltText(ByAltTextOptions),
    Title(ByTitleOptions),
    TestId(ByTestIdOptions),
}

impl Options {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        match self {
            Options::Role(opts) => opts.to_json_string(),
            Options::Text(opts) => opts.to_json_string(),
            Options::LabelText(opts) => opts.to_json_string(),
            // ... etc
        }
    }
}
```

#### `src/options/builders.rs`
```rust
use super::*;

// Macro to generate builder methods
macro_rules! impl_common_options {
    ($type:ty) => {
        impl $type {
            pub fn new() -> Self {
                Self::default()
            }
            
            pub fn exact(mut self, exact: bool) -> Self {
                self.exact = Some(exact);
                self
            }
        }
    };
}

// Apply to types that have exact option
impl_common_options!(ByRoleOptions);
impl_common_options!(ByTextOptions);
impl_common_options!(ByLabelTextOptions);
// ... etc

// Role-specific builders
impl ByRoleOptions {
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }
    
    pub fn level(mut self, level: u8) -> Self {
        self.level = Some(level);
        self
    }
    
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

// Label-specific builders
impl ByLabelTextOptions {
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }
}
```

#### `src/selectors/mod.rs`
```rust
use crate::options::{Options, *};

pub mod validation;

// Main selector enum
#[derive(Debug, Clone)]
pub enum By {
    Role(String, Option<Options>),
    Text(String, Option<Options>),
    LabelText(String, Option<Options>),
    PlaceholderText(String, Option<Options>),
    DisplayValue(String, Option<Options>),
    AltText(String, Option<Options>),
    Title(String, Option<Options>),
    TestId(String, Option<Options>),
}

impl By {
    // Constructors
    pub fn role(value: impl Into<String>) -> Self {
        Self::Role(value.into(), None)
    }
    
    pub fn role_with_options(value: impl Into<String>, options: ByRoleOptions) -> Self {
        Self::Role(value.into(), Some(Options::Role(options)))
    }
    
    // ... other constructors
    
    // Common methods
    pub fn value(&self) -> &str {
        match self {
            By::Role(value, _) => value,
            By::Text(value, _) => value,
            // ... etc
        }
    }
    
    pub fn selector_type(&self) -> &'static str {
        match self {
            By::Role(_, _) => "role",
            By::Text(_, _) => "text",
            // ... etc
        }
    }
    
    pub fn function_suffix(&self) -> &'static str {
        match self {
            By::Role(_, _) => "Role",
            By::Text(_, _) => "Text",
            // ... etc
        }
    }
}
```

### Option 2: Feature-based organization
```
src/
├── lib.rs
├── core/
│   ├── mod.rs        // Screen + core functionality
│   ├── query.rs      // QueryExecutor
│   └── script.rs     // JavaScript generation
├── selectors/
│   ├── mod.rs        // By enum + common functionality
│   ├── role.rs       // Role-specific logic + validation
│   ├── text.rs       // Text-specific logic + validation
│   └── accessibility.rs // Accessibility helpers
├── options/
│   ├── mod.rs        // Common option traits + Options enum
│   ├── types.rs      // All option struct definitions
│   └── builders.rs   // Builder implementations
└── config/
    ├── mod.rs        // Configuration options
    └── defaults.rs   // Default configurations
```

### Option 3: Type-driven organization
```
src/
├── lib.rs
├── types/
│   ├── mod.rs        // All type definitions
│   ├── selectors.rs  // By enum + selector types
│   ├── options.rs    // All option types
│   └── errors.rs     // Error types
├── builders/
│   ├── mod.rs        // Builder pattern implementations
│   ├── selectors.rs  // Selector builders
│   └── options.rs    // Option builders
├── execution/
│   ├── mod.rs        // Query execution logic
│   ├── script.rs     // JavaScript generation
│   └── validation.rs // Input validation
└── utils/
    ├── mod.rs        // Utility functions
    └── serialization.rs // JSON serialization helpers
```

## Benefits
- Related functionality grouped together
- Easier to maintain consistency
- Single source of truth for each concept
- Clearer module boundaries
- Easier to find and modify code
- Better IDE navigation

## Migration Strategy
1. Create new module structure alongside existing
2. Move common functionality to unified modules
3. Update imports gradually
4. Remove old modules once migration complete
5. Update documentation and examples

## Impact
- **Organization**: High (much cleaner structure)
- **Maintainability**: High (easier to maintain)
- **Complexity**: Medium (requires careful migration)
- **Breaking**: Medium (import paths change, but can be phased)

mentor review: we have to discussed it but that's great