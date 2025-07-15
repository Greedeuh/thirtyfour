# 15. Code Duplication

## Problem
Repetitive patterns across option modules (`text.rs`, `role.rs`, etc.) could be macro-generated or use generics.

## Current Duplication

### Option struct patterns
All option modules follow the same pattern:

```rust
// text.rs
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByTextOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl ByTextOptions {
    pub fn new() -> Self { Self::default() }
    pub fn exact(mut self, exact: bool) -> Self { self.exact = Some(exact); self }
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> { serde_json::to_string(self) }
    pub fn to_json_value(&self) -> Result<Value, serde_json::Error> { serde_json::to_value(self) }
}

// title.rs - nearly identical
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByTitleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}
// ... same impl pattern
```

## Solutions

### Macro-generated option structs
```rust
// src/macros.rs
macro_rules! define_options {
    (
        $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Default, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $(
                $(#[$field_attr])*
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $field: $field_type,
            )*
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
                serde_json::to_string(self)
            }

            pub fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
                serde_json::to_value(self)
            }

            $(
                pub fn $field(mut self, value: <$field_type as OptionField>::Inner) -> Self {
                    self.$field = Some(value);
                    self
                }
            )*
        }
    };
}

// Helper trait for option field types
trait OptionField {
    type Inner;
}

impl OptionField for Option<bool> {
    type Inner = bool;
}

impl OptionField for Option<String> {
    type Inner = String;
}

impl OptionField for Option<u8> {
    type Inner = u8;
}
```

### Usage of macro
```rust
// text.rs
use crate::macros::define_options;

define_options! {
    ByTextOptions {
        exact: Option<bool>,
    }
}

// role.rs
define_options! {
    ByRoleOptions {
        exact: Option<bool>,
        hidden: Option<bool>,
        level: Option<u8>,
        name: Option<String>,
        description: Option<String>,
        current: Option<String>,
        expanded: Option<bool>,
        checked: Option<bool>,
        pressed: Option<bool>,
        selected: Option<bool>,
        busy: Option<bool>,
        value_min: Option<f64>,
        value_max: Option<f64>,
        value_now: Option<f64>,
        value_text: Option<String>,
    }
}
```

### Generic option builder
```rust
// src/options/mod.rs
use serde::Serialize;
use std::marker::PhantomData;

pub trait OptionBuilder: Default + Serialize {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

pub struct GenericOptions<T> {
    exact: Option<bool>,
    hidden: Option<bool>,
    _phantom: PhantomData<T>,
}

// Marker types for different option contexts
pub struct TextContext;
pub struct RoleContext;
pub struct LabelContext;

impl<T> GenericOptions<T> {
    pub fn new() -> Self {
        Self {
            exact: None,
            hidden: None,
            _phantom: PhantomData,
        }
    }
    
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = Some(exact);
        self
    }
}

impl GenericOptions<RoleContext> {
    pub fn level(mut self, level: u8) -> Self {
        // Role-specific method would require extending the struct
        // This approach has limitations
        self
    }
}

pub type ByTextOptions = GenericOptions<TextContext>;
pub type ByRoleOptions = GenericOptions<RoleContext>;
```

### Trait-based approach with derive macro
```rust
// Better approach using a custom derive macro
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TestingLibraryOptions)]
pub fn derive_testing_library_options(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
                serde_json::to_string(self)
            }

            pub fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
                serde_json::to_value(self)
            }
        }
        
        impl crate::options::OptionBuilder for #name {}
    };

    TokenStream::from(expanded)
}

// Usage:
#[derive(Debug, Clone, Default, Serialize, TestingLibraryOptions)]
#[serde(rename_all = "camelCase")]
pub struct ByTextOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl ByTextOptions {
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = Some(exact);
        self
    }
}
```

### Unified options module
```rust
// src/options.rs - Single file for all option types
use serde::Serialize;

pub trait OptionBuilder: Serialize + Default {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

// Common options that most selectors support
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

// Role-specific options extend common options
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByRoleOptions {
    #[serde(flatten)]
    pub common: CommonOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // ... other role-specific options
}

// Text options only need common options
pub type ByTextOptions = CommonOptions;

// Implement builders using macros for the repeated patterns
macro_rules! impl_common_builders {
    ($type:ty) => {
        impl $type {
            pub fn new() -> Self {
                Self::default()
            }
            
            pub fn exact(mut self, exact: bool) -> Self {
                self.exact = Some(exact);
                self
            }
            
            pub fn hidden(mut self, hidden: bool) -> Self {
                self.hidden = Some(hidden);
                self
            }
        }
        
        impl OptionBuilder for $type {}
    };
}

impl_common_builders!(CommonOptions);

impl ByRoleOptions {
    pub fn level(mut self, level: u8) -> Self {
        self.level = Some(level);
        self
    }
    
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}
```

## Benefits
- Eliminates repetitive code
- Easier to maintain and extend
- Consistent API across all option types
- Reduces chance of copy-paste errors
- Single source of truth for option patterns

## Impact
- **Maintainability**: High (much easier to maintain)
- **Code Quality**: High (eliminates duplication)
- **Complexity**: Medium (requires macro/generic design)
- **Breaking**: Medium (might change option module structure)

mentor review: it's a bit too much to define macro but the rest is great