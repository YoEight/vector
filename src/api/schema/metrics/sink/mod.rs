mod generic;

use super::{EventsInTotal, EventsOutTotal, ProcessedBytesTotal, ProcessedEventsTotal};
use crate::event::Metric;
use async_graphql::Interface;

#[derive(Debug, Clone, Interface)]
#[graphql(
    field(name = "processed_events_total", type = "Option<ProcessedEventsTotal>"),
    field(name = "processed_bytes_total", type = "Option<ProcessedBytesTotal>"),
    field(name = "events_in_total", type = "Option<EventsInTotal>"),
    field(name = "events_out_total", type = "Option<EventsOutTotal>")
)]
pub enum SinkMetrics {
    GenericSinkMetrics(generic::GenericSinkMetrics),
}

pub trait IntoSinkMetrics {
    fn into_sink_metrics(self, component_type: &str) -> SinkMetrics;
}

impl IntoSinkMetrics for Vec<Metric> {
    fn into_sink_metrics(self, _component_type: &str) -> SinkMetrics {
        SinkMetrics::GenericSinkMetrics(generic::GenericSinkMetrics::new(self))
    }
}
