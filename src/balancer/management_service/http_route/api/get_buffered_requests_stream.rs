use std::convert::Infallible;
use std::time::Duration;

use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::Responder;
use actix_web_lab::sse;
use log::error;

use crate::balancer::buffered_request_manager::BufferedRequestManager;
use crate::produces_snapshot::ProducesSnapshot as _;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/api/v1/buffered_requests/stream")]
async fn respond(
    buffered_request_manager: web::Data<BufferedRequestManager>,
) -> Result<impl Responder, Error> {
    let event_stream = async_stream::stream! {
        let send_event = |info| {
            match serde_json::to_string(&info) {
                Ok(json) => Some(Ok::<_, Infallible>(sse::Event::Data(sse::Data::new(json)))),
                Err(err) => {
                    error!("Failed to serialize buffered requests info: {err}");
                    None
                }
            }
        };

        if let Some(event) = send_event(buffered_request_manager.make_snapshot()) {
            yield event;
        }

        loop {
            tokio::select! {
                _ = buffered_request_manager.update_notifier.notified() => {
                    if let Some(event) = send_event(buffered_request_manager.make_snapshot()) {
                        yield event;
                    }
                },
            }
        }
    };

    Ok(sse::Sse::from_stream(event_stream).with_keep_alive(Duration::from_secs(10)))
}
