// use clap::{App, Arg, SubCommand};
use std::{env, sync::Arc, time::Duration};

mod crawler;
mod nodes;
mod spiders;

use crate::crawler::Crawler;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let cli = App::new(clap::crate_name!())
    //     .version(clap::crate_version!())
    //     .about(clap::crate_description!())
    //     .subcommand(SubCommand::with_name("spiders").about("List all spiders"))
    //     .subcommand(
    //         SubCommand::with_name("run").about("Run a spider").arg(
    //             Arg::with_name("spider")
    //                 .short("s")
    //                 .long("spider")
    //                 .help("The spider to run")
    //                 .takes_value(true)
    //                 .required(true),
    //         ),
    //     )
    //     .setting(clap::AppSettings::ArgRequiredElseHelp)
    //     .setting(clap::AppSettings::VersionlessSubcommands)
    //     .get_matches();

    env::set_var("RUST_LOG", "info,crawler=debug");
    env_logger::init();

    let crawler = Crawler::new(Duration::from_millis(200), 2, 500);
    // TODO: rust-tls not working https://github.com/seanmonstar/reqwest/issues/1039
    let spider = Arc::new(spiders::ArcgisSpider::new(
        "https://gisapps.cityofchicago.org/arcgis/rest/services/".to_string(),
    ));
    crawler.run(spider).await;

    Ok(())
}
