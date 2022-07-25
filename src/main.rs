pub mod morphemes;

use clap::Parser;
use morphemes::Word;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    gloss: String,
}

fn main() {
    let cli = Cli::parse();

    let word = Word::from(cli.gloss);

    println!("{}: {}", word.content, morphemes::gloss_word(&word));
}
