use clap::{Arg, Command};
use std::{env, sync::Arc, time::Duration};

mod crawler;
mod display;
mod nodes;
mod spiders;

use crate::crawler::Crawler;
use crate::display::print_node_tree;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("url").required(true).index(1))
        .get_matches();

    env::set_var("RUST_LOG", "info,crawler=debug");
    env_logger::init();

    // TODO: Validate that URL is valid URL
    let url = cli.value_of("url").unwrap().to_string();

    let crawler = Crawler::new(Duration::from_millis(200), 2, 500);
    // TODO: rust-tls not working https://github.com/seanmonstar/reqwest/issues/1039
    let spider = Arc::new(spiders::ArcgisSpider::new(url.clone()));
    let nodes = crawler.run(spider).await;

    print_node_tree(url, nodes)?;

    Ok(())
}
