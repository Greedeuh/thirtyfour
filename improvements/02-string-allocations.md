# 2. String Allocations Optimization

## Problem
Heavy use of `clone()`, `to_string()`, and `format!()` macros (416 occurrences). Many unnecessary string allocations.

## Current Patterns
```rust
// Excessive cloning
let method_name = format!("{method_prefix}{function_suffix}");
let script = format!("return {base_call};");

// Unnecessary string conversions
value.into() // when &str would suffice
selector.function_suffix() // returns &str but often cloned
```

## Solutions

### Use `Cow<str>` for flexible string handling
```rust
use std::borrow::Cow;

pub enum By {
    Role(Cow<'static, str>, Option<Options>),
    Text(Cow<'static, str>, Option<Options>),
    // ...
}
```

### Use `&str` parameters where possible
```rust
// Instead of
pub fn role(value: impl Into<String>) -> Self

// Use
pub fn role(value: impl Into<Cow<'static, str>>) -> Self
```

### Cache formatted strings
```rust
impl QueryExecutor {
    fn get_method_name(&self, prefix: &str, suffix: &str) -> String {
        // Cache common combinations
        static COMMON_METHODS: Lazy<HashMap<(&str, &str), String>> = Lazy::new(|| {
            let mut map = HashMap::new();
            map.insert(("getBy", "Role"), "getByRole".to_string());
            map.insert(("queryBy", "Role"), "queryByRole".to_string());
            // ... other common combinations
            map
        });
        
        COMMON_METHODS.get(&(prefix, suffix))
            .cloned()
            .unwrap_or_else(|| format!("{prefix}{suffix}"))
    }
}
```

## Benefits
- Reduced memory allocations
- Better performance
- Less pressure on garbage collector
- More efficient string handling

## Impact
- **Performance**: High (significant allocation reduction)
- **Complexity**: Medium (requires careful API design)
- **Breaking**: Potentially medium (API signature changes)


mentor review: it's a bit overkill to use Co or to get rid of format!. When it's easier to read we should keep it but I agree that when we require a String we should require a &str when possible