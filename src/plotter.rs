//use bigdecimal::ToPrimitive;
use chrono::Duration;
use ifmt::iformat;
use plotters::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

use crate::{model::candle::Candle, utils::str_to_datetime};

pub fn plot(symbol: &str, data: &[Candle]) -> Result<(), Box<dyn std::error::Error>> {
    //let data = get_data();
    let root = BitMapBackend::new("plotters-doc-data/stock.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_price = data.iter().fold(dec!(0), |acc, x| acc.max(x.high));
    let min_price = data.iter().fold(max_price, |acc, x| acc.min(x.low));

    let min_price = min_price.to_f32().unwrap();
    let max_price = max_price.to_f32().unwrap();

    let from_date = str_to_datetime(&data[0].close_time) - Duration::minutes(15); /* +  */
    let to_date = str_to_datetime(&data[data.len() - 1].close_time) + Duration::minutes(15); /* */

    let mut chart_candles = ChartBuilder::on(&root)
        .x_label_area_size(80)
        .y_label_area_size(80)
        .caption(iformat!("{symbol} price"), ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, min_price..max_price)?;

    chart_candles
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()?;

    let series = data.iter().map(|x| {
        CandleStick::new(
            str_to_datetime(&x.close_time),
            x.open.to_f32().unwrap(),
            x.high.to_f32().unwrap(),
            x.low.to_f32().unwrap(),
            x.close.to_f32().unwrap(),
            &GREEN,
            &RED,
            12,
        )
    });

    let candle_stick = chart_candles.draw_series(series)?;

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Line Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..10, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((-10..=10).map(|x| (x, x * x)), &GREEN))
        .unwrap();

    Ok(())
}
