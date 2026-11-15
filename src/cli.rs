use crate::algorithm::Algorithm;
use clap::Parser;

#[derive(Parser, Debug)]
/// A simple file compressor.
///
/// Compress files and directories without having to remember the correct commands and parameters.
pub struct Args {
    /// Input file, directory, or glob
    pub input: String,

    /// Output archive file (optional)
    /// If not specified, the input file path is used along with an appropriate file extension.
    pub output: Option<String>,

    #[arg(short, long, value_enum, default_value_t = Algorithm::SevenZ)]
    /// The compression algrorithm to use
    pub algorithm: Algorithm,

    #[arg(short, long)]
    /// Delete the input after compressing
    ///
    /// [default: false]
    pub delete: bool,
}
