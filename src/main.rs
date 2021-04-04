#![feature(nll)]
#![feature(associated_type_bounds)]
#[macro_use]
extern crate enum_display_derive;
pub mod app;
pub mod candles_range;
pub mod candles_utils;
pub mod config;
pub mod model;
pub mod repository;
pub mod service;
pub mod tac_plotters;
pub mod utils;
use crate::app::Application;
use crate::service::checker::Checker;
use crate::service::streamer::Streamer;
use crate::service::technicals::ema_tac::EmaTac;
use crate::service::technicals::macd::macd_tac::MacdTac;
use candles_utils::str_to_datetime;
use config::{candles_selection::CandlesSelection, selection::Selection};
use eyre::Result;
use log::{info, Level, LevelFilter};
use repository::repository_candle::RepositoryCandle;
use service::{exchange::Exchange, technicals::technical::TechnicalDefinition};
use std::collections::HashMap;
#[cfg(debug_assertions)]
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Commands")]
enum Command {
    /// Check content
    Check {},
    /// Synchronize
    Sync {},
    /// Fix records
    Fix {},
    /// Delete all candles'
    DeleteAll,
    /// List
    List {},
    /// Import from exchange
    Import {},
    /// Plot graph
    Plot {},
    /// Triangle
    Triangle {},
    /// Interactive stream
    Stream {},
    /// Run trader bot back test
    BackTest {},
    /// Run script trader bot back test
    ScriptBackTest {
        /// Rhai script file
        #[structopt(short, long)]
        file: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rustrade", about = "A Rust Bot Trade")]
struct Args {
    /// Enabled debug level
    #[structopt(short, long)]
    debug: bool,
    /// Symbol (e.g. BTCUSDT)
    #[structopt(short = "y", long, default_value = "BTCUSDT")]
    symbol: String,
    /// Minutes (e.g. 15)
    #[structopt(short, long, default_value = "15")]
    minutes: u32,
    /// Start date time
    #[structopt(short, long, default_value = "2020-11-01 00:00:00")]
    start_time: String,
    /// End date time
    #[structopt(short, long, default_value = "2020-12-01 00:00:00")]
    end_time: String,
    #[structopt(subcommand)]
    command: Command,
}

pub fn selection_factory(candles_selection: CandlesSelection) -> Selection {
    let mut tacs = HashMap::new();
    for tac in vec![MacdTac::definition(), EmaTac::definition()] {
        tacs.insert(tac.name.clone(), tac);
    }
    Selection {
        tacs,
        candles_selection,
        image_name: "out/stock.png".to_string(),
    }
}

fn create_repo() -> Result<RepositoryCandle> {
    RepositoryCandle::new(LevelFilter::Debug)
}

fn create_exchange() -> Result<Exchange> {
    Exchange::new(Level::Debug)
}

fn candles_selection_from_arg(opt: &Args) -> CandlesSelection {
    CandlesSelection::from(
        &opt.symbol,
        &opt.minutes,
        str_to_datetime(&opt.start_time),
        str_to_datetime(&opt.end_time),
    )
}

fn create_app(candles_selection: CandlesSelection) -> Result<Application> {
    let selection = selection_factory(candles_selection);
    Ok(Application::new(
        create_repo()?,
        create_exchange()?,
        selection,
    ))
}

fn create_checker(candles_selection: CandlesSelection) -> Result<Checker> {
    let exchange = create_exchange()?;
    let repo = create_repo()?;
    let checker = Checker::new(candles_selection, repo, exchange);
    Ok(checker)
}

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let level = if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    utils::log_utils::setup_log(level, module_path!());

    #[cfg(debug_assertions)]
    {
        info!("Ativando backtrace");
        env::set_var("RUST_BACKTRACE", "1");
    };

    dotenv::dotenv()?;

    let candles_selection = candles_selection_from_arg(&args);

    let mut app = create_app(candles_selection.clone())?;

    match args.command {
        Command::Check {} => {
            let checker = create_checker(candles_selection)?;
            checker.check_inconsist();
        }
        Command::Sync {} => {
            let checker = create_checker(candles_selection)?;
            checker.synchronize()?;
        }
        Command::Fix {} => {
            let checker = create_checker(candles_selection)?;
            checker.delete_inconsist();
        }
        Command::DeleteAll {} => {
            let repo = create_repo()?;
            repo.delete_all_candles()?;
        }
        Command::List {} => {
            let repo = create_repo()?;
            repo.list_candles(&args.symbol, &args.minutes, &10);
        }
        Command::Plot {} => app.plot_selection()?,
        Command::Stream {} => {
            let mut streamer = Streamer::new(&mut app);
            streamer.run()?;
        }
        Command::Import {} => {}
        Command::Triangle {} => {
            app.plot_triangles()?;
        }
        Command::BackTest {} => app.run_back_test()?,
        Command::ScriptBackTest { file } => app.run_script_test(&file)?,
    };
    info!("Exiting program");
    Ok(())
}
