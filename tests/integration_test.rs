// Integration test for SigmaOS-alerting-distributed
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use SigmaOS_alerting_distributed as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
