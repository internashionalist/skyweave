use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, RwLock};
use tracing::Instrument;
use uuid::Uuid;

/// 3D position in meters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
/// orientation in radians
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orientation {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

/// state of a single UAV sent to WebSocket clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UavState {
    pub id: Uuid,
    pub callsign: String,
    pub position: Position,
    pub orientation: Orientation,
    pub velocity_mps: f64,
    pub last_update: DateTime<Utc>,
}

/// payload sent from the C++ sim over UDP - 
/// serialized as JSON from telemetry_encoder.cpp
#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryFrame {
    pub id: Uuid,
    pub callsign: String,
    pub position: Position,
    pub orientation: Orientation,
    pub velocity_mps: f64,
}
/// state of UAV swarm
#[derive(Clone, Default)]
pub struct SwarmState {
    inner: Arc<RwLock<HashMap<Uuid, UavState>>>,
}

impl SwarmState {
    pub fn new() -> Self {
        SwarmState {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn upsert_uav(&self, state: UavState) {
        let mut map = self.inner.write().await;
        map.insert(state.id, state);
    }

    pub async fn get_uav(&self, id: Uuid) -> Option<UavState> {
        let map = self.inner.read().await;
        map.get(&id).cloned()
    }

    pub async fn list_uavs(&self) -> Vec<UavState> {
        let map = self.inner.read().await;
        map.values().cloned().collect()
    }
}

/// shared handles used by both UDP listener and WebSocket server
#[derive(Clone)]
pub struct TelemetryShared {
    pub swarm: SwarmState,
    pub tx: broadcast::Sender<UavState>,
}

/// start UDP listener that receives telemetry frames, updates swarm state,
/// and broadcasts each update to WebSocket clients
pub async fn run_udp_listener(bind_addr: SocketAddr, shared: TelemetryShared) {
    let socket = match UdpSocket::bind(bind_addr).await {
        Ok(s) => s,
        Err(err) => {
            tracing::error!("Failed to bind UDP socket at {}: {}", bind_addr, err);
            return;
        }
    };

    tracing::info!("UDP telemetry listener bound at {}", bind_addr);

    loop {
        let mut buf = vec![0u8; 2048];
        let span = tracing::info_span!("udp_recv");
        let shared = shared.clone();
        let socket = &socket;
        let addr = bind_addr;

        let fut = async move {
            let (len, src) = match socket.recv_from(&mut buf).await {
                Ok(res) => res,
                Err(err) => {
                    tracing::warn!("Error receiving UDP packet: {}", err);
                    return;
                }
            };

            let data = &buf[..len];
            tracing::debug!("Received {} bytes from {}", len, src);

            match decode_frame(data) {
                Ok(uav) => {
                    let id = uav.id;
                    shared.swarm.upsert_uav(uav.clone()).await;

                    // Broadcast the update; ignore if there are no listeners.
                    if let Err(e) = shared.tx.send(uav) {
                        tracing::debug!(
                            "No WebSocket subscribers for update {}: {}",
                            id,
                            e
                        );
                    }
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to decode telemetry frame from {}: {}",
                        src,
                        err
                    );
                }
            }
        };

        fut.instrument(span).await;
        // loop continues
        tracing::trace!("Waiting for next UDP telemetry packet on {}", addr);
    }
}

/// decode a UDP datagram into a `UavState`.
///
/// (for now) this expects a single JSON object matching `TelemetryFrame`
fn decode_frame(data: &[u8]) -> Result<UavState, serde_json::Error> {
    let frame: TelemetryFrame = serde_json::from_slice(data)?;
    Ok(UavState {
        id: frame.id,
        callsign: frame.callsign,
        position: frame.position,
        orientation: frame.orientation,
        velocity_mps: frame.velocity_mps,
        last_update: Utc::now(),
    })
}
