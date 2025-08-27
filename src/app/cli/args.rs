use ::clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'p', long = "path")]
    pub storage_path: String,
}
