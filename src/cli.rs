use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "modcheck", version = "1.0", author = "Author Name <author@example.com>", about = "Checks mod versions")]
pub struct Cli {
    #[clap(long, value_name = "FILE", help = "Sets the source file")]
    pub src: String,

    #[clap(long, value_name = "LOADER", help = "Sets the loader type")]
    pub loader: Option<String>,

    #[clap(long, value_name = "VERSION", help = "Sets the minimum version", conflicts_with = "exact_version")]
    pub min_version: Option<String>,

    #[clap(long, value_name = "VERSION", help = "Sets the exact version", conflicts_with = "min_version")]
    pub exact_version: Option<String>,
}
