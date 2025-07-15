
I think we can do better on test to reduce the duplication.

### Parameterized tests to reduce duplication
```rust
use rstest::*;

#[derive(Debug)]
enum QueryMethod {
    Get,
    Query, 
    Find,
}

#[derive(Debug)]
struct TestCase {
    method: QueryMethod,
    selector: By,
    html_file: &'static str,
    expected_id: &'static str,
    should_succeed: bool,
}

#[rstest]
#[case::get_role_button(TestCase {
    method: QueryMethod::Get,
    selector: By::role("button"),
    html_file: "sample_page.html",
    expected_id: "button-id",
    should_succeed: true,
})]
#[case::query_role_button(TestCase {
    method: QueryMethod::Query,
    selector: By::role("button"), 
    html_file: "sample_page.html",
    expected_id: "button-id",
    should_succeed: true,
})]
#[case::get_nonexistent(TestCase {
    method: QueryMethod::Get,
    selector: By::role("nonexistent"),
    html_file: "sample_page.html", 
    expected_id: "",
    should_succeed: false,
})]
async fn test_query_method(#[case] test_case: TestCase) {
    let (driver, screen) = get_driver_and_screen(test_case.html_file).await;
    
    let result = match test_case.method {
        QueryMethod::Get => screen.get(test_case.selector).await.map(Some),
        QueryMethod::Query => screen.query(test_case.selector).await,
        QueryMethod::Find => screen.find(test_case.selector).await.map(Some),
    };
    
    match (result, test_case.should_succeed) {
        (Ok(Some(element)), true) => {
            let id = element.id().await.unwrap().unwrap();
            assert_eq!(id, test_case.expected_id);
        }
        (Ok(None), false) => {
            // Expected when using query() on non-existent elements
        }
        (Err(_), false) => {
            // Expected when using get() or find() on non-existent elements  
        }
        _ => panic!("Unexpected result for test case: {:?}", test_case), // TODO: helpfull logs
    }
    
    quit_driver(driver).await;
}
```
