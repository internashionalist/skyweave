use crate::telemetry::{TelemetryShared, UavState};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
        Path,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::Instrument;

/// build the Axum router that serves:
/// - GET /health   : simple health check
/// - GET /uavs     : snapshot of current swarm state
/// - GET /ws       : WebSocket streaming telemetry updates
pub fn router(shared: TelemetryShared) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/uavs", get(list_uavs))
        .route("/uavs/:id", get(get_uav))
        .route("/ws", get(ws_handler))
        .with_state(shared)
}

async fn health() -> &'static str {
    "ok"
}

async fn list_uavs(State(shared): State<TelemetryShared>) -> Json<Vec<UavState>> {
    let list = shared.swarm.list_uavs().await;
    Json(list)
}

async fn get_uav(
    State(shared): State<TelemetryShared>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let parsed = match uuid::Uuid::parse_str(&id) {
        Ok(v) => v,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid UUID").into_response(),
    };

    match shared.swarm.get_uav(parsed).await {
        Some(uav) => Json(uav).into_response(),
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

async fn ws_handler(
    State(shared): State<TelemetryShared>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, shared))
}

/// handle a single WebSocket client connection
async fn handle_ws(socket: WebSocket, shared: TelemetryShared) {
    let span = tracing::info_span!("ws_client");
    async move {
        let (mut sender, mut _receiver) = socket.split();

        // send a snapshot of all UAVs on connect
        if let Ok(initial) = serde_json::to_string(&shared.swarm.list_uavs().await) {
            let _ = sender.send(Message::Text(initial)).await;
        }

        // subscribe to telemetry updates
        let mut rx = shared.tx.subscribe();

        loop {
            match rx.recv().await {
                Ok(update) => {
                    let msg = match serde_json::to_string(&update) {
                        Ok(m) => m,
                        Err(err) => {
                            tracing::warn!("Failed to serialize update: {}", err);
                            continue;
                        }
                    };

                    if sender.send(Message::Text(msg)).await.is_err() {
                        tracing::info!("WebSocket client disconnected");
                        break;
                    }
                }
                Err(err) => {
                    tracing::warn!("WebSocket broadcast recv error: {}", err);
                    break;
                }
            }
        }
    }
    .instrument(span)
    .await;
}
