mod pie;

use console::set_colors_enabled;
use structopt::{clap::AppSettings, StructOpt};

/// Command Line Utility for Stylish Interactive Charts
#[derive(Debug, StructOpt)]
#[structopt(name = "plot")]
#[structopt(global_setting = AppSettings::VersionlessSubcommands)]
struct Plot {
    #[structopt(subcommand)]
    cmd: PlotSubcommand,
}

#[derive(Debug, StructOpt)]
enum PlotSubcommand {
    Pie(pie::Pie),
}

fn main() {
    let program = Plot::from_args();
    set_colors_enabled(true);

    match program.cmd {
        PlotSubcommand::Pie(x) => x.run(),
    }
    .unwrap();
}
