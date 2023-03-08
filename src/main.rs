use clap::Parser;
use color_eyre::Report;
use gelper::setup::setup;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "synker", version = "0.1.0", author = "Maciej Urban")]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

fn main() -> Result<(), Report> {
    setup()?;
    let args = Args::parse();
    info!(args.config);

    Ok(())
}
