use std::convert::Infallible;
use std::time::Duration;

use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::StreamExt as _;
use futures_util::stream::Stream;
use tokio_stream::wrappers::IntervalStream;

/// `GET /sse/heartbeat` — placeholder SSE stream (ADR-008).
pub async fn sse_heartbeat() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(2)))
        .enumerate()
        .map(|(i, _)| {
            Ok(Event::default()
                .event("heartbeat")
                .data(format!("tick-{i}")))
        });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}
