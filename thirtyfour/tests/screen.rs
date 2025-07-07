use crate::common::*;
use assert_matches::assert_matches;
use rstest::rstest;
use std::time::Duration;
use thirtyfour::components::{ElementResolverMulti, ElementResolverSingle};
use thirtyfour::error::WebDriverErrorInner;
use thirtyfour::support::block_on;
use thirtyfour::{components::SelectElement, prelude::*};

mod common;

#[rstest]
fn by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;
        
        Ok(())
    })
}
