use crate::{
    candles_range::candles_to_ranges_missing,
    config::{candles_selection::CandlesSelection, symbol_minutes::SymbolMinutes},
    exchange::Exchange,
    model::{candle::Candle, open_close::OpenClose},
    repository::Repository,
    technicals::heikin_ashi,
};
use anyhow::anyhow;
use ifmt::iformat;
use log::debug;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

pub trait CandlesProvider {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>>;

    fn clone_provider(&self) -> Box<dyn CandlesProvider>;
}

pub struct CandlesProviderBufferSingleton {
    exchange: Exchange,
    repository: Repository,
    buffer: HashMap<SymbolMinutes, Vec<Candle>>,
}

impl CandlesProviderBufferSingleton {
    pub fn new(repository: Repository, exchange: Exchange) -> Self {
        Self {
            exchange,
            repository,
            buffer: HashMap::new(),
        }
    }

    fn candles(&mut self, candles_selection: CandlesSelection) -> anyhow::Result<Vec<Candle>> {
        let start = Instant::now();
        debug!("Initializing import...");

        fn candles_to_buf(heikin_ashi: bool, candles: &mut Vec<Candle>, buff: &mut Vec<Candle>) {
            if heikin_ashi {
                let candles = candles.iter().collect::<Vec<_>>();
                let candles_ref = candles.as_slice();
                let mut candles = heikin_ashi::heikin_ashi(candles_ref);
                buff.append(&mut candles);
            } else {
                buff.append(candles);
            }
            buff.sort();
        }

        // Normalize default start/end date time
        let start_time = &candles_selection.start_time;
        let end_time = &candles_selection.end_time;
        let minutes = &candles_selection.symbol_minutes.minutes;
        let symbol_minutes = &candles_selection.symbol_minutes;

        // Get candles from buffer
        debug!("Retrieving candles buffer {:?} {:?}...", start_time, end_time);
        let mut candles_buf = self.buffer.entry(symbol_minutes.clone()).or_default();
        debug!("Candles buffer count: {}", candles_buf.len());

        debug!("Retrieving ranges missing...");
        let ranges_missing = candles_to_ranges_missing(
            &OpenClose::from_date(start_time, minutes),
            &OpenClose::from_date(end_time, minutes),
            &candles_selection.symbol_minutes.minutes,
            candles_buf.iter().collect::<Vec<_>>().as_slice(),
        )?;
        debug!("Buffer ranges missing count: {}", ranges_missing.len());

        for range_missing in ranges_missing.iter() {
            let (start_time, end_time) = range_missing;

            // Get candles from repository
            debug!("Retrieving candles repository {:?} {:?}...", start_time, end_time);
            let mut candles_repo = self
                .repository
                .candles_by_time(&candles_selection.symbol_minutes, &start_time.open(minutes), &end_time.open(minutes))
                .unwrap_or_default();
            debug!("Candles repository count: {}", candles_repo.len());

            candles_to_buf(candles_selection.heikin_ashi, &mut candles_repo, &mut candles_buf);

            loop {
                // Get ranges missing
                debug!("Retrieving ranges missing...");
                let ranges_missing = candles_to_ranges_missing(
                    &start_time,
                    &end_time,
                    &candles_selection.symbol_minutes.minutes,
                    candles_repo.iter().collect::<Vec<_>>().as_slice(),
                )?;
                debug!("Repository ranges missing count: {}", ranges_missing.len());
                if ranges_missing.is_empty() {
                    break;
                }

                for range_missing in ranges_missing.iter() {
                    let (start_time, end_time) = range_missing;

                    debug!("Retrieving candles exchange {:?} {:?}...", start_time, end_time);
                    let mut candles_exch = self.exchange.candles(
                        &candles_selection.symbol_minutes,
                        &Some(start_time.open(minutes)),
                        &Some(end_time.open(minutes)),
                    )?;
                    debug!("Candles exchange count: {}", candles_exch.len());

                    // Save news candles on repository
                    self.repository.add_candles(&mut candles_exch)?;

                    // Insert candles on buffer
                    candles_to_buf(candles_selection.heikin_ashi, &mut candles_exch, &mut candles_buf);
                }
            }
        }

        let candles = candles_buf
            .par_iter()
            .filter(|c| &c.open_time >= start_time && &c.open_time <= end_time)
            .cloned()
            .collect::<Vec<_>>();

        debug!("{}", iformat!("Finished candles retrieve count: {candles.len()} elapsed: {start.elapsed():?}"));

        // candles_buf.iter().collect::<Vec<_>>()
        Ok(candles)
    }
}

