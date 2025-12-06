use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File or folder to be watched
    #[arg(short, long)]
    pub watch: String,
    /// Tab URL target part
    #[arg(short, long)]
    pub target: String,
}
