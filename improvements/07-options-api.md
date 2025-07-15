# 7. Options API Improvement

## Problem
Current `*_with_options()` methods are verbose and not ergonomic.

## Current API
```rust
// Verbose and clunky
let opts = ByRoleOptions::new().exact(true);
let button = screen.get(By::role_with_options("button", opts)).await?;

// Many separate methods
By::role("button")
By::role_with_options("button", opts)
By::text("Welcome")
By::text_with_options("Welcome", text_opts)
```

## Solution: Fluent Builder API
```rust
// Clean and fluent
let button = screen.get(By::role("button").exact(true).hidden(false)).await?;
let heading = screen.get(By::text("Welcome").exact(true)).await?;
```

## Implementation

### Enhanced By enum with builder methods
```rust
impl By {
    // Chainable option methods for all selector types
    pub fn exact(mut self, exact: bool) -> Self {
        match &mut self {
            By::Role(_, ref mut opts) => {
                let role_opts = opts.get_or_insert_with(|| Options::Role(ByRoleOptions::default()));
                if let Options::Role(ref mut role_opts) = role_opts {
                    role_opts.exact = Some(exact);
                }
            }
            By::Text(_, ref mut opts) => {
                let text_opts = opts.get_or_insert_with(|| Options::Text(ByTextOptions::default()));
                if let Options::Text(ref mut text_opts) = text_opts {
                    text_opts.exact = Some(exact);
                }
            }
            // ... handle other variants
        }
        self
    }
    
    pub fn hidden(mut self, hidden: bool) -> Self {
        // Similar implementation for hidden option
        self
    }
    
    pub fn level(mut self, level: u8) -> Self {
        // For heading level in role queries
        self
    }
}
```

### Trait-based approach for type safety
```rust
trait HasExactOption {
    fn exact(self, exact: bool) -> Self;
}

trait HasHiddenOption {
    fn hidden(self, hidden: bool) -> Self;
}

impl HasExactOption for By {
    fn exact(mut self, exact: bool) -> Self {
        // Implementation
    }
}

impl HasHiddenOption for By {
    fn hidden(mut self, hidden: bool) -> Self {
        // Implementation  
    }
}
```

### Alternative: Selector-specific builders
```rust
pub struct RoleSelector {
    value: String,
    options: ByRoleOptions,
}

impl RoleSelector {
    pub fn exact(mut self, exact: bool) -> Self {
        self.options.exact = Some(exact);
        self
    }
    
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.options.hidden = Some(hidden);
        self
    }
    
    pub fn level(mut self, level: u8) -> Self {
        self.options.level = Some(level);
        self
    }
}

impl From<RoleSelector> for By {
    fn from(selector: RoleSelector) -> Self {
        By::Role(selector.value, Some(Options::Role(selector.options)))
    }
}

impl By {
    pub fn role(value: impl Into<String>) -> RoleSelector {
        RoleSelector {
            value: value.into(),
            options: ByRoleOptions::default(),
        }
    }
}
```

## Usage Examples
```rust
// Simple usage (no change)
let button = screen.get(By::role("button")).await?;

// With options (much cleaner)
let submit_button = screen.get(
    By::role("button")
        .exact(true)
        .hidden(false)
).await?;

// Complex role query
let heading = screen.get(
    By::role("heading")
        .level(2)
        .exact(false)
).await?;

// Text query with options
let welcome_text = screen.get(
    By::text("Welcome")
        .exact(true)
).await?;
```

## Benefits
- Much more ergonomic API
- Discoverable through IDE autocomplete
- Type-safe option combinations
- Backwards compatible
- Reduces boilerplate code

## Impact
- **Ergonomics**: High (much cleaner API)
- **Complexity**: Medium (requires careful design)
- **Breaking**: Low (can maintain current API alongside)