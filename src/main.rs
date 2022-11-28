mod args;

use args::Args;
use bookery::{build_client, get_hits};
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("querying {}", args.term);
    let client = build_client();
    let Some(hits) = get_hits(&client, &args.term) else {
        return;
    };
    if hits.hits.len() > 0 {
        let doc = &hits.hits[0].doc;
        println!("{}\n{}", doc.chapter[0], doc.sentence[0]);
    }
}
