#![feature(nll)]
#![feature(associated_type_bounds)]
#[macro_use]
extern crate enum_display_derive;
pub mod app;
pub mod config;
pub mod model;
pub mod repository;
pub mod services;
pub mod utils;
use crate::app::Application;
use crate::repository::candle_repository::CandleRepository;
use crate::repository::pool_factory::pool_factory;
use crate::repository::symbol_repository::SymbolRepository;
use crate::services::candles_checker::CandlesChecker;
use crate::services::provider::candles_utils::str_to_datetime;
use crate::services::streamer::Streamer;
use crate::services::technicals::ema_tac::EmaTac;
use crate::services::technicals::macd_tac::MacdTac;
use config::{candles_selection::CandlesSelection, selection::Selection};
use eyre::Result;
use log::{info, Level, LevelFilter};
use services::{
    exchange::Exchange,
    technicals::{rsi_tac::RsiTac, technical::TechnicalDefinition},
};
use sqlx::PgPool;
#[cfg(debug_assertions)]
use std::env;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use structopt::StructOpt;

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
    command: Commands,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Commands")]
enum Commands {
    /// Candle commands
    Candle(Candle),
    /// Plot graph
    Plot {},
    /// Triangle
    Triangle {},
    /// Interactive stream
    Stream {},
    /// Run script trader bot back test
    ScriptBackTest {
        /// Rhai script file
        #[structopt(short, long)]
        file: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Commands")]
enum Candle {
    List {},
    /// Fix records
    Fix {},
    /// Delete all candles'
    DeleteAll,
    /// Import from exchange
    Import {},
    /// Check content
    Check {},
    /// Synchronize
    Sync {},
}

pub fn selection_default(candles_selection: CandlesSelection) -> Selection {
    let mut tacs = HashMap::new();
    for tac in vec![
        RsiTac::definition(),
        MacdTac::definition(),
        EmaTac::definition(),
    ] {
        tacs.insert(tac.name.clone(), tac);
    }
    Selection {
        tacs,
        candles_selection,
        image_name: "out/stock.png".to_string(),
    }
}

fn create_repository_candle(pool: Arc<RwLock<PgPool>>) -> CandleRepository {
    CandleRepository::new(pool)
}

fn create_exchange(repository_symbol: SymbolRepository) -> Result<Exchange> {
    Exchange::new(repository_symbol, Level::Debug)
}

fn candles_selection_from_arg(repository_symbol: SymbolRepository, opt: &Args) -> CandlesSelection {
    let symbol = repository_symbol.symbol_by_pair(&opt.symbol).unwrap().id;
    CandlesSelection::from(
        symbol,
        opt.minutes as i32,
        str_to_datetime(&opt.start_time),
        str_to_datetime(&opt.end_time),
    )
}

fn create_app(
    pool: Arc<RwLock<PgPool>>,
    repository_symbol: SymbolRepository,
    candles_selection: CandlesSelection,
) -> Result<Application> {
    let selection = selection_default(candles_selection);
    Ok(Application::new(
        create_repository_candle(pool),
        create_exchange(repository_symbol)?,
        selection,
    ))
}

fn create_checker(
    pool: Arc<RwLock<PgPool>>,
    repository_symbol: SymbolRepository,
    candles_selection: CandlesSelection,
) -> Result<CandlesChecker> {
    let exchange = create_exchange(repository_symbol)?;
    let repository = create_repository_candle(pool.clone());
    let checker = CandlesChecker::new(pool, candles_selection, repository, exchange);
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

    let pool = pool_factory(LevelFilter::Debug)?;
    let repository_symbol = SymbolRepository::new(pool.clone());

    let candles_selection = candles_selection_from_arg(repository_symbol.clone(), &args);

    let mut app = create_app(pool.clone(), repository_symbol.clone(), candles_selection)?;

    match args.command {
        Commands::Candle(candle) => match candle {
            Candle::List {} => {
                let repo = create_repository_candle(pool);
                let symbol = repository_symbol.symbol_by_pair(&args.symbol).unwrap().id;
                repo.list_candles(symbol, args.minutes as i32, 10);
            }
            Candle::Fix {} => {
                let checker = create_checker(pool, repository_symbol, candles_selection)?;
                checker.delete_inconsist();
            }
            Candle::DeleteAll => {
                let candle_repostiory = create_repository_candle(pool);
                candle_repostiory.delete_all_candles()?;
            }
            Candle::Import {} => {}
            Candle::Check {} => {
                let checker = create_checker(pool, repository_symbol, candles_selection)?;
                checker.check_inconsist();
            }
            Candle::Sync {} => {
                let checker = create_checker(pool, repository_symbol, candles_selection)?;
                checker.synchronize()?;
            }
        },

        Commands::Plot {} => app.plot_selection()?,
        Commands::Stream {} => {
            let mut streamer = Streamer::new(&mut app);
            streamer.run()?;
        }
        Commands::Triangle {} => {
            app.plot_triangles()?;
        }
        Commands::ScriptBackTest { file } => app.run_script_test(pool, &file)?,
    };
    info!("Exiting program");
    Ok(())
}
