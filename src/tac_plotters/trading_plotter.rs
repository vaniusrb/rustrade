use super::indicator_plotter::PlotterIndicatorContext;
use crate::{
    config::selection::Selection,
    strategy::{
        trader_register::TradeOperation,
        trend_enum::{Operation, Side},
    },
};
use chrono::{DateTime, Utc};
use plotters::{
    coord::types::RangedCoordf32,
    prelude::{Cartesian2d, ChartContext, RangedDateTime, TriangleMarker},
    style::RGBColor,
};
use plotters_bitmap::{bitmap_pixel::RGBPixel, BitMapBackend};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

pub struct TradingPlotter<'a> {
    trades: &'a [TradeOperation],
}

impl<'a> TradingPlotter<'a> {
    pub fn new(trades: &'a [TradeOperation]) -> Self {
        TradingPlotter { trades }
    }
}

impl<'a> PlotterIndicatorContext for TradingPlotter<'a> {
    fn plot(
        &self,
        _selection: &Selection,
        chart_context: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>,
        >,
    ) -> eyre::Result<()> {
        let red = RGBColor(164, 0, 16);
        let green = RGBColor(16, 128, 32);

        let trades = &self.trades;

        let sell_iter = trades
            .iter()
            .filter(|p| p.operation.to_side() == Side::Sold)
            .map(|c| (c.now, c.price.0.to_f32().unwrap()))
            .collect::<Vec<_>>();

        let buy_iter = trades
            .iter()
            .filter(|p| p.operation.to_side() == Side::Bought)
            .map(|c| (c.now, c.price.0.to_f32().unwrap()))
            .collect::<Vec<_>>();

        chart_context.draw_series(sell_iter.iter().map(|point| TriangleMarker::new(*point, 10, &green)))?;
        chart_context.draw_series(buy_iter.iter().map(|point| TriangleMarker::new(*point, 10, &red)))?;

        // let lows = PointSeries::of_element(
        //     sell_iter.into_iter(),
        //     6,                      // 3
        //     ShapeStyle::from(&red), /* .filled() */
        //     &|coord, size, style| {
        //         EmptyElement::at(coord) + Circle::new((0, 0), size * 3, style)
        //         //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        //     },
        // );
        // chart_context.draw_series(lows)?;

        // let tops = PointSeries::of_element(
        //     trades
        //         .iter()
        //         .filter(|p| p.operation == Operation::Buy)
        //         .map(|c| (c.now, c.price.to_f32().unwrap())),
        //     6,                        // 3
        //     ShapeStyle::from(&green), /*.filled()*/
        //     &|coord, size, style| {
        //         EmptyElement::at(coord) + Circle::new((0, 0), size * 3, style)
        //         //+ Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        //     },
        // );
        // chart_context.draw_series(tops)?;

        Ok(())
    }

    fn min_max(&self) -> (f64, f64) {
        let max = self.trades.iter().fold(dec!(0), |acc, t| acc.max(t.price.0));
        let min = self.trades.iter().fold(max, |acc, t| acc.min(t.price.0));
        (min.to_f64().unwrap(), max.to_f64().unwrap())
    }
}