#[derive(Clone)]
pub struct CandlesProviderBuffer {
    candles_provider_singleton: Rc<RefCell<CandlesProviderBufferSingleton>>,
    candles_selection_opt: Option<CandlesSelection>,
}

impl CandlesProviderBuffer {
    pub fn new(candles_provider_singleton: Rc<RefCell<CandlesProviderBufferSingleton>>) -> Self {
        Self {
            candles_provider_singleton,
            candles_selection_opt: None,
        }
    }
    pub fn set_candles_selection(&mut self, candles_selection: CandlesSelection) {
        self.candles_selection_opt = Some(candles_selection);
    }
}

impl CandlesProvider for CandlesProviderBuffer {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>> {
        let candles_selection = self
            .candles_selection_opt
            .as_ref()
            .cloned()
            .ok_or_else(|| -> anyhow::Error { anyhow!("candles_selection not definied!") })?;
        self.candles_provider_singleton.borrow_mut().candles(candles_selection)
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        todo!()
    }
}

pub struct CandlesProviderSelection {
    candles_provider: CandlesProviderBuffer,
    candles_selection: CandlesSelection,
}

impl<'a> CandlesProviderSelection {
    pub fn new(candles_provider: CandlesProviderBuffer, candles_selection: CandlesSelection) -> Self {
        Self {
            candles_provider,
            candles_selection,
        }
    }

    pub fn candles_selection(&self) -> CandlesSelection {
        self.candles_selection.clone()
    }
}

impl<'a> CandlesProvider for CandlesProviderSelection {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>> {
        // TODO HERE SHOULD FILTER
        self.candles_provider.set_candles_selection(self.candles_selection.clone());
        self.candles_provider.candles()
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        Box::new(Self::new(self.candles_provider.clone(), self.candles_selection.clone()))
    }
}

pub struct CandlesProviderVec {
    candles: Vec<Candle>,
}

impl<'a> CandlesProviderVec {
    pub fn new(candles: &'a [Candle], last_n: usize) -> Self {
        let start = (candles.len() - last_n).max(0);
        Self {
            candles: candles[start..candles.len()].to_vec(),
        }
    }
}

impl CandlesProvider for CandlesProviderVec {
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>> {
        Ok(self.candles.to_vec())
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        Box::new(Self::new(self.candles.as_ref(), 0))
    }
}

pub struct CandlesProviderClosure<F>
where
    F: FnMut() -> anyhow::Result<Vec<Candle>>,
{
    call_back: F,
}

impl<'a, F> CandlesProviderClosure<F>
where
    F: FnMut() -> anyhow::Result<Vec<Candle>>,
{
    pub fn new(call_back: F) -> Self
    where
        F: FnMut() -> anyhow::Result<Vec<Candle>>,
    {
        Self { call_back }
    }
}

impl<'a, F> CandlesProvider for CandlesProviderClosure<F>
where
    F: FnMut() -> anyhow::Result<Vec<Candle>>,
{
    fn candles(&mut self) -> anyhow::Result<Vec<Candle>> {
        (self.call_back)()
    }

    fn clone_provider(&self) -> Box<dyn CandlesProvider> {
        todo!()
    }
}
