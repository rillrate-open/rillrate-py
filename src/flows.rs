use pyo3::prelude::*;
use pyo3::types::PyDict;
use rill_protocol::flow::core::FlowMode;
use rillrate as rr;
use rr::table::{Col, Row};

pub fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Board>()?;
    m.add_class::<Counter>()?;
    m.add_class::<Gauge>()?;
    m.add_class::<Histogram>()?;
    m.add_class::<Pulse>()?;
    m.add_class::<Table>()?;
    Ok(())
}

#[pyclass]
pub struct Counter {
    tracer: rr::Counter,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(path: String) -> Self {
        let spec = rr::CounterSpec;
        let tracer = rr::Counter::new(path, FlowMode::Realtime, spec);
        Self { tracer }
    }

    fn inc(&mut self, delta: i64) {
        self.tracer.inc(delta);
    }
}

fn get_from<'a, T>(kwargs: Option<&'a PyDict>, name: &'a str) -> PyResult<T>
where
    T: FromPyObject<'a> + Default,
{
    if let Some(dict) = kwargs {
        if let Some(value) = dict.get_item(name) {
            return value.extract();
        }
    }
    Ok(T::default())
}

#[pyclass]
pub struct Gauge {
    tracer: rr::Gauge,
}

#[pymethods]
impl Gauge {
    #[new]
    #[args(kwargs = "**")]
    fn new(path: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let opts = rr::GaugeOpts {
            min: get_from(kwargs, "min")?,
            lower: get_from(kwargs, "lower")?,
            max: get_from(kwargs, "max")?,
            higher: get_from(kwargs, "higher")?,
        };
        let tracer = rr::Gauge::new(path, FlowMode::Realtime, opts);
        Ok(Self { tracer })
    }

    fn set(&mut self, value: f64) {
        self.tracer.set(value);
    }
}

#[pyclass]
pub struct Pulse {
    tracer: rr::Pulse,
}

#[pymethods]
impl Pulse {
    #[new]
    fn new(path: String) -> Self {
        let spec = rr::PulseSpec::default();
        let tracer = rr::Pulse::new(path, FlowMode::Realtime, spec);
        Self { tracer }
    }

    fn push(&mut self, value: f64) {
        self.tracer.push(value);
    }
}

#[pyclass]
pub struct Histogram {
    tracer: rr::Histogram,
}

#[pymethods]
impl Histogram {
    #[new]
    fn new(path: String, levels: Vec<f64>) -> Self {
        let spec = rr::HistogramSpec { levels };
        let tracer = rr::Histogram::new(path, FlowMode::Realtime, spec);
        Self { tracer }
    }

    fn add(&mut self, value: f64) {
        self.tracer.add(value);
    }
}

#[pyclass]
pub struct Board {
    tracer: rr::Board,
}

#[pymethods]
impl Board {
    #[new]
    fn new(path: String) -> Self {
        let spec = rr::BoardSpec;
        let tracer = rr::Board::new(path, FlowMode::Realtime, spec);
        Self { tracer }
    }

    fn set(&mut self, key: String, value: String) {
        self.tracer.set(key, value);
    }
}

#[pyclass]
pub struct Table {
    tracer: rr::Table,
}

#[pymethods]
impl Table {
    #[new]
    fn new(path: String, columns: Vec<(u64, String)>) -> Self {
        let columns = columns
            .into_iter()
            .map(|(col, title)| (Col(col), title))
            .collect();
        let spec = rr::TableSpec { columns };
        let tracer = rr::Table::new(path, FlowMode::Realtime, spec);
        Self { tracer }
    }

    fn add_row(&mut self, row: u64) {
        self.tracer.add_row(Row(row));
    }

    fn del_row(&mut self, row: u64) {
        self.tracer.del_row(Row(row));
    }

    fn set_cell(&mut self, row: u64, col: u64, value: String) {
        self.tracer.set_cell(Row(row), Col(col), value);
    }
}

/*
#[pyclass]
pub struct Logger {
    tracer: rr::Logger,
}

#[pymethods]
impl Logger {
    #[new]
    fn new(path: String) -> Self {
        let tracer = rr::Logger::new(&path).unwrap();
        Self { tracer }
    }

    fn is_active(&mut self) -> bool {
        self.tracer.is_active()
    }

    fn log(&mut self, msg: String) {
        self.tracer.log(msg);
    }
}
*/
