mod core;
use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, SubCommand};
use env_logger;

fn main() -> Result<()> {
    env_logger::init();
    let matches = App::new("ChartMuseum command line interface")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .required(true)
                .env("CHARTMUSEUM_URL")
                .help("ChartMuseum URL with scheme and port, eg: http://127.0.0.1:8080.")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("list").about("list charts name in chartmuseum"))
        .subcommand(
            SubCommand::with_name("versions")
                .about("list all version of chart")
                .arg(
                    Arg::with_name("CHART")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("delete given version of chart")
                .arg(
                    Arg::with_name("CHART")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("VERSION")
                        .required(true)
                        .takes_value(true)
                        .index(2),
                ),
        )
        .get_matches();

    let endpoint_input = matches.value_of("url").unwrap();
    // handle endpoint trailing slash
    let endpoint = format!("{}/", endpoint_input.trim_end_matches("/"));
    let client = core::client::Client::new(&endpoint, None)?;

    match matches.subcommand() {
        ("list", Some(_arg_matches)) => {
            let repos = client.list_charts()?;
            for chart_name in &repos {
                println!("{}", chart_name);
            }
        }
        ("versions", Some(arg_matches)) => {
            let chart_name = arg_matches.value_of("CHART").unwrap();
            let metadata = client.chart_versions(chart_name)?;
            for versioned_metadata in &metadata {
                println!("{}", versioned_metadata.version)
            }
        }
        ("delete", Some(arg_matches)) => {
            let chart_name = arg_matches.value_of("CHART").unwrap();
            let version = arg_matches.value_of("VERSION").unwrap();
            client.del_chart_version(chart_name, version);
        },
        _ => {
            ()
        }
    }
    Ok(())
}
