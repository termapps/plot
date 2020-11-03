use structopt::StructOpt;

use std::io::Result;

/// Pie chart
#[derive(Debug, StructOpt)]
pub struct Pie {}

impl Pie {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
