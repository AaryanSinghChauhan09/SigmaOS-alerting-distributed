//! SigmaOS-alerting-distributed - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-alerting-distributed");
    println!("SigmaOS SigmaOS-alerting-distributed v0.1.0");
    Ok(())
}
