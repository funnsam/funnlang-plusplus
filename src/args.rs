pub use clap::*;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[arg(help = "The file to be compiled")]
    pub input: String,

    #[arg(short, long, help = "Output file's filename", default_value = "a.out")]
    pub output: String,

    #[arg(long, help = "The style for error reporting", value_enum, default_value_t = ErrorStyle::Normal)]
    pub error_style: ErrorStyle,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ErrorStyle {
    Normal,
    Compact,
    NoHighlight,
    Simple
}
