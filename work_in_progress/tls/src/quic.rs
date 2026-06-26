use heapless::Vec;

use crate::{crypto::MAX_HASH_OUTPUT, message::Extension};

pub fn transport_parameters_extension(params: &[u8]) -> Extension {
    crate::message::ext_quic_transport_parameters(params)
}

pub struct QuicSecrets {
    pub client_early_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_handshake_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub server_handshake_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_application_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub server_application_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub exporter_master_secret: Vec<u8, MAX_HASH_OUTPUT>,
}

pub fn extract_quic_secrets(
    key_schedule: &crate::key_schedule::KeySchedule,
    server_hello_transcript: &[u8],
    server_finished_transcript: &[u8],
) -> QuicSecrets {
    let c_hs = key_schedule.client_handshake_traffic_secret(server_hello_transcript);
    let s_hs = key_schedule.server_handshake_traffic_secret(server_hello_transcript);
    let c_ap = key_schedule.client_application_traffic_secret(server_finished_transcript);
    let s_ap = key_schedule.server_application_traffic_secret(server_finished_transcript);
    let exp = key_schedule.exporter_master_secret(server_finished_transcript);

    let mut client_early = Vec::new();
    client_early.extend_from_slice(key_schedule.early_secret()).unwrap();

    QuicSecrets {
        client_early_traffic_secret: client_early,
        client_handshake_traffic_secret: c_hs,
        server_handshake_traffic_secret: s_hs,
        client_application_traffic_secret: c_ap,
        server_application_traffic_secret: s_ap,
        exporter_master_secret: exp,
    }
}
