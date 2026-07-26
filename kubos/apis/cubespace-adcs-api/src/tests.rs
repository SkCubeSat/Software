use crate::*;

#[test]
fn generated_matrix_specs_are_available() {
    assert_eq!(COMMAND_SPECS.len(), 33);
    assert_eq!(TELEMETRY_SPECS.len(), 102);
    assert_eq!(command_spec(58).unwrap().name, "Control Mode");
    assert_eq!(telemetry_spec(170).unwrap().name, "FSS CubeSense Sun Raw");
}

#[test]
fn can_id_round_trip_matches_adcs_scripts() {
    let can_id = build_can_id(MSG_TYPE_TC, 58, 1, 4);

    assert_eq!(can_id, 0x013A_0104);
    assert_eq!(
        decode_can_id(can_id),
        CanIdFields {
            msg_type: MSG_TYPE_TC,
            tctlm_id: 58,
            src_addr: 1,
            dst_addr: 4,
        }
    );
}

#[test]
fn encode_control_mode_payload() {
    let command = ControlModeCommand {
        control_mode: 0,
        control_timeout: 0,
    };

    assert_eq!(command.encode().unwrap(), vec![0, 0, 0]);
}

#[test]
fn encode_current_unix_time_payload() {
    let command = CurrentUnixTimeCommand {
        current_unix_seconds: 1_700_000_000,
        current_unix_nanoseconds: 123_456_789,
    };
    let payload = command.encode().unwrap();

    assert_eq!(payload.len(), 8);
    assert_eq!(&payload[0..4], &1_700_000_000_u32.to_le_bytes());
    assert_eq!(&payload[4..8], &123_456_789_u32.to_le_bytes());
}

#[test]
fn decode_raw_cubesense_sun_payload() {
    let mut payload = vec![0; 33];
    payload[0..4].copy_from_slice(&10_u32.to_le_bytes());
    payload[4..8].copy_from_slice(&20_u32.to_le_bytes());
    payload[8..10].copy_from_slice(&100_i16.to_le_bytes());
    payload[10..12].copy_from_slice(&(-200_i16).to_le_bytes());
    payload[12] = 1;
    payload[13] = 0;
    payload[32] = 0b0000_1010;

    let decoded = FssCubesenseSunRawTelemetry::decode(&payload).unwrap();

    assert_eq!(decoded.time_integer_seconds, 10);
    assert_eq!(decoded.fss0_alpha_angle_raw, 100);
    assert_eq!(decoded.fss0_alpha_angle, 1.0);
    assert_eq!(decoded.fss0_beta_angle_raw, -200);
    assert_eq!(decoded.fss0_beta_angle, -2.0);
    assert_eq!(decoded.fss0_capture_result, Some("Captured".to_string()));
    assert_eq!(
        decoded.fss0_detection_result,
        Some("NoDetection".to_string())
    );
    assert!(!decoded.fss0_valid_flag);
    assert!(decoded.fss1_valid_flag);
    assert!(!decoded.fss2_valid_flag);
    assert!(decoded.fss3_valid_flag);
}

#[test]
fn decode_hil_enum_labels() {
    let mut payload = vec![0; 107];
    payload[0] = 12;
    payload[1] = 5;
    payload[2] = 0b0000_1110;

    let decoded = HilTelemetry::decode(&payload).unwrap();

    assert_eq!(decoded.active_control_mode, Some("ConXYZWheel".to_string()));
    assert_eq!(
        decoded.active_estimator_mode,
        Some("EstFullEkf".to_string())
    );
    assert_eq!(decoded.active_orbit_mode, Some("OrbAsgp4".to_string()));
    assert_eq!(
        decoded.source_of_current_orbit_pos_and_vel,
        Some("NavAsgp4Tle".to_string())
    );
    assert_eq!(
        decoded.active_operational_state,
        Some("OpStateManual".to_string())
    );
}
