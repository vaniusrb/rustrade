use super::plotter_indicator_area::PlotterIndicatorArea;
use crate::service::technicals::technical::TechnicalIndicators;
use crate::service::technicals::{indicator::Indicator, rsi_tac::RsiTac};
use plotters::prelude::*;
use plotters::style::BLACK;

pub struct RsiPlotter<'a> {
    rsi_tac: &'a RsiTac,
}

impl<'a> RsiPlotter<'a> {
    pub fn new(rsi_tac: &'a RsiTac) -> Self {
        RsiPlotter { rsi_tac }
    }
}

impl<'a> PlotterIndicatorArea for RsiPlotter<'a> {
    fn technical_indicators(&self) -> Box<dyn TechnicalIndicators> {
        Box::new(self.rsi_tac.clone()) as Box<dyn TechnicalIndicators>
    }

    fn indicator_color(&self, indicator: &Indicator) -> RGBColor {
        match &indicator.name[..] {
            "rsi" => RGBColor(0, 0, 255),
            _ => BLACK,
        }
    }
}
