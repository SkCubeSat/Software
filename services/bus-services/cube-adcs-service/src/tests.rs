use async_graphql::{EmptySubscription, Schema};
use cubespace_adcs_api::{
    MSG_TYPE_TC_ACK, MSG_TYPE_TC_EXT, MSG_TYPE_TLM_NACK, MSG_TYPE_TLM_RESP,
    MSG_TYPE_TLM_RESP_EXT, build_can_id,
};
use rust_can::mock::MockStream;
use rust_can::{CanFrame, Connection};
use std::time::Duration;

use crate::schema::{MutationRoot, QueryRoot};
use crate::subsystem::{AdcsServiceConfig, CubeAdcsError, Subsystem};

fn test_config() -> AdcsServiceConfig {
    AdcsServiceConfig {
        interface: "vcan0".to_string(),
        bitrate: 1_000_000,
        source_address: 1,
        destination_address: 4,
        timeout: Duration::from_millis(10),
        bring_interface_up: false,
    }
}

#[test]
fn schema_exposes_generated_adcs_fields() {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .finish();
    let sdl = schema.sdl();

    assert!(sdl.contains("fssCubesenseSunRaw"));
    assert!(sdl.contains("hil"));
    assert!(sdl.contains("controlMode"));
    assert!(sdl.contains("currentUnixTime"));
    assert!(sdl.contains("adcsOperationalState"));
}

#[test]
fn schema_exposes_service_control_fields() {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .finish();
    let sdl = schema.sdl();

    assert!(sdl.contains("health"));
    assert!(sdl.contains("setInterfaceUp"));
    assert!(sdl.contains("resetInterface"));
    assert!(sdl.contains("sendCommandRaw"));
}

#[test]
fn long_command_is_split_into_extended_telecommand_frames() {
    let mut mock = MockStream::default();
    let tx_id = build_can_id(MSG_TYPE_TC_EXT, 54, 1, 4);
    mock.write
        .set_input(CanFrame::extended(tx_id, &[0, 1, 2, 3, 4, 5, 6, 7]));
    mock.write
        .set_input(CanFrame::extended(tx_id, &[8, 9, 10, 11]));
    mock.read.set_output(vec![CanFrame::extended(
        build_can_id(MSG_TYPE_TC_ACK, 54, 4, 1),
        &[],
    )]);
    let subsystem = Subsystem::with_connection(test_config(), Connection::new(Box::new(mock)));

    let ack = subsystem
        .send_command_payload(54, &(0_u8..12).collect::<Vec<_>>())
        .unwrap();
    assert!(ack.acknowledged);
}

#[test]
fn short_telemetry_accepts_normal_response_type() {
    let mut mock = MockStream::default();
    mock.write.set_input(CanFrame::extended(
        build_can_id(cubespace_adcs_api::MSG_TYPE_TLM_REQ, 133, 1, 4),
        &[],
    ));
    mock.read.set_output(vec![CanFrame::extended(
        build_can_id(MSG_TYPE_TLM_RESP, 133, 4, 1),
        &[1, 2, 3, 4, 5, 6, 7, 8],
    )]);
    let subsystem = Subsystem::with_connection(test_config(), Connection::new(Box::new(mock)));

    assert_eq!(
        subsystem.request_telemetry_payload(133, 8).unwrap(),
        vec![1, 2, 3, 4, 5, 6, 7, 8]
    );
}

#[test]
fn extended_telemetry_reassembles_matching_frames() {
    let mut mock = MockStream::default();
    mock.write.set_input(CanFrame::extended(
        build_can_id(cubespace_adcs_api::MSG_TYPE_TLM_REQ, 170, 1, 4),
        &[],
    ));
    let rx_id = build_can_id(MSG_TYPE_TLM_RESP_EXT, 170, 4, 1);
    mock.read.set_output(vec![
        CanFrame::extended(rx_id, &[0, 1, 2, 3, 4, 5, 6, 7]),
        CanFrame::extended(rx_id, &[8, 9, 10, 11, 12, 13, 14, 15]),
        CanFrame::extended(rx_id, &[16, 17, 18, 19, 20, 21, 22, 23]),
        CanFrame::extended(rx_id, &[24, 25, 26, 27, 28, 29, 30, 31]),
        CanFrame::extended(rx_id, &[32]),
    ]);
    let subsystem = Subsystem::with_connection(test_config(), Connection::new(Box::new(mock)));

    assert_eq!(
        subsystem.request_telemetry_payload(170, 33).unwrap(),
        (0_u8..33).collect::<Vec<_>>()
    );
}

#[test]
fn telemetry_nack_is_reported_without_timing_out() {
    let mut mock = MockStream::default();
    mock.write.set_input(CanFrame::extended(
        build_can_id(cubespace_adcs_api::MSG_TYPE_TLM_REQ, 133, 1, 4),
        &[],
    ));
    mock.read.set_output(vec![CanFrame::extended(
        build_can_id(MSG_TYPE_TLM_NACK, 133, 4, 1),
        &[3],
    )]);
    let subsystem = Subsystem::with_connection(test_config(), Connection::new(Box::new(mock)));

    assert!(matches!(
        subsystem.request_telemetry_payload(133, 8),
        Err(CubeAdcsError::TelemetryNack {
            telemetry_id: 133,
            error_code: Some(3)
        })
    ));
}
