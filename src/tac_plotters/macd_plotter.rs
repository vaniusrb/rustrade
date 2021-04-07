use super::plotter_indicator_area::PlotterIndicatorArea;
use crate::service::technicals::technical::TechnicalIndicators;
use crate::service::technicals::{indicator::Indicator, macd_tac::MacdTac};
use plotters::prelude::*;

pub struct MacdPlotter<'a> {
    macd_tac: &'a MacdTac,
}

impl<'a> MacdPlotter<'a> {
    pub fn new(macd_tac: &'a MacdTac) -> Self {
        MacdPlotter { macd_tac }
    }
}

impl<'a> PlotterIndicatorArea for MacdPlotter<'a> {
    fn indicator_color(&self, indicator: &Indicator) -> RGBColor {
        match &indicator.name[..] {
            "macd" => RGBColor(0, 0, 255),
            "signal" => RGBColor(255, 0, 0),
            _ => BLACK,
        }
    }

    fn technical_indicators(&self) -> &dyn TechnicalIndicators {
        self.macd_tac
    }
}
