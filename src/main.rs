use clap::Parser;

use rocket::rocket::Rocket;

mod rocket;

enum Palette {
    America,
}

#[derive(Parser, Debug)]
#[clap(name = "rocket")]
struct RocketOpts {
    #[clap(short, long)]
    height: usize,
    #[clap(short, long, default_value="america")]
    palette: String,
}

fn main() {
    // Choose color palette
    // Height
    // End must be > "1"
    // Different sections might have couplers to join different widths
    let args = RocketOpts::parse();

    let rkt = Rocket::new(args.height);
    println!("{}", rkt);
}
