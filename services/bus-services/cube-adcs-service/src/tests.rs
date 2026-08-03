use async_graphql::{EmptySubscription, Schema};
use cubespace_adcs_api::{
    MSG_TYPE_TC_ACK, MSG_TYPE_TC_EXT, MSG_TYPE_TLM_NACK, MSG_TYPE_TLM_RESP, MSG_TYPE_TLM_RESP_EXT,
    build_can_id,
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
    assert!(sdl.contains("telemetryRaw"));
    assert!(sdl.contains("payloadHex"));
    assert!(sdl.contains("expectedLengthBytes"));
    assert!(sdl.contains("receivedLengthBytes"));
    assert!(sdl.contains("setInterfaceUp"));
    assert!(sdl.contains("resetInterface"));
    assert!(sdl.contains("sendCommandRaw"));
}

#[test]
fn schema_exposes_all_additional_command_fields() {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .finish();
    let sdl = schema.sdl();

    for field in [
        "referenceRotationAngle",
        "disableMagneticRwlMomentumManagement",
        "referenceParametersForFmcScan",
        "referenceIrcVector",
        "referenceLlhTargetCommand",
        "targetSatelliteOrbitParameterCommand",
        "simulationRawSensorTelemetry",
        "transferFrame",
        "fileTransferSetup",
        "passThroughTctlm",
        "portMap",
        "unsolicitedTelemetrySetup",
        "setRequestImageLogTransferSetup",
        "setADummyEvent",
        "unsolicitedEventMessageSetup",
        "formatAllLogs",
    ] {
        assert!(sdl.contains(field), "missing GraphQL mutation {field}");
    }
}

#[test]
fn long_command_is_split_into_extended_telecommand_frames() {
    let mut mock = MockStream::default();
    let tx_id = build_can_id(MSG_TYPE_TC_EXT, 54, 1, 4);
    mock.write
        .set_input(CanFrame::extended(tx_id, &[0, 1, 2, 3, 4, 5, 6, 1]));
    mock.write
        .set_input(CanFrame::extended(tx_id, &[7, 8, 9, 10, 11, 0]));
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
        CanFrame::extended(rx_id, &[0, 1, 2, 3, 4, 5, 6, 4]),
        CanFrame::extended(rx_id, &[7, 8, 9, 10, 11, 12, 13, 3]),
        CanFrame::extended(rx_id, &[14, 15, 16, 17, 18, 19, 20, 2]),
        CanFrame::extended(rx_id, &[21, 22, 23, 24, 25, 26, 27, 1]),
        CanFrame::extended(rx_id, &[28, 29, 30, 31, 32, 0]),
    ]);
    let subsystem = Subsystem::with_connection(test_config(), Connection::new(Box::new(mock)));

    assert_eq!(
        subsystem.request_telemetry_payload(170, 33).unwrap(),
        (0_u8..33).collect::<Vec<_>>()
    );
}

#[test]
fn extended_telemetry_rejects_out_of_sequence_frames() {
    let mut mock = MockStream::default();
    mock.write.set_input(CanFrame::extended(
        build_can_id(cubespace_adcs_api::MSG_TYPE_TLM_REQ, 170, 1, 4),
        &[],
    ));
    let rx_id = build_can_id(MSG_TYPE_TLM_RESP_EXT, 170, 4, 1);
    mock.read.set_output(vec![
        CanFrame::extended(rx_id, &[0, 1, 2, 3, 4, 5, 6, 4]),
        CanFrame::extended(rx_id, &[7, 8, 9, 10, 11, 12, 13, 2]),
    ]);
    let subsystem = Subsystem::with_connection(test_config(), Connection::new(Box::new(mock)));

    assert!(matches!(
        subsystem.request_telemetry_payload(170, 33),
        Err(CubeAdcsError::Can(message)) if message.contains("out of sequence")
    ));
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
