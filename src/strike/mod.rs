use crate::measure::trace_sample::TraceSample;

#[derive(Debug, Clone, Default)]
pub struct StrikePath {
    pub point_samples: Vec<TraceSample>,
}


pub struct Strike<R> {
    pub path: StrikePath,
    pub render: R
}