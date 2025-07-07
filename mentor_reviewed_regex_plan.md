# Mentor-Reviewed Regex Implementation Plan

## Mentor's Feedback

**Question**: "Can we just do this `TextMatch::Regex("/Save.*/".to_string())` into `/Save.*/`?"

**Analysis**: The mentor is suggesting a much simpler approach - users provide the complete regex literal (including slashes) as a string, and we just remove the quotes during serialization.

## Simplified Approach Based on Mentor Review

### 1. User API Design

Users would create regex patterns like this:
```rust
// User provides complete regex literal as string
TextMatch::Regex("/Save.*/".to_string())        // Basic regex
TextMatch::Regex("/save/i".to_string())         // With flags
TextMatch::Regex("/^Submit.*Form$/".to_string()) // Anchored
```

### 2. Simple String Replacement

Instead of complex JSON processing, just remove quotes from regex strings:

```rust
impl Serialize for TextMatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        match self {
            TextMatch::Exact(s) => s.serialize(serializer),
            TextMatch::Substring(s) => s.serialize(serializer),
            TextMatch::Regex(pattern) => {
                // For regex, we need to serialize without quotes
                // This is tricky with standard JSON serialization
                // We'll handle this at the script generation level
                pattern.serialize(serializer)
            }
        }
    }
}
```

### 3. Script Generation Fix

The key insight is to handle regex at the script generation level:

```rust
async fn execute_tl_role_method(&self, method: &str, role: &str, options: Option<&ByRoleOptions>) -> WebDriverResult<ScriptRet> {
    let script = match options {
        Some(opts) => {
            let options_json = opts.to_json_string().map_err(|e| {
                crate::error::WebDriverError::Json(format!("Failed to serialize role options: {}", e))
            })?;
            
            // Convert regex strings to literals: "/pattern/" -> /pattern/
            let processed_json = process_regex_in_json(&options_json);
            
            format!("return window.__TL__.{}(document, '{}', {});", method, role, processed_json)
        }
        None => {
            format!("return window.__TL__.{}(document, '{}');", method, role)
        }
    };
    
    self.driver.execute(script, vec![]).await
}

fn process_regex_in_json(json: &str) -> String {
    // Simple replacement: "\/pattern\/" -> /pattern/
    // Look for quoted strings that start and end with /
    let re = regex::Regex::new(r#""(/[^"]*/)""#).unwrap();
    re.replace_all(json, "$1").to_string()
}
```

## Implementation Steps

### Step 1: Update Script Processing
1. Add `process_regex_in_json` function
2. Modify `execute_tl_role_method` to use it
3. Keep existing `TextMatch::Regex` serialization as-is

### Step 2: Add Validation
```rust
impl TextMatch {
    pub fn validate_regex(&self) -> Result<(), String> {
        match self {
            TextMatch::Regex(pattern) => {
                // Check if it looks like a regex literal
                if !pattern.starts_with('/') || !pattern.ends_with('/') {
                    return Err("Regex pattern must start and end with '/' (e.g., '/pattern/')".to_string());
                }
                
                // Extract pattern without slashes and validate
                let inner_pattern = &pattern[1..pattern.len()-1];
                regex::Regex::new(inner_pattern)
                    .map_err(|e| format!("Invalid regex pattern: {}", e))?;
                
                Ok(())
            }
            _ => Ok(())
        }
    }
}
```

### Step 3: Update Tests
```rust
// Update existing test to use proper regex literal
#[rstest]
fn test_name_regex_match(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        
        // Use proper regex literal syntax
        let options = ByRoleOptions::new()
            .name(TextMatch::Regex("/Save.*Document/".to_string()));
        
        let button = screen.get_by_role_with_options("button", &options).await?;
        assert_eq!(button.text().await?, "Save Document");

        Ok(())
    })
}
```

## Benefits of Mentor's Approach

1. **Extremely Simple**: Just string replacement, no complex JSON manipulation
2. **User-Friendly**: Users provide complete regex literals as they would write in JavaScript
3. **Flexible**: Supports all regex features including flags
4. **Minimal Code Changes**: Only affects script generation, not serialization logic

## Example Usage

```rust
// Basic pattern matching
TextMatch::Regex("/Save.*/".to_string())

// Case-insensitive matching
TextMatch::Regex("/save/i".to_string())

// Word boundaries
TextMatch::Regex("/\\bSubmit\\b/".to_string())

// Multiple alternatives
TextMatch::Regex("/(Save|Export)/".to_string())
```

## Generated JavaScript

```javascript
// Input: TextMatch::Regex("/Save.*/")
// JSON: {"name": "/Save.*/"}
// Processed: {"name": /Save.*/}
// Final: window.__TL__.getByRole(document, 'button', {name: /Save.*/})
```

## Validation Rules

1. **Must start and end with '/'**: `/pattern/`
2. **May include flags**: `/pattern/flags`
3. **Inner pattern must be valid regex**: Validated with Rust regex crate
4. **Proper escaping**: Users responsible for escaping special characters

## Migration Strategy

1. **Update existing tests** to use proper regex literal syntax
2. **Add validation** to catch malformed regex patterns early
3. **Document new usage** with clear examples
4. **Test thoroughly** with various regex patterns

This approach directly addresses the mentor's question and provides the simplest possible implementation while maintaining full regex functionality.