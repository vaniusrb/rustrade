pub trait Indicator {
    fn value(&self) -> eyre::Result<f64>;

    fn min_max(&self) -> (f64, f64);
}
