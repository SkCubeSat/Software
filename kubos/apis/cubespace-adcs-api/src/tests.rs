use crate::*;

#[test]
fn generated_matrix_specs_are_available() {
    assert_eq!(COMMAND_SPECS.len(), 33);
    assert_eq!(ADDITIONAL_COMMAND_SPECS.len(), 16);
    assert_eq!(TELEMETRY_SPECS.len(), 102);
    assert_eq!(command_spec(58).unwrap().name, "Control Mode");
    assert_eq!(telemetry_spec(170).unwrap().name, "FSS CubeSense Sun Raw");
}

#[test]
fn mission_selected_raw_command_specs_are_available() {
    let port_map = command_spec(111).unwrap();
    assert_eq!(port_map.length_bytes, 120);
    assert_eq!(port_map.fields.len(), 48);
    assert_eq!(port_map.fields[0].offset_bits, 0);
    assert_eq!(port_map.fields.last().unwrap().offset_bits, 928);

    let format_logs = command_spec(122).unwrap();
    assert_eq!(format_logs.length_bytes, 1);
    assert_eq!(format_logs.fields.len(), 1);
}

#[test]
fn raw_telemetry_response_preserves_payload_details() {
    let telemetry = telemetry_spec(133).unwrap();
    let response = RawTelemetryResponse::success_response(telemetry, 8, "0102030405060708".into());

    assert!(response.success);
    assert_eq!(response.telemetry_id, 133);
    assert_eq!(response.name, "Current Unix Time");
    assert_eq!(response.expected_length_bytes, 8);
    assert_eq!(response.received_length_bytes, 8);
    assert_eq!(response.payload_hex, "0102030405060708");
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
fn encode_adcs_operational_state_payload() {
    let command = AdcsOperationalStateCommand {
        adcs_operational_state: 2,
    };

    assert_eq!(command.encode().unwrap(), vec![2]);
    let spec = command_spec(72).unwrap();
    assert_eq!(spec.length_bytes, 1);
    assert_eq!(spec.fields.len(), 1);
    assert_eq!(spec.fields[0].offset_bits, 0);
    assert_eq!(spec.fields[0].length_bits, 8);
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

#[test]
fn decode_mag_sensing_element_enum_labels() {
    let primary = MagSensingElementConfigurationTelemetry::decode(&[0b0000_0000]).unwrap();
    assert_eq!(primary.mag0_sensing_element_raw, 0);
    assert_eq!(primary.mag0_sensing_element, Some("MagPrimary".to_string()));
    assert_eq!(primary.mag1_sensing_element_raw, 0);
    assert_eq!(primary.mag1_sensing_element, Some("MagPrimary".to_string()));

    let redundant = MagSensingElementConfigurationTelemetry::decode(&[0b0000_0011]).unwrap();
    assert_eq!(redundant.mag0_sensing_element_raw, 1);
    assert_eq!(
        redundant.mag0_sensing_element,
        Some("MagRedundant".to_string())
    );
    assert_eq!(redundant.mag1_sensing_element_raw, 1);
    assert_eq!(
        redundant.mag1_sensing_element,
        Some("MagRedundant".to_string())
    );
}

#[test]
fn decode_adcs_run_mode_enum_labels() {
    for (raw, expected) in [
        (0, "Off"),
        (1, "Enabled"),
        (2, "Triggered"),
        (3, "Simulation"),
    ] {
        let decoded = AdcsRunModeTelemetry::decode(&[raw]).unwrap();
        assert_eq!(decoded.adcs_run_mode_raw, raw);
        assert_eq!(decoded.adcs_run_mode.as_deref(), Some(expected));
    }

    assert_eq!(
        AdcsRunModeTelemetry::decode(&[4]).unwrap().adcs_run_mode,
        None
    );

    let default = DefaultModeConfigurationTelemetry::decode(&[3, 0, 0, 0]).unwrap();
    assert_eq!(default.default_adcs_run_mode_raw, 3);
    assert_eq!(default.default_adcs_run_mode.as_deref(), Some("Simulation"));
}

#[test]
fn decode_orbit_mode_enum_labels() {
    for (raw, expected) in [
        (0, "OrbTle"),
        (1, "OrbTleGnss"),
        (2, "OrbAsgp4"),
        (3, "OrbAsgp4Gnss"),
    ] {
        let decoded = OrbitModeTelemetry::decode(&[raw]).unwrap();
        assert_eq!(decoded.orbit_mode_raw, raw);
        assert_eq!(decoded.orbit_mode.as_deref(), Some(expected));
    }

    assert_eq!(OrbitModeTelemetry::decode(&[4]).unwrap().orbit_mode, None);
}

#[test]
fn generated_telemetry_has_no_null_enum_placeholder() {
    assert!(!include_str!("telemetry.rs").contains("unknown_enum_label("));

    for table in [
        "table_13",
        "table_29",
        "table_30",
        "table_38",
        "table_41",
        "table_51",
        "table_55",
        "table_56",
        "table_57",
        "table_58",
        "table_59",
        "table_61",
        "table_64",
        "table_78",
        "table_80",
        "table_84",
        "table_85",
        "table_88",
        "table_90",
        "table_91",
        "table_93",
        "table_99",
        "table_101",
        "table_107",
        "table_109",
        "table_110",
        "table_111",
        "table_113",
        "table_114",
        "table_117",
        "table_120",
        "table_126",
        "table_139",
        "table_140",
        "table_175",
        "table_176",
        "table_177",
        "table_178",
        "table_179",
        "table_180",
        "table_181",
        "table_182",
        "table_183",
        "table_200",
        "table_203",
        "table_205",
        "table_208",
        "table_214",
        "table_215",
    ] {
        assert!(
            crate::telemetry::telemetry_enum_label(table, 0).is_some(),
            "{} is missing its zero-value enum label",
            table
        );
    }
}
