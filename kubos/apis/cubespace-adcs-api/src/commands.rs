//! Generated typed CubeSpace ADCS telecommands.

use crate::{codec, AdcsResult, CommandSpec, DataType, FieldSpec, Telecommand};
use async_graphql::InputObject;

const RESET_COMMAND_FIELDS: &[FieldSpec] = &[FieldSpec {
    offset_bits: 0,
    length_bits: 8,
    name: "Reset Type",
    data_type: DataType::Enum,
    description: "The type of reset to perform. Possible values are in Table 3",
    scale: None,
    unit: None,
    enum_table: Some("table_3"),
}];

/// Telecommand input for Reset.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct ResetCommand {
    /// Reset Type. The type of reset to perform. Possible values are in Table 3
    pub reset_type: u8,
}

impl Telecommand for ResetCommand {
    const ID: u8 = 1;
    const NAME: &'static str = "Reset";
    const LENGTH_BYTES: usize = 1;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Reset Type",
            u128::from(self.reset_type),
        )?;
        Ok(payload)
    }
}

const CURRENT_UNIX_TIME_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "Current Unix seconds",
        data_type: DataType::Uint,
        description: "Current Unix time s. (Unit of measure is [s])",
        scale: None,
        unit: Some("s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "Current Unix Nanoseconds",
        data_type: DataType::Uint,
        description: "Current Unix time ns. (Unit of measure is [ns])",
        scale: None,
        unit: Some("ns"),
        enum_table: None,
    },
];

/// Telecommand input for Current Unix Time.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct CurrentUnixTimeCommand {
    /// Current Unix seconds. Current Unix time s. (Unit of measure is [s])
    pub current_unix_seconds: u32,
    /// Current Unix Nanoseconds. Current Unix time ns. (Unit of measure is [ns])
    pub current_unix_nanoseconds: u32,
}

impl Telecommand for CurrentUnixTimeCommand {
    const ID: u8 = 2;
    const NAME: &'static str = "Current Unix Time";
    const LENGTH_BYTES: usize = 8;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "Current Unix seconds",
            u128::from(self.current_unix_seconds),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "Current Unix Nanoseconds",
            u128::from(self.current_unix_nanoseconds),
        )?;
        Ok(payload)
    }
}

const ERROR_LOG_CLEAR_COMMAND_FIELDS: &[FieldSpec] = &[];

/// Telecommand input for Error Log Clear.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ErrorLogClearCommand {}

impl Telecommand for ErrorLogClearCommand {
    const ID: u8 = 5;
    const NAME: &'static str = "Error Log Clear";
    const LENGTH_BYTES: usize = 0;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let payload = vec![0; Self::LENGTH_BYTES];
        Ok(payload)
    }
}

const PERSIST_CONFIG_COMMAND_FIELDS: &[FieldSpec] = &[];

/// Telecommand input for Persist Config.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistConfigCommand {}

impl Telecommand for PersistConfigCommand {
    const ID: u8 = 7;
    const NAME: &'static str = "Persist Config";
    const LENGTH_BYTES: usize = 0;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let payload = vec![0; Self::LENGTH_BYTES];
        Ok(payload)
    }
}

const CONTROL_AND_ESTIMATION_MODE_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Control mode",
        data_type: DataType::Enum,
        description: "Control mode. Possible values are in Table 9",
        scale: None,
        unit: None,
        enum_table: Some("table_9"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "Main estimator mode",
        data_type: DataType::Enum,
        description: "Main estimator mode. Possible values are in Table 10",
        scale: None,
        unit: None,
        enum_table: Some("table_10"),
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 8,
        name: "Backup estimator mode",
        data_type: DataType::Enum,
        description: "Backup estimator mode. Possible values are in Table 10",
        scale: None,
        unit: None,
        enum_table: Some("table_10"),
    },
    FieldSpec {
        offset_bits: 24,
        length_bits: 16,
        name: "Control timeout",
        data_type: DataType::Uint,
        description: "Control timeout. (Unit of measure is [s])",
        scale: None,
        unit: Some("s"),
        enum_table: None,
    },
];

/// Telecommand input for Control and estimation mode.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct ControlAndEstimationModeCommand {
    /// Control mode. Control mode. Possible values are in Table 9
    pub control_mode: u8,
    /// Main estimator mode. Main estimator mode. Possible values are in Table 10
    pub main_estimator_mode: u8,
    /// Backup estimator mode. Backup estimator mode. Possible values are in Table 10
    pub backup_estimator_mode: u8,
    /// Control timeout. Control timeout. (Unit of measure is [s])
    pub control_timeout: u16,
}

impl Telecommand for ControlAndEstimationModeCommand {
    const ID: u8 = 42;
    const NAME: &'static str = "Control and estimation mode";
    const LENGTH_BYTES: usize = 5;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Control mode",
            u128::from(self.control_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "Main estimator mode",
            u128::from(self.main_estimator_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            8,
            "Backup estimator mode",
            u128::from(self.backup_estimator_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            24,
            16,
            "Control timeout",
            u128::from(self.control_timeout),
        )?;
        Ok(payload)
    }
}

const COMMANDED_GNSS_MEASUREMENTS_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "GNSS Time integer seconds",
        data_type: DataType::Uint,
        description: "GNSS Unix time integer seconds. (Unit of measure is [s])",
        scale: None,
        unit: Some("s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "GNSS Time nanoseconds",
        data_type: DataType::Uint,
        description: "GNSS Unix time fraction nanoseconds. (Unit of measure is [ns])",
        scale: None,
        unit: Some("ns"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "Satellite position vector X component (GNSS frame)",
        data_type: DataType::Int,
        description:
            "Satellite position vector X component (GNSS frame). (Unit of measure is [cm])",
        scale: None,
        unit: Some("cm"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 32,
        name: "Satellite position vector Y component (GNSS frame)",
        data_type: DataType::Int,
        description:
            "Satellite position vector Y component (GNSS frame). (Unit of measure is [cm])",
        scale: None,
        unit: Some("cm"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 32,
        name: "Satellite position vector Z component (GNSS frame)",
        data_type: DataType::Int,
        description:
            "Satellite position vector Z component (GNSS frame). (Unit of measure is [cm])",
        scale: None,
        unit: Some("cm"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 160,
        length_bits: 32,
        name: "Satellite velocity vector X component (GNSS frame)",
        data_type: DataType::Int,
        description:
            "Satellite velocity vector X component (GNSS frame). (Unit of measure is [cm/s])",
        scale: None,
        unit: Some("cm/s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 192,
        length_bits: 32,
        name: "Satellite velocity vector Y component (GNSS frame)",
        data_type: DataType::Int,
        description:
            "Satellite velocity vector Y component (GNSS frame). (Unit of measure is [cm/s])",
        scale: None,
        unit: Some("cm/s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 224,
        length_bits: 32,
        name: "Satellite velocity vector Z component (GNSS frame)",
        data_type: DataType::Int,
        description:
            "Satellite velocity vector Z component (GNSS frame). (Unit of measure is [cm/s])",
        scale: None,
        unit: Some("cm/s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 256,
        length_bits: 1,
        name: "Sync Time",
        data_type: DataType::Bool,
        description: "Flag to indicate if RTC should sync with unix time",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Commanded GNSS Measurements.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct CommandedGnssMeasurementsCommand {
    /// GNSS Time integer seconds. GNSS Unix time integer seconds. (Unit of measure is [s])
    pub gnss_time_integer_seconds: u32,
    /// GNSS Time nanoseconds. GNSS Unix time fraction nanoseconds. (Unit of measure is [ns])
    pub gnss_time_nanoseconds: u32,
    /// Satellite position vector X component (GNSS frame). Satellite position vector X component (GNSS frame). (Unit of measure is [cm])
    pub satellite_position_vector_x_component_gnss_frame: i32,
    /// Satellite position vector Y component (GNSS frame). Satellite position vector Y component (GNSS frame). (Unit of measure is [cm])
    pub satellite_position_vector_y_component_gnss_frame: i32,
    /// Satellite position vector Z component (GNSS frame). Satellite position vector Z component (GNSS frame). (Unit of measure is [cm])
    pub satellite_position_vector_z_component_gnss_frame: i32,
    /// Satellite velocity vector X component (GNSS frame). Satellite velocity vector X component (GNSS frame). (Unit of measure is [cm/s])
    pub satellite_velocity_vector_x_component_gnss_frame: i32,
    /// Satellite velocity vector Y component (GNSS frame). Satellite velocity vector Y component (GNSS frame). (Unit of measure is [cm/s])
    pub satellite_velocity_vector_y_component_gnss_frame: i32,
    /// Satellite velocity vector Z component (GNSS frame). Satellite velocity vector Z component (GNSS frame). (Unit of measure is [cm/s])
    pub satellite_velocity_vector_z_component_gnss_frame: i32,
    /// Sync Time. Flag to indicate if RTC should sync with unix time
    pub sync_time: bool,
}

impl Telecommand for CommandedGnssMeasurementsCommand {
    const ID: u8 = 49;
    const NAME: &'static str = "Commanded GNSS Measurements";
    const LENGTH_BYTES: usize = 33;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "GNSS Time integer seconds",
            u128::from(self.gnss_time_integer_seconds),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "GNSS Time nanoseconds",
            u128::from(self.gnss_time_nanoseconds),
        )?;
        codec::write_signed(
            &mut payload,
            64,
            32,
            "Satellite position vector X component (GNSS frame)",
            i128::from(self.satellite_position_vector_x_component_gnss_frame),
        )?;
        codec::write_signed(
            &mut payload,
            96,
            32,
            "Satellite position vector Y component (GNSS frame)",
            i128::from(self.satellite_position_vector_y_component_gnss_frame),
        )?;
        codec::write_signed(
            &mut payload,
            128,
            32,
            "Satellite position vector Z component (GNSS frame)",
            i128::from(self.satellite_position_vector_z_component_gnss_frame),
        )?;
        codec::write_signed(
            &mut payload,
            160,
            32,
            "Satellite velocity vector X component (GNSS frame)",
            i128::from(self.satellite_velocity_vector_x_component_gnss_frame),
        )?;
        codec::write_signed(
            &mut payload,
            192,
            32,
            "Satellite velocity vector Y component (GNSS frame)",
            i128::from(self.satellite_velocity_vector_y_component_gnss_frame),
        )?;
        codec::write_signed(
            &mut payload,
            224,
            32,
            "Satellite velocity vector Z component (GNSS frame)",
            i128::from(self.satellite_velocity_vector_z_component_gnss_frame),
        )?;
        codec::write_unsigned(
            &mut payload,
            256,
            1,
            "Sync Time",
            if self.sync_time { 1 } else { 0 },
        )?;
        Ok(payload)
    }
}

const ORBIT_MODE_COMMAND_FIELDS: &[FieldSpec] = &[FieldSpec {
    offset_bits: 0,
    length_bits: 8,
    name: "Orbit mode",
    data_type: DataType::Enum,
    description: "Orbit calculation mode. Possible values are in Table 19",
    scale: None,
    unit: None,
    enum_table: Some("table_19"),
}];

/// Telecommand input for Orbit Mode.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct OrbitModeCommand {
    /// Orbit mode. Orbit calculation mode. Possible values are in Table 19
    pub orbit_mode: u8,
}

impl Telecommand for OrbitModeCommand {
    const ID: u8 = 51;
    const NAME: &'static str = "Orbit Mode";
    const LENGTH_BYTES: usize = 1;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Orbit mode",
            u128::from(self.orbit_mode),
        )?;
        Ok(payload)
    }
}

const MAG_DEPLOY_COMMAND_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 1,
        name: "Deploy MAG0",
        data_type: DataType::Bool,
        description: "Deploy MAG0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 1,
        length_bits: 1,
        name: "Deploy MAG1",
        data_type: DataType::Bool,
        description: "Deploy MAG1",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Mag Deploy Command.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct MagDeployCommand {
    /// Deploy MAG0. Deploy MAG0
    pub deploy_mag0: bool,
    /// Deploy MAG1. Deploy MAG1
    pub deploy_mag1: bool,
}

impl Telecommand for MagDeployCommand {
    const ID: u8 = 52;
    const NAME: &'static str = "Mag Deploy Command";
    const LENGTH_BYTES: usize = 1;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            1,
            "Deploy MAG0",
            if self.deploy_mag0 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            1,
            1,
            "Deploy MAG1",
            if self.deploy_mag1 { 1 } else { 0 },
        )?;
        Ok(payload)
    }
}

const REFERENCE_RPY_VALUES_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "RPY Roll command",
        data_type: DataType::Float,
        description: "RPY Roll command. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "RPY Pitch command",
        data_type: DataType::Float,
        description: "RPY Pitch command. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "RPY Yaw command",
        data_type: DataType::Float,
        description: "RPY Yaw command. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
];

/// Telecommand input for Reference RPY Values.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct ReferenceRpyValuesCommand {
    /// RPY Roll command. RPY Roll command. (Unit of measure is [deg])
    pub rpy_roll_command: f32,
    /// RPY Pitch command. RPY Pitch command. (Unit of measure is [deg])
    pub rpy_pitch_command: f32,
    /// RPY Yaw command. RPY Yaw command. (Unit of measure is [deg])
    pub rpy_yaw_command: f32,
}

impl Telecommand for ReferenceRpyValuesCommand {
    const ID: u8 = 54;
    const NAME: &'static str = "Reference RPY Values";
    const LENGTH_BYTES: usize = 12;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "RPY Roll command",
            u128::from(self.rpy_roll_command.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "RPY Pitch command",
            u128::from(self.rpy_pitch_command.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            32,
            "RPY Yaw command",
            u128::from(self.rpy_yaw_command.to_bits()),
        )?;
        Ok(payload)
    }
}

const OPENLOOPCOMMANDMTQ_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 16,
        name: "MTQ0 open-loop on-time command",
        data_type: DataType::Int,
        description: "MTQ0 open-loop on-time command. (Unit of measure is [ms])",
        scale: None,
        unit: Some("ms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 16,
        name: "MTQ1 open-loop on-time command",
        data_type: DataType::Int,
        description: "MTQ1 open-loop on-time command. (Unit of measure is [ms])",
        scale: None,
        unit: Some("ms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 16,
        name: "MTQ2 open-loop on-time command",
        data_type: DataType::Int,
        description: "MTQ2 open-loop on-time command. (Unit of measure is [ms])",
        scale: None,
        unit: Some("ms"),
        enum_table: None,
    },
];

/// Telecommand input for OpenLoopCommandMtq.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct OpenloopcommandmtqCommand {
    /// MTQ0 open-loop on-time command. MTQ0 open-loop on-time command. (Unit of measure is [ms])
    pub mtq0_open_loop_on_time_command: i16,
    /// MTQ1 open-loop on-time command. MTQ1 open-loop on-time command. (Unit of measure is [ms])
    pub mtq1_open_loop_on_time_command: i16,
    /// MTQ2 open-loop on-time command. MTQ2 open-loop on-time command. (Unit of measure is [ms])
    pub mtq2_open_loop_on_time_command: i16,
}

impl Telecommand for OpenloopcommandmtqCommand {
    const ID: u8 = 55;
    const NAME: &'static str = "OpenLoopCommandMtq";
    const LENGTH_BYTES: usize = 6;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_signed(
            &mut payload,
            0,
            16,
            "MTQ0 open-loop on-time command",
            i128::from(self.mtq0_open_loop_on_time_command),
        )?;
        codec::write_signed(
            &mut payload,
            16,
            16,
            "MTQ1 open-loop on-time command",
            i128::from(self.mtq1_open_loop_on_time_command),
        )?;
        codec::write_signed(
            &mut payload,
            32,
            16,
            "MTQ2 open-loop on-time command",
            i128::from(self.mtq2_open_loop_on_time_command),
        )?;
        Ok(payload)
    }
}

const POWERSTATE_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "RWL0 power state",
        data_type: DataType::Enum,
        description: "RWL0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "RWL1 power state",
        data_type: DataType::Enum,
        description: "RWL1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 8,
        name: "RWL2 power state",
        data_type: DataType::Enum,
        description: "RWL2 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 24,
        length_bits: 8,
        name: "RWL3 power state",
        data_type: DataType::Enum,
        description: "RWL3 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 8,
        name: "MAG0 power state",
        data_type: DataType::Enum,
        description: "MAG0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 40,
        length_bits: 8,
        name: "MAG1 power state",
        data_type: DataType::Enum,
        description: "MAG1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 8,
        name: "GYR0 power state",
        data_type: DataType::Enum,
        description: "GYR0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 56,
        length_bits: 8,
        name: "GYR1 power state",
        data_type: DataType::Enum,
        description: "GYR1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 8,
        name: "FSS0 power state",
        data_type: DataType::Enum,
        description: "FSS0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 72,
        length_bits: 8,
        name: "FSS1 power state",
        data_type: DataType::Enum,
        description: "FSS1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 80,
        length_bits: 8,
        name: "FSS2 power state",
        data_type: DataType::Enum,
        description: "FSS2 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 88,
        length_bits: 8,
        name: "FSS3 power state",
        data_type: DataType::Enum,
        description: "FSS3 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 8,
        name: "HSS0 power state",
        data_type: DataType::Enum,
        description: "HSS0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 104,
        length_bits: 8,
        name: "HSS1 power state",
        data_type: DataType::Enum,
        description: "HSS1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 112,
        length_bits: 8,
        name: "STR0 power state",
        data_type: DataType::Enum,
        description: "STR0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 120,
        length_bits: 8,
        name: "STR1 power state",
        data_type: DataType::Enum,
        description: "STR1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 8,
        name: "ExtSensor0 power state",
        data_type: DataType::Enum,
        description: "ExtSensor0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 136,
        length_bits: 8,
        name: "ExtSensor1 power state",
        data_type: DataType::Enum,
        description: "ExtSensor1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 144,
        length_bits: 8,
        name: "EXTGYR0 power state",
        data_type: DataType::Enum,
        description: "EXTGYR0 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
    FieldSpec {
        offset_bits: 152,
        length_bits: 8,
        name: "EXTGYR1 power state",
        data_type: DataType::Enum,
        description: "EXTGYR1 power state. Possible values are in Table 24",
        scale: None,
        unit: None,
        enum_table: Some("table_24"),
    },
];

/// Telecommand input for PowerState.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct PowerstateCommand {
    /// RWL0 power state. RWL0 power state. Possible values are in Table 24
    pub rwl0_power_state: u8,
    /// RWL1 power state. RWL1 power state. Possible values are in Table 24
    pub rwl1_power_state: u8,
    /// RWL2 power state. RWL2 power state. Possible values are in Table 24
    pub rwl2_power_state: u8,
    /// RWL3 power state. RWL3 power state. Possible values are in Table 24
    pub rwl3_power_state: u8,
    /// MAG0 power state. MAG0 power state. Possible values are in Table 24
    pub mag0_power_state: u8,
    /// MAG1 power state. MAG1 power state. Possible values are in Table 24
    pub mag1_power_state: u8,
    /// GYR0 power state. GYR0 power state. Possible values are in Table 24
    pub gyr0_power_state: u8,
    /// GYR1 power state. GYR1 power state. Possible values are in Table 24
    pub gyr1_power_state: u8,
    /// FSS0 power state. FSS0 power state. Possible values are in Table 24
    pub fss0_power_state: u8,
    /// FSS1 power state. FSS1 power state. Possible values are in Table 24
    pub fss1_power_state: u8,
    /// FSS2 power state. FSS2 power state. Possible values are in Table 24
    pub fss2_power_state: u8,
    /// FSS3 power state. FSS3 power state. Possible values are in Table 24
    pub fss3_power_state: u8,
    /// HSS0 power state. HSS0 power state. Possible values are in Table 24
    pub hss0_power_state: u8,
    /// HSS1 power state. HSS1 power state. Possible values are in Table 24
    pub hss1_power_state: u8,
    /// STR0 power state. STR0 power state. Possible values are in Table 24
    pub str0_power_state: u8,
    /// STR1 power state. STR1 power state. Possible values are in Table 24
    pub str1_power_state: u8,
    /// ExtSensor0 power state. ExtSensor0 power state. Possible values are in Table 24
    pub extsensor0_power_state: u8,
    /// ExtSensor1 power state. ExtSensor1 power state. Possible values are in Table 24
    pub extsensor1_power_state: u8,
    /// EXTGYR0 power state. EXTGYR0 power state. Possible values are in Table 24
    pub extgyr0_power_state: u8,
    /// EXTGYR1 power state. EXTGYR1 power state. Possible values are in Table 24
    pub extgyr1_power_state: u8,
}

impl Telecommand for PowerstateCommand {
    const ID: u8 = 56;
    const NAME: &'static str = "PowerState";
    const LENGTH_BYTES: usize = 20;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "RWL0 power state",
            u128::from(self.rwl0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "RWL1 power state",
            u128::from(self.rwl1_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            8,
            "RWL2 power state",
            u128::from(self.rwl2_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            24,
            8,
            "RWL3 power state",
            u128::from(self.rwl3_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            8,
            "MAG0 power state",
            u128::from(self.mag0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            40,
            8,
            "MAG1 power state",
            u128::from(self.mag1_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            48,
            8,
            "GYR0 power state",
            u128::from(self.gyr0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            56,
            8,
            "GYR1 power state",
            u128::from(self.gyr1_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            8,
            "FSS0 power state",
            u128::from(self.fss0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            72,
            8,
            "FSS1 power state",
            u128::from(self.fss1_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            80,
            8,
            "FSS2 power state",
            u128::from(self.fss2_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            88,
            8,
            "FSS3 power state",
            u128::from(self.fss3_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            96,
            8,
            "HSS0 power state",
            u128::from(self.hss0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            104,
            8,
            "HSS1 power state",
            u128::from(self.hss1_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            112,
            8,
            "STR0 power state",
            u128::from(self.str0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            120,
            8,
            "STR1 power state",
            u128::from(self.str1_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            128,
            8,
            "ExtSensor0 power state",
            u128::from(self.extsensor0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            136,
            8,
            "ExtSensor1 power state",
            u128::from(self.extsensor1_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            144,
            8,
            "EXTGYR0 power state",
            u128::from(self.extgyr0_power_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            152,
            8,
            "EXTGYR1 power state",
            u128::from(self.extgyr1_power_state),
        )?;
        Ok(payload)
    }
}

const ADCS_RUN_MODE_COMMAND_FIELDS: &[FieldSpec] = &[FieldSpec {
    offset_bits: 0,
    length_bits: 8,
    name: "ADCS run mode",
    data_type: DataType::Enum,
    description: "ADCS run mode. Possible values are in Table 26",
    scale: None,
    unit: None,
    enum_table: Some("table_26"),
}];

/// Telecommand input for ADCS Run Mode.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct AdcsRunModeCommand {
    /// ADCS run mode. ADCS run mode. Possible values are in Table 26
    pub adcs_run_mode: u8,
}

impl Telecommand for AdcsRunModeCommand {
    const ID: u8 = 57;
    const NAME: &'static str = "ADCS Run Mode";
    const LENGTH_BYTES: usize = 1;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "ADCS run mode",
            u128::from(self.adcs_run_mode),
        )?;
        Ok(payload)
    }
}

const CONTROL_MODE_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Control mode",
        data_type: DataType::Enum,
        description: "Control mode. Possible values are in Table 9",
        scale: None,
        unit: None,
        enum_table: Some("table_9"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 16,
        name: "Control timeout",
        data_type: DataType::Uint,
        description: "Control timeout. (Unit of measure is [s])",
        scale: None,
        unit: Some("s"),
        enum_table: None,
    },
];

/// Telecommand input for Control Mode.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct ControlModeCommand {
    /// Control mode. Control mode. Possible values are in Table 9
    pub control_mode: u8,
    /// Control timeout. Control timeout. (Unit of measure is [s])
    pub control_timeout: u16,
}

impl Telecommand for ControlModeCommand {
    const ID: u8 = 58;
    const NAME: &'static str = "Control Mode";
    const LENGTH_BYTES: usize = 3;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Control mode",
            u128::from(self.control_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            16,
            "Control timeout",
            u128::from(self.control_timeout),
        )?;
        Ok(payload)
    }
}

const WHEEL_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "RWL0 inertia",
        data_type: DataType::Float,
        description: "RWL0 inertia. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "RWL0 maximum momentum",
        data_type: DataType::Float,
        description: "RWL0 maximum momentum. (Unit of measure is [N.m.s])",
        scale: None,
        unit: Some("N.m.s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "RWL0 maximum torque",
        data_type: DataType::Float,
        description: "RWL0 maximum torque. (Unit of measure is [N.m])",
        scale: None,
        unit: Some("N.m"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 32,
        name: "RWL1 inertia",
        data_type: DataType::Float,
        description: "RWL1 inertia. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 32,
        name: "RWL1 maximum momentum",
        data_type: DataType::Float,
        description: "RWL1 maximum momentum. (Unit of measure is [N.m.s])",
        scale: None,
        unit: Some("N.m.s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 160,
        length_bits: 32,
        name: "RWL1 maximum torque",
        data_type: DataType::Float,
        description: "RWL1 maximum torque. (Unit of measure is [N.m])",
        scale: None,
        unit: Some("N.m"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 192,
        length_bits: 32,
        name: "RWL2 inertia",
        data_type: DataType::Float,
        description: "RWL2 inertia. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 224,
        length_bits: 32,
        name: "RWL2 maximum momentum",
        data_type: DataType::Float,
        description: "RWL2 maximum momentum. (Unit of measure is [N.m.s])",
        scale: None,
        unit: Some("N.m.s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 256,
        length_bits: 32,
        name: "RWL2 maximum torque",
        data_type: DataType::Float,
        description: "RWL2 maximum torque. (Unit of measure is [N.m])",
        scale: None,
        unit: Some("N.m"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 288,
        length_bits: 32,
        name: "RWL3 inertia",
        data_type: DataType::Float,
        description: "RWL3 inertia. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 320,
        length_bits: 32,
        name: "RWL3 maximum momentum",
        data_type: DataType::Float,
        description: "RWL3 maximum momentum. (Unit of measure is [N.m.s])",
        scale: None,
        unit: Some("N.m.s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 352,
        length_bits: 32,
        name: "RWL3 maximum torque",
        data_type: DataType::Float,
        description: "RWL3 maximum torque. (Unit of measure is [N.m])",
        scale: None,
        unit: Some("N.m"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 384,
        length_bits: 32,
        name: "Wheel ramp torque",
        data_type: DataType::Float,
        description: "Wheel ramp torque. (Unit of measure is [N.m])",
        scale: None,
        unit: Some("N.m"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 416,
        length_bits: 8,
        name: "Wheel scheme",
        data_type: DataType::Enum,
        description: "Wheel scheme. Possible values are in Table 29",
        scale: None,
        unit: None,
        enum_table: Some("table_29"),
    },
    FieldSpec {
        offset_bits: 424,
        length_bits: 8,
        name: "Failed wheel ID",
        data_type: DataType::Enum,
        description: "Failed wheel ID. Possible values are in Table 30",
        scale: None,
        unit: None,
        enum_table: Some("table_30"),
    },
    FieldSpec {
        offset_bits: 432,
        length_bits: 32,
        name: "Pyramid nominal momentum",
        data_type: DataType::Float,
        description: "Pyramid nominal momentum. (Unit of measure is [N.m.s])",
        scale: None,
        unit: Some("N.m.s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 464,
        length_bits: 32,
        name: "Pyramid tilt angle",
        data_type: DataType::Float,
        description: "Pyramid tilt angle. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
];

/// Telecommand input for Wheel Configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct WheelConfigurationCommand {
    /// RWL0 inertia. RWL0 inertia. (Unit of measure is [kg.m^2])
    pub rwl0_inertia: f32,
    /// RWL0 maximum momentum. RWL0 maximum momentum. (Unit of measure is [N.m.s])
    pub rwl0_maximum_momentum: f32,
    /// RWL0 maximum torque. RWL0 maximum torque. (Unit of measure is [N.m])
    pub rwl0_maximum_torque: f32,
    /// RWL1 inertia. RWL1 inertia. (Unit of measure is [kg.m^2])
    pub rwl1_inertia: f32,
    /// RWL1 maximum momentum. RWL1 maximum momentum. (Unit of measure is [N.m.s])
    pub rwl1_maximum_momentum: f32,
    /// RWL1 maximum torque. RWL1 maximum torque. (Unit of measure is [N.m])
    pub rwl1_maximum_torque: f32,
    /// RWL2 inertia. RWL2 inertia. (Unit of measure is [kg.m^2])
    pub rwl2_inertia: f32,
    /// RWL2 maximum momentum. RWL2 maximum momentum. (Unit of measure is [N.m.s])
    pub rwl2_maximum_momentum: f32,
    /// RWL2 maximum torque. RWL2 maximum torque. (Unit of measure is [N.m])
    pub rwl2_maximum_torque: f32,
    /// RWL3 inertia. RWL3 inertia. (Unit of measure is [kg.m^2])
    pub rwl3_inertia: f32,
    /// RWL3 maximum momentum. RWL3 maximum momentum. (Unit of measure is [N.m.s])
    pub rwl3_maximum_momentum: f32,
    /// RWL3 maximum torque. RWL3 maximum torque. (Unit of measure is [N.m])
    pub rwl3_maximum_torque: f32,
    /// Wheel ramp torque. Wheel ramp torque. (Unit of measure is [N.m])
    pub wheel_ramp_torque: f32,
    /// Wheel scheme. Wheel scheme. Possible values are in Table 29
    pub wheel_scheme: u8,
    /// Failed wheel ID. Failed wheel ID. Possible values are in Table 30
    pub failed_wheel_id: u8,
    /// Pyramid nominal momentum. Pyramid nominal momentum. (Unit of measure is [N.m.s])
    pub pyramid_nominal_momentum: f32,
    /// Pyramid tilt angle. Pyramid tilt angle. (Unit of measure is [deg])
    pub pyramid_tilt_angle: f32,
}

impl Telecommand for WheelConfigurationCommand {
    const ID: u8 = 59;
    const NAME: &'static str = "Wheel Configuration";
    const LENGTH_BYTES: usize = 62;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "RWL0 inertia",
            u128::from(self.rwl0_inertia.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "RWL0 maximum momentum",
            u128::from(self.rwl0_maximum_momentum.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            32,
            "RWL0 maximum torque",
            u128::from(self.rwl0_maximum_torque.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            96,
            32,
            "RWL1 inertia",
            u128::from(self.rwl1_inertia.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            128,
            32,
            "RWL1 maximum momentum",
            u128::from(self.rwl1_maximum_momentum.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            160,
            32,
            "RWL1 maximum torque",
            u128::from(self.rwl1_maximum_torque.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            192,
            32,
            "RWL2 inertia",
            u128::from(self.rwl2_inertia.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            224,
            32,
            "RWL2 maximum momentum",
            u128::from(self.rwl2_maximum_momentum.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            256,
            32,
            "RWL2 maximum torque",
            u128::from(self.rwl2_maximum_torque.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            288,
            32,
            "RWL3 inertia",
            u128::from(self.rwl3_inertia.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            320,
            32,
            "RWL3 maximum momentum",
            u128::from(self.rwl3_maximum_momentum.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            352,
            32,
            "RWL3 maximum torque",
            u128::from(self.rwl3_maximum_torque.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            384,
            32,
            "Wheel ramp torque",
            u128::from(self.wheel_ramp_torque.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            416,
            8,
            "Wheel scheme",
            u128::from(self.wheel_scheme),
        )?;
        codec::write_unsigned(
            &mut payload,
            424,
            8,
            "Failed wheel ID",
            u128::from(self.failed_wheel_id),
        )?;
        codec::write_unsigned(
            &mut payload,
            432,
            32,
            "Pyramid nominal momentum",
            u128::from(self.pyramid_nominal_momentum.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            464,
            32,
            "Pyramid tilt angle",
            u128::from(self.pyramid_tilt_angle.to_bits()),
        )?;
        Ok(payload)
    }
}

const ADCS_SATELLITE_CONFIG_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "Moment of inertia Ixx",
        data_type: DataType::Float,
        description: "Moment of inertia Ixx. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "Moment of inertia Iyy",
        data_type: DataType::Float,
        description: "Moment of inertia Iyy. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "Moment of inertia Izz",
        data_type: DataType::Float,
        description: "Moment of inertia Izz. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 32,
        name: "Product of inertia Ixy",
        data_type: DataType::Float,
        description: "Product of inertia Ixy. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 32,
        name: "Product of inertia Ixz",
        data_type: DataType::Float,
        description: "Product of inertia Ixz. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 160,
        length_bits: 32,
        name: "Product of inertia Iyz",
        data_type: DataType::Float,
        description: "Product of inertia Iyz. (Unit of measure is [kg.m^2])",
        scale: None,
        unit: Some("kg.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 192,
        length_bits: 16,
        name: "Sun-pointing body vector X component",
        data_type: DataType::Int,
        description: "Sun-pointing body vector X component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 208,
        length_bits: 16,
        name: "Sun-pointing body vector Y component",
        data_type: DataType::Int,
        description: "Sun-pointing body vector Y component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 224,
        length_bits: 16,
        name: "Sun-pointing body vector Z component",
        data_type: DataType::Int,
        description: "Sun-pointing body vector Z component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 240,
        length_bits: 16,
        name: "Target-tracking body vector X component",
        data_type: DataType::Int,
        description: "Target-tracking body vector X component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 256,
        length_bits: 16,
        name: "Target-tracking body vector Y component",
        data_type: DataType::Int,
        description: "Target-tracking body vector Y component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 272,
        length_bits: 16,
        name: "Target-tracking body vector Z component",
        data_type: DataType::Int,
        description: "Target-tracking body vector Z component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 288,
        length_bits: 16,
        name: "Satellite-tracking body vector X component",
        data_type: DataType::Int,
        description: "Satellite-tracking body vector X component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 304,
        length_bits: 16,
        name: "Satellite-tracking body vector Y component",
        data_type: DataType::Int,
        description: "Satellite-tracking body vector Y component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 320,
        length_bits: 16,
        name: "Satellite-tracking body vector Z component",
        data_type: DataType::Int,
        description: "Satellite-tracking body vector Z component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for ADCS Satellite Config.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct AdcsSatelliteConfigCommand {
    /// Moment of inertia Ixx. Moment of inertia Ixx. (Unit of measure is [kg.m^2])
    pub moment_of_inertia_ixx: f32,
    /// Moment of inertia Iyy. Moment of inertia Iyy. (Unit of measure is [kg.m^2])
    pub moment_of_inertia_iyy: f32,
    /// Moment of inertia Izz. Moment of inertia Izz. (Unit of measure is [kg.m^2])
    pub moment_of_inertia_izz: f32,
    /// Product of inertia Ixy. Product of inertia Ixy. (Unit of measure is [kg.m^2])
    pub product_of_inertia_ixy: f32,
    /// Product of inertia Ixz. Product of inertia Ixz. (Unit of measure is [kg.m^2])
    pub product_of_inertia_ixz: f32,
    /// Product of inertia Iyz. Product of inertia Iyz. (Unit of measure is [kg.m^2])
    pub product_of_inertia_iyz: f32,
    /// Sun-pointing body vector X component. Sun-pointing body vector X component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub sun_pointing_body_vector_x_component: f64,
    /// Sun-pointing body vector Y component. Sun-pointing body vector Y component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub sun_pointing_body_vector_y_component: f64,
    /// Sun-pointing body vector Z component. Sun-pointing body vector Z component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub sun_pointing_body_vector_z_component: f64,
    /// Target-tracking body vector X component. Target-tracking body vector X component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub target_tracking_body_vector_x_component: f64,
    /// Target-tracking body vector Y component. Target-tracking body vector Y component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub target_tracking_body_vector_y_component: f64,
    /// Target-tracking body vector Z component. Target-tracking body vector Z component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub target_tracking_body_vector_z_component: f64,
    /// Satellite-tracking body vector X component. Satellite-tracking body vector X component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub satellite_tracking_body_vector_x_component: f64,
    /// Satellite-tracking body vector Y component. Satellite-tracking body vector Y component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub satellite_tracking_body_vector_y_component: f64,
    /// Satellite-tracking body vector Z component. Satellite-tracking body vector Z component. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*10000.0
    pub satellite_tracking_body_vector_z_component: f64,
}

impl Telecommand for AdcsSatelliteConfigCommand {
    const ID: u8 = 61;
    const NAME: &'static str = "ADCS Satellite Config";
    const LENGTH_BYTES: usize = 42;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "Moment of inertia Ixx",
            u128::from(self.moment_of_inertia_ixx.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "Moment of inertia Iyy",
            u128::from(self.moment_of_inertia_iyy.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            32,
            "Moment of inertia Izz",
            u128::from(self.moment_of_inertia_izz.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            96,
            32,
            "Product of inertia Ixy",
            u128::from(self.product_of_inertia_ixy.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            128,
            32,
            "Product of inertia Ixz",
            u128::from(self.product_of_inertia_ixz.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            160,
            32,
            "Product of inertia Iyz",
            u128::from(self.product_of_inertia_iyz.to_bits()),
        )?;
        codec::write_signed(
            &mut payload,
            192,
            16,
            "Sun-pointing body vector X component",
            (self.sun_pointing_body_vector_x_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            208,
            16,
            "Sun-pointing body vector Y component",
            (self.sun_pointing_body_vector_y_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            224,
            16,
            "Sun-pointing body vector Z component",
            (self.sun_pointing_body_vector_z_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            240,
            16,
            "Target-tracking body vector X component",
            (self.target_tracking_body_vector_x_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            256,
            16,
            "Target-tracking body vector Y component",
            (self.target_tracking_body_vector_y_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            272,
            16,
            "Target-tracking body vector Z component",
            (self.target_tracking_body_vector_z_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            288,
            16,
            "Satellite-tracking body vector X component",
            (self.satellite_tracking_body_vector_x_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            304,
            16,
            "Satellite-tracking body vector Y component",
            (self.satellite_tracking_body_vector_y_component * 10000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            320,
            16,
            "Satellite-tracking body vector Z component",
            (self.satellite_tracking_body_vector_z_component * 10000.0).round() as i128,
        )?;
        Ok(payload)
    }
}

const MAG0_ORBIT_CALIBRATION_CONFIG_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 16,
        name: "Magnetometer channel 1 offset",
        data_type: DataType::Int,
        description: "Magnetometer channel 1 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 16,
        name: "Magnetometer channel 2 offset",
        data_type: DataType::Int,
        description: "Magnetometer channel 2 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 16,
        name: "Magnetometer channel 3 offset",
        data_type: DataType::Int,
        description: "Magnetometer channel 3 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S11",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S11. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S22",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S22. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 80,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S33",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S33. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S12",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S12. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 112,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S13",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S13. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S21",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S21. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 144,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S23",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S23. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 160,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S31",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S31. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 176,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S32",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S32. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Mag0 Orbit Calibration Config.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct Mag0OrbitCalibrationConfigCommand {
    /// Magnetometer channel 1 offset. Magnetometer channel 1 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_channel_1_offset: f64,
    /// Magnetometer channel 2 offset. Magnetometer channel 2 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_channel_2_offset: f64,
    /// Magnetometer channel 3 offset. Magnetometer channel 3 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_channel_3_offset: f64,
    /// Magnetometer sensitivity matrix S11. Magnetometer sensitivity matrix S11. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s11: f64,
    /// Magnetometer sensitivity matrix S22. Magnetometer sensitivity matrix S22. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s22: f64,
    /// Magnetometer sensitivity matrix S33. Magnetometer sensitivity matrix S33. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s33: f64,
    /// Magnetometer sensitivity matrix S12. Magnetometer sensitivity matrix S12. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s12: f64,
    /// Magnetometer sensitivity matrix S13. Magnetometer sensitivity matrix S13. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s13: f64,
    /// Magnetometer sensitivity matrix S21. Magnetometer sensitivity matrix S21. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s21: f64,
    /// Magnetometer sensitivity matrix S23. Magnetometer sensitivity matrix S23. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s23: f64,
    /// Magnetometer sensitivity matrix S31. Magnetometer sensitivity matrix S31. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s31: f64,
    /// Magnetometer sensitivity matrix S32. Magnetometer sensitivity matrix S32. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s32: f64,
}

impl Telecommand for Mag0OrbitCalibrationConfigCommand {
    const ID: u8 = 63;
    const NAME: &'static str = "Mag0 Orbit Calibration Config";
    const LENGTH_BYTES: usize = 24;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_signed(
            &mut payload,
            0,
            16,
            "Magnetometer channel 1 offset",
            (self.magnetometer_channel_1_offset * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            16,
            16,
            "Magnetometer channel 2 offset",
            (self.magnetometer_channel_2_offset * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            32,
            16,
            "Magnetometer channel 3 offset",
            (self.magnetometer_channel_3_offset * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            48,
            16,
            "Magnetometer sensitivity matrix S11",
            (self.magnetometer_sensitivity_matrix_s11 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            64,
            16,
            "Magnetometer sensitivity matrix S22",
            (self.magnetometer_sensitivity_matrix_s22 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            80,
            16,
            "Magnetometer sensitivity matrix S33",
            (self.magnetometer_sensitivity_matrix_s33 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            96,
            16,
            "Magnetometer sensitivity matrix S12",
            (self.magnetometer_sensitivity_matrix_s12 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            112,
            16,
            "Magnetometer sensitivity matrix S13",
            (self.magnetometer_sensitivity_matrix_s13 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            128,
            16,
            "Magnetometer sensitivity matrix S21",
            (self.magnetometer_sensitivity_matrix_s21 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            144,
            16,
            "Magnetometer sensitivity matrix S23",
            (self.magnetometer_sensitivity_matrix_s23 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            160,
            16,
            "Magnetometer sensitivity matrix S31",
            (self.magnetometer_sensitivity_matrix_s31 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            176,
            16,
            "Magnetometer sensitivity matrix S32",
            (self.magnetometer_sensitivity_matrix_s32 * 1000.0).round() as i128,
        )?;
        Ok(payload)
    }
}

const DEFAULT_MODE_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Default ADCS run mode",
        data_type: DataType::Enum,
        description: "Default ADCS run mode. Possible values are in Table 26",
        scale: None,
        unit: None,
        enum_table: Some("table_26"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "Default ADCS operational state",
        data_type: DataType::Enum,
        description: "Default ADCS operational state. Possible values are in Table 36",
        scale: None,
        unit: None,
        enum_table: Some("table_36"),
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 8,
        name: "Default control mode in OpStateSafe",
        data_type: DataType::Enum,
        description: "Default control mode in OpStateSafe. Possible values are in Table 9",
        scale: None,
        unit: None,
        enum_table: Some("table_9"),
    },
    FieldSpec {
        offset_bits: 24,
        length_bits: 8,
        name: "Default control mode in OpStateAuto",
        data_type: DataType::Enum,
        description: "Default control mode in OpStateAuto. Possible values are in Table 9",
        scale: None,
        unit: None,
        enum_table: Some("table_9"),
    },
];

/// Telecommand input for Default Mode Configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct DefaultModeConfigurationCommand {
    /// Default ADCS run mode. Default ADCS run mode. Possible values are in Table 26
    pub default_adcs_run_mode: u8,
    /// Default ADCS operational state. Default ADCS operational state. Possible values are in Table 36
    pub default_adcs_operational_state: u8,
    /// Default control mode in OpStateSafe. Default control mode in OpStateSafe. Possible values are in Table 9
    pub default_control_mode_in_opstatesafe: u8,
    /// Default control mode in OpStateAuto. Default control mode in OpStateAuto. Possible values are in Table 9
    pub default_control_mode_in_opstateauto: u8,
}

impl Telecommand for DefaultModeConfigurationCommand {
    const ID: u8 = 64;
    const NAME: &'static str = "Default Mode Configuration";
    const LENGTH_BYTES: usize = 4;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Default ADCS run mode",
            u128::from(self.default_adcs_run_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "Default ADCS operational state",
            u128::from(self.default_adcs_operational_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            8,
            "Default control mode in OpStateSafe",
            u128::from(self.default_control_mode_in_opstatesafe),
        )?;
        codec::write_unsigned(
            &mut payload,
            24,
            8,
            "Default control mode in OpStateAuto",
            u128::from(self.default_control_mode_in_opstateauto),
        )?;
        Ok(payload)
    }
}

const MOUNTING_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "StackX mounting",
        data_type: DataType::Enum,
        description: "StackX mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "StackY mounting",
        data_type: DataType::Enum,
        description: "StackY mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 8,
        name: "StackZ mounting",
        data_type: DataType::Enum,
        description: "StackZ mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 24,
        length_bits: 8,
        name: "MTQ0 mounting",
        data_type: DataType::Enum,
        description: "MTQ0 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 8,
        name: "MTQ1 mounting",
        data_type: DataType::Enum,
        description: "MTQ1 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 40,
        length_bits: 8,
        name: "MTQ2 mounting",
        data_type: DataType::Enum,
        description: "MTQ2 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 8,
        name: "Wheel0 mounting",
        data_type: DataType::Enum,
        description: "Wheel0 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 56,
        length_bits: 8,
        name: "Wheel1 mounting",
        data_type: DataType::Enum,
        description: "Wheel1 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 8,
        name: "Wheel2 mounting",
        data_type: DataType::Enum,
        description: "Wheel2 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 72,
        length_bits: 8,
        name: "Wheel3 mounting",
        data_type: DataType::Enum,
        description: "Wheel3 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 80,
        length_bits: 16,
        name: "Pyramid RWL mounting alpha angle",
        data_type: DataType::Int,
        description: "Pyramid RWL mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 16,
        name: "Pyramid RWL mounting beta angle",
        data_type: DataType::Int,
        description: "Pyramid RWL mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 112,
        length_bits: 16,
        name: "Pyramid RWL mounting gamma angle",
        data_type: DataType::Int,
        description: "Pyramid RWL mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 8,
        name: "CSS0 mounting",
        data_type: DataType::Enum,
        description: "CSS0 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 136,
        length_bits: 8,
        name: "CSS1 mounting",
        data_type: DataType::Enum,
        description: "CSS1 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 144,
        length_bits: 8,
        name: "CSS2 mounting",
        data_type: DataType::Enum,
        description: "CSS2 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 152,
        length_bits: 8,
        name: "CSS3 mounting",
        data_type: DataType::Enum,
        description: "CSS3 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 160,
        length_bits: 8,
        name: "CSS4 mounting",
        data_type: DataType::Enum,
        description: "CSS4 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 168,
        length_bits: 8,
        name: "CSS5 mounting",
        data_type: DataType::Enum,
        description: "CSS5 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 176,
        length_bits: 8,
        name: "CSS6 mounting",
        data_type: DataType::Enum,
        description: "CSS6 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 184,
        length_bits: 8,
        name: "CSS7 mounting",
        data_type: DataType::Enum,
        description: "CSS7 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 192,
        length_bits: 8,
        name: "CSS8 mounting",
        data_type: DataType::Enum,
        description: "CSS8 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 200,
        length_bits: 8,
        name: "CSS9 mounting",
        data_type: DataType::Enum,
        description: "CSS9 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 208,
        length_bits: 16,
        name: "FSS0 mounting alpha angle",
        data_type: DataType::Int,
        description: "FSS0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 224,
        length_bits: 16,
        name: "FSS0 mounting beta angle",
        data_type: DataType::Int,
        description: "FSS0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 240,
        length_bits: 16,
        name: "FSS0 mounting gamma angle",
        data_type: DataType::Int,
        description: "FSS0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 256,
        length_bits: 16,
        name: "FSS1 mounting alpha angle",
        data_type: DataType::Int,
        description: "FSS1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 272,
        length_bits: 16,
        name: "FSS1 mounting beta angle",
        data_type: DataType::Int,
        description: "FSS1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 288,
        length_bits: 16,
        name: "FSS1 mounting gamma angle",
        data_type: DataType::Int,
        description: "FSS1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 304,
        length_bits: 16,
        name: "FSS2 mounting alpha angle",
        data_type: DataType::Int,
        description: "FSS2 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 320,
        length_bits: 16,
        name: "FSS2 mounting beta angle",
        data_type: DataType::Int,
        description: "FSS2 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 336,
        length_bits: 16,
        name: "FSS2 mounting gamma angle",
        data_type: DataType::Int,
        description: "FSS2 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 352,
        length_bits: 16,
        name: "FSS3 mounting alpha angle",
        data_type: DataType::Int,
        description: "FSS3 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 368,
        length_bits: 16,
        name: "FSS3 mounting beta angle",
        data_type: DataType::Int,
        description: "FSS3 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 384,
        length_bits: 16,
        name: "FSS3 mounting gamma angle",
        data_type: DataType::Int,
        description: "FSS3 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 400,
        length_bits: 16,
        name: "HSS0 mounting alpha angle",
        data_type: DataType::Int,
        description: "HSS0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 416,
        length_bits: 16,
        name: "HSS0 mounting beta angle",
        data_type: DataType::Int,
        description: "HSS0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 432,
        length_bits: 16,
        name: "HSS0 mounting gamma angle",
        data_type: DataType::Int,
        description: "HSS0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 448,
        length_bits: 16,
        name: "HSS1 mounting alpha angle",
        data_type: DataType::Int,
        description: "HSS1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 464,
        length_bits: 16,
        name: "HSS1 mounting beta angle",
        data_type: DataType::Int,
        description: "HSS1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 480,
        length_bits: 16,
        name: "HSS1 mounting gamma angle",
        data_type: DataType::Int,
        description: "HSS1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 496,
        length_bits: 16,
        name: "MAG0 mounting alpha angle",
        data_type: DataType::Int,
        description: "MAG0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 512,
        length_bits: 16,
        name: "MAG0 mounting beta angle",
        data_type: DataType::Int,
        description: "MAG0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 528,
        length_bits: 16,
        name: "MAG0 mounting gamma angle",
        data_type: DataType::Int,
        description: "MAG0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 544,
        length_bits: 16,
        name: "MAG1 mounting alpha angle",
        data_type: DataType::Int,
        description: "MAG1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 560,
        length_bits: 16,
        name: "MAG1 mounting beta angle",
        data_type: DataType::Int,
        description: "MAG1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 576,
        length_bits: 16,
        name: "MAG1 mounting gamma angle",
        data_type: DataType::Int,
        description: "MAG1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 592,
        length_bits: 16,
        name: "STR0 mounting alpha angle",
        data_type: DataType::Int,
        description: "STR0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 608,
        length_bits: 16,
        name: "STR0 mounting beta angle",
        data_type: DataType::Int,
        description: "STR0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 624,
        length_bits: 16,
        name: "STR0 mounting gamma angle",
        data_type: DataType::Int,
        description: "STR0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 640,
        length_bits: 16,
        name: "STR1 mounting alpha angle",
        data_type: DataType::Int,
        description: "STR1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 656,
        length_bits: 16,
        name: "STR1 mounting beta angle",
        data_type: DataType::Int,
        description: "STR1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 672,
        length_bits: 16,
        name: "STR1 mounting gamma angle",
        data_type: DataType::Int,
        description: "STR1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 688,
        length_bits: 16,
        name: "ExtSensor0 mounting alpha angle",
        data_type: DataType::Int,
        description: "ExtSensor0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 704,
        length_bits: 16,
        name: "ExtSensor0 mounting beta angle",
        data_type: DataType::Int,
        description: "ExtSensor0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 720,
        length_bits: 16,
        name: "ExtSensor0 mounting gamma angle",
        data_type: DataType::Int,
        description: "ExtSensor0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 736,
        length_bits: 16,
        name: "ExtSensor1 mounting alpha angle",
        data_type: DataType::Int,
        description: "ExtSensor1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 752,
        length_bits: 16,
        name: "ExtSensor1 mounting beta angle",
        data_type: DataType::Int,
        description: "ExtSensor1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 768,
        length_bits: 16,
        name: "ExtSensor1 mounting gamma angle",
        data_type: DataType::Int,
        description: "ExtSensor1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 784,
        length_bits: 8,
        name: "ExtGyro0 axis1 mounting",
        data_type: DataType::Enum,
        description: "ExtGyro0 axis1 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 792,
        length_bits: 8,
        name: "ExtGyro0 axis2 mounting",
        data_type: DataType::Enum,
        description: "ExtGyro0 axis2 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 800,
        length_bits: 8,
        name: "ExtGyro0 axis3 mounting",
        data_type: DataType::Enum,
        description: "ExtGyro0 axis3 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 808,
        length_bits: 8,
        name: "ExtGyro1 axis1 mounting",
        data_type: DataType::Enum,
        description: "ExtGyro1 axis1 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 816,
        length_bits: 8,
        name: "ExtGyro1 axis2 mounting",
        data_type: DataType::Enum,
        description: "ExtGyro1 axis2 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
    FieldSpec {
        offset_bits: 824,
        length_bits: 8,
        name: "ExtGyro1 axis3 mounting",
        data_type: DataType::Enum,
        description: "ExtGyro1 axis3 mounting. Possible values are in Table 38",
        scale: None,
        unit: None,
        enum_table: Some("table_38"),
    },
];

/// Telecommand input for Mounting Configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct MountingConfigurationCommand {
    /// StackX mounting. StackX mounting. Possible values are in Table 38
    pub stackx_mounting: u8,
    /// StackY mounting. StackY mounting. Possible values are in Table 38
    pub stacky_mounting: u8,
    /// StackZ mounting. StackZ mounting. Possible values are in Table 38
    pub stackz_mounting: u8,
    /// MTQ0 mounting. MTQ0 mounting. Possible values are in Table 38
    pub mtq0_mounting: u8,
    /// MTQ1 mounting. MTQ1 mounting. Possible values are in Table 38
    pub mtq1_mounting: u8,
    /// MTQ2 mounting. MTQ2 mounting. Possible values are in Table 38
    pub mtq2_mounting: u8,
    /// Wheel0 mounting. Wheel0 mounting. Possible values are in Table 38
    pub wheel0_mounting: u8,
    /// Wheel1 mounting. Wheel1 mounting. Possible values are in Table 38
    pub wheel1_mounting: u8,
    /// Wheel2 mounting. Wheel2 mounting. Possible values are in Table 38
    pub wheel2_mounting: u8,
    /// Wheel3 mounting. Wheel3 mounting. Possible values are in Table 38
    pub wheel3_mounting: u8,
    /// Pyramid RWL mounting alpha angle. Pyramid RWL mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub pyramid_rwl_mounting_alpha_angle: f64,
    /// Pyramid RWL mounting beta angle. Pyramid RWL mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub pyramid_rwl_mounting_beta_angle: f64,
    /// Pyramid RWL mounting gamma angle. Pyramid RWL mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub pyramid_rwl_mounting_gamma_angle: f64,
    /// CSS0 mounting. CSS0 mounting. Possible values are in Table 38
    pub css0_mounting: u8,
    /// CSS1 mounting. CSS1 mounting. Possible values are in Table 38
    pub css1_mounting: u8,
    /// CSS2 mounting. CSS2 mounting. Possible values are in Table 38
    pub css2_mounting: u8,
    /// CSS3 mounting. CSS3 mounting. Possible values are in Table 38
    pub css3_mounting: u8,
    /// CSS4 mounting. CSS4 mounting. Possible values are in Table 38
    pub css4_mounting: u8,
    /// CSS5 mounting. CSS5 mounting. Possible values are in Table 38
    pub css5_mounting: u8,
    /// CSS6 mounting. CSS6 mounting. Possible values are in Table 38
    pub css6_mounting: u8,
    /// CSS7 mounting. CSS7 mounting. Possible values are in Table 38
    pub css7_mounting: u8,
    /// CSS8 mounting. CSS8 mounting. Possible values are in Table 38
    pub css8_mounting: u8,
    /// CSS9 mounting. CSS9 mounting. Possible values are in Table 38
    pub css9_mounting: u8,
    /// FSS0 mounting alpha angle. FSS0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss0_mounting_alpha_angle: f64,
    /// FSS0 mounting beta angle. FSS0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss0_mounting_beta_angle: f64,
    /// FSS0 mounting gamma angle. FSS0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss0_mounting_gamma_angle: f64,
    /// FSS1 mounting alpha angle. FSS1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss1_mounting_alpha_angle: f64,
    /// FSS1 mounting beta angle. FSS1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss1_mounting_beta_angle: f64,
    /// FSS1 mounting gamma angle. FSS1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss1_mounting_gamma_angle: f64,
    /// FSS2 mounting alpha angle. FSS2 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss2_mounting_alpha_angle: f64,
    /// FSS2 mounting beta angle. FSS2 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss2_mounting_beta_angle: f64,
    /// FSS2 mounting gamma angle. FSS2 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss2_mounting_gamma_angle: f64,
    /// FSS3 mounting alpha angle. FSS3 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss3_mounting_alpha_angle: f64,
    /// FSS3 mounting beta angle. FSS3 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss3_mounting_beta_angle: f64,
    /// FSS3 mounting gamma angle. FSS3 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub fss3_mounting_gamma_angle: f64,
    /// HSS0 mounting alpha angle. HSS0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub hss0_mounting_alpha_angle: f64,
    /// HSS0 mounting beta angle. HSS0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub hss0_mounting_beta_angle: f64,
    /// HSS0 mounting gamma angle. HSS0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub hss0_mounting_gamma_angle: f64,
    /// HSS1 mounting alpha angle. HSS1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub hss1_mounting_alpha_angle: f64,
    /// HSS1 mounting beta angle. HSS1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub hss1_mounting_beta_angle: f64,
    /// HSS1 mounting gamma angle. HSS1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub hss1_mounting_gamma_angle: f64,
    /// MAG0 mounting alpha angle. MAG0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub mag0_mounting_alpha_angle: f64,
    /// MAG0 mounting beta angle. MAG0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub mag0_mounting_beta_angle: f64,
    /// MAG0 mounting gamma angle. MAG0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub mag0_mounting_gamma_angle: f64,
    /// MAG1 mounting alpha angle. MAG1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub mag1_mounting_alpha_angle: f64,
    /// MAG1 mounting beta angle. MAG1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub mag1_mounting_beta_angle: f64,
    /// MAG1 mounting gamma angle. MAG1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub mag1_mounting_gamma_angle: f64,
    /// STR0 mounting alpha angle. STR0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub str0_mounting_alpha_angle: f64,
    /// STR0 mounting beta angle. STR0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub str0_mounting_beta_angle: f64,
    /// STR0 mounting gamma angle. STR0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub str0_mounting_gamma_angle: f64,
    /// STR1 mounting alpha angle. STR1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub str1_mounting_alpha_angle: f64,
    /// STR1 mounting beta angle. STR1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub str1_mounting_beta_angle: f64,
    /// STR1 mounting gamma angle. STR1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub str1_mounting_gamma_angle: f64,
    /// ExtSensor0 mounting alpha angle. ExtSensor0 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub extsensor0_mounting_alpha_angle: f64,
    /// ExtSensor0 mounting beta angle. ExtSensor0 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub extsensor0_mounting_beta_angle: f64,
    /// ExtSensor0 mounting gamma angle. ExtSensor0 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub extsensor0_mounting_gamma_angle: f64,
    /// ExtSensor1 mounting alpha angle. ExtSensor1 mounting alpha angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub extsensor1_mounting_alpha_angle: f64,
    /// ExtSensor1 mounting beta angle. ExtSensor1 mounting beta angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub extsensor1_mounting_beta_angle: f64,
    /// ExtSensor1 mounting gamma angle. ExtSensor1 mounting gamma angle. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*100.0 (formatted value is in [deg] units)
    pub extsensor1_mounting_gamma_angle: f64,
    /// ExtGyro0 axis1 mounting. ExtGyro0 axis1 mounting. Possible values are in Table 38
    pub extgyro0_axis1_mounting: u8,
    /// ExtGyro0 axis2 mounting. ExtGyro0 axis2 mounting. Possible values are in Table 38
    pub extgyro0_axis2_mounting: u8,
    /// ExtGyro0 axis3 mounting. ExtGyro0 axis3 mounting. Possible values are in Table 38
    pub extgyro0_axis3_mounting: u8,
    /// ExtGyro1 axis1 mounting. ExtGyro1 axis1 mounting. Possible values are in Table 38
    pub extgyro1_axis1_mounting: u8,
    /// ExtGyro1 axis2 mounting. ExtGyro1 axis2 mounting. Possible values are in Table 38
    pub extgyro1_axis2_mounting: u8,
    /// ExtGyro1 axis3 mounting. ExtGyro1 axis3 mounting. Possible values are in Table 38
    pub extgyro1_axis3_mounting: u8,
}

impl Telecommand for MountingConfigurationCommand {
    const ID: u8 = 65;
    const NAME: &'static str = "Mounting Configuration";
    const LENGTH_BYTES: usize = 104;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "StackX mounting",
            u128::from(self.stackx_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "StackY mounting",
            u128::from(self.stacky_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            8,
            "StackZ mounting",
            u128::from(self.stackz_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            24,
            8,
            "MTQ0 mounting",
            u128::from(self.mtq0_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            8,
            "MTQ1 mounting",
            u128::from(self.mtq1_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            40,
            8,
            "MTQ2 mounting",
            u128::from(self.mtq2_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            48,
            8,
            "Wheel0 mounting",
            u128::from(self.wheel0_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            56,
            8,
            "Wheel1 mounting",
            u128::from(self.wheel1_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            8,
            "Wheel2 mounting",
            u128::from(self.wheel2_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            72,
            8,
            "Wheel3 mounting",
            u128::from(self.wheel3_mounting),
        )?;
        codec::write_signed(
            &mut payload,
            80,
            16,
            "Pyramid RWL mounting alpha angle",
            (self.pyramid_rwl_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            96,
            16,
            "Pyramid RWL mounting beta angle",
            (self.pyramid_rwl_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            112,
            16,
            "Pyramid RWL mounting gamma angle",
            (self.pyramid_rwl_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_unsigned(
            &mut payload,
            128,
            8,
            "CSS0 mounting",
            u128::from(self.css0_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            136,
            8,
            "CSS1 mounting",
            u128::from(self.css1_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            144,
            8,
            "CSS2 mounting",
            u128::from(self.css2_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            152,
            8,
            "CSS3 mounting",
            u128::from(self.css3_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            160,
            8,
            "CSS4 mounting",
            u128::from(self.css4_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            168,
            8,
            "CSS5 mounting",
            u128::from(self.css5_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            176,
            8,
            "CSS6 mounting",
            u128::from(self.css6_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            184,
            8,
            "CSS7 mounting",
            u128::from(self.css7_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            192,
            8,
            "CSS8 mounting",
            u128::from(self.css8_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            200,
            8,
            "CSS9 mounting",
            u128::from(self.css9_mounting),
        )?;
        codec::write_signed(
            &mut payload,
            208,
            16,
            "FSS0 mounting alpha angle",
            (self.fss0_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            224,
            16,
            "FSS0 mounting beta angle",
            (self.fss0_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            240,
            16,
            "FSS0 mounting gamma angle",
            (self.fss0_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            256,
            16,
            "FSS1 mounting alpha angle",
            (self.fss1_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            272,
            16,
            "FSS1 mounting beta angle",
            (self.fss1_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            288,
            16,
            "FSS1 mounting gamma angle",
            (self.fss1_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            304,
            16,
            "FSS2 mounting alpha angle",
            (self.fss2_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            320,
            16,
            "FSS2 mounting beta angle",
            (self.fss2_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            336,
            16,
            "FSS2 mounting gamma angle",
            (self.fss2_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            352,
            16,
            "FSS3 mounting alpha angle",
            (self.fss3_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            368,
            16,
            "FSS3 mounting beta angle",
            (self.fss3_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            384,
            16,
            "FSS3 mounting gamma angle",
            (self.fss3_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            400,
            16,
            "HSS0 mounting alpha angle",
            (self.hss0_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            416,
            16,
            "HSS0 mounting beta angle",
            (self.hss0_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            432,
            16,
            "HSS0 mounting gamma angle",
            (self.hss0_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            448,
            16,
            "HSS1 mounting alpha angle",
            (self.hss1_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            464,
            16,
            "HSS1 mounting beta angle",
            (self.hss1_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            480,
            16,
            "HSS1 mounting gamma angle",
            (self.hss1_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            496,
            16,
            "MAG0 mounting alpha angle",
            (self.mag0_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            512,
            16,
            "MAG0 mounting beta angle",
            (self.mag0_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            528,
            16,
            "MAG0 mounting gamma angle",
            (self.mag0_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            544,
            16,
            "MAG1 mounting alpha angle",
            (self.mag1_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            560,
            16,
            "MAG1 mounting beta angle",
            (self.mag1_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            576,
            16,
            "MAG1 mounting gamma angle",
            (self.mag1_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            592,
            16,
            "STR0 mounting alpha angle",
            (self.str0_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            608,
            16,
            "STR0 mounting beta angle",
            (self.str0_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            624,
            16,
            "STR0 mounting gamma angle",
            (self.str0_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            640,
            16,
            "STR1 mounting alpha angle",
            (self.str1_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            656,
            16,
            "STR1 mounting beta angle",
            (self.str1_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            672,
            16,
            "STR1 mounting gamma angle",
            (self.str1_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            688,
            16,
            "ExtSensor0 mounting alpha angle",
            (self.extsensor0_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            704,
            16,
            "ExtSensor0 mounting beta angle",
            (self.extsensor0_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            720,
            16,
            "ExtSensor0 mounting gamma angle",
            (self.extsensor0_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            736,
            16,
            "ExtSensor1 mounting alpha angle",
            (self.extsensor1_mounting_alpha_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            752,
            16,
            "ExtSensor1 mounting beta angle",
            (self.extsensor1_mounting_beta_angle * 100.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            768,
            16,
            "ExtSensor1 mounting gamma angle",
            (self.extsensor1_mounting_gamma_angle * 100.0).round() as i128,
        )?;
        codec::write_unsigned(
            &mut payload,
            784,
            8,
            "ExtGyro0 axis1 mounting",
            u128::from(self.extgyro0_axis1_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            792,
            8,
            "ExtGyro0 axis2 mounting",
            u128::from(self.extgyro0_axis2_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            800,
            8,
            "ExtGyro0 axis3 mounting",
            u128::from(self.extgyro0_axis3_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            808,
            8,
            "ExtGyro1 axis1 mounting",
            u128::from(self.extgyro1_axis1_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            816,
            8,
            "ExtGyro1 axis2 mounting",
            u128::from(self.extgyro1_axis2_mounting),
        )?;
        codec::write_unsigned(
            &mut payload,
            824,
            8,
            "ExtGyro1 axis3 mounting",
            u128::from(self.extgyro1_axis3_mounting),
        )?;
        Ok(payload)
    }
}

const MAG1_ORBIT_CALIBRATION_CONFIG_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 16,
        name: "Magnetometer channel 1 offset",
        data_type: DataType::Int,
        description: "Magnetometer channel 1 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 16,
        name: "Magnetometer channel 2 offset",
        data_type: DataType::Int,
        description: "Magnetometer channel 2 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 16,
        name: "Magnetometer channel 3 offset",
        data_type: DataType::Int,
        description: "Magnetometer channel 3 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S11",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S11. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S22",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S22. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 80,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S33",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S33. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S12",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S12. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 112,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S13",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S13. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S21",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S21. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 144,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S23",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S23. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 160,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S31",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S31. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 176,
        length_bits: 16,
        name: "Magnetometer sensitivity matrix S32",
        data_type: DataType::Int,
        description: "Magnetometer sensitivity matrix S32. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Mag1 Orbit Calibration Config.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct Mag1OrbitCalibrationConfigCommand {
    /// Magnetometer channel 1 offset. Magnetometer channel 1 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_channel_1_offset: f64,
    /// Magnetometer channel 2 offset. Magnetometer channel 2 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_channel_2_offset: f64,
    /// Magnetometer channel 3 offset. Magnetometer channel 3 offset. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_channel_3_offset: f64,
    /// Magnetometer sensitivity matrix S11. Magnetometer sensitivity matrix S11. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s11: f64,
    /// Magnetometer sensitivity matrix S22. Magnetometer sensitivity matrix S22. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s22: f64,
    /// Magnetometer sensitivity matrix S33. Magnetometer sensitivity matrix S33. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s33: f64,
    /// Magnetometer sensitivity matrix S12. Magnetometer sensitivity matrix S12. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s12: f64,
    /// Magnetometer sensitivity matrix S13. Magnetometer sensitivity matrix S13. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s13: f64,
    /// Magnetometer sensitivity matrix S21. Magnetometer sensitivity matrix S21. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s21: f64,
    /// Magnetometer sensitivity matrix S23. Magnetometer sensitivity matrix S23. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s23: f64,
    /// Magnetometer sensitivity matrix S31. Magnetometer sensitivity matrix S31. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s31: f64,
    /// Magnetometer sensitivity matrix S32. Magnetometer sensitivity matrix S32. Raw parameter value is obtained using the formula: (raw parameter) = (formatted value)*1000.0
    pub magnetometer_sensitivity_matrix_s32: f64,
}

impl Telecommand for Mag1OrbitCalibrationConfigCommand {
    const ID: u8 = 66;
    const NAME: &'static str = "Mag1 Orbit Calibration Config";
    const LENGTH_BYTES: usize = 24;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_signed(
            &mut payload,
            0,
            16,
            "Magnetometer channel 1 offset",
            (self.magnetometer_channel_1_offset * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            16,
            16,
            "Magnetometer channel 2 offset",
            (self.magnetometer_channel_2_offset * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            32,
            16,
            "Magnetometer channel 3 offset",
            (self.magnetometer_channel_3_offset * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            48,
            16,
            "Magnetometer sensitivity matrix S11",
            (self.magnetometer_sensitivity_matrix_s11 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            64,
            16,
            "Magnetometer sensitivity matrix S22",
            (self.magnetometer_sensitivity_matrix_s22 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            80,
            16,
            "Magnetometer sensitivity matrix S33",
            (self.magnetometer_sensitivity_matrix_s33 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            96,
            16,
            "Magnetometer sensitivity matrix S12",
            (self.magnetometer_sensitivity_matrix_s12 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            112,
            16,
            "Magnetometer sensitivity matrix S13",
            (self.magnetometer_sensitivity_matrix_s13 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            128,
            16,
            "Magnetometer sensitivity matrix S21",
            (self.magnetometer_sensitivity_matrix_s21 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            144,
            16,
            "Magnetometer sensitivity matrix S23",
            (self.magnetometer_sensitivity_matrix_s23 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            160,
            16,
            "Magnetometer sensitivity matrix S31",
            (self.magnetometer_sensitivity_matrix_s31 * 1000.0).round() as i128,
        )?;
        codec::write_signed(
            &mut payload,
            176,
            16,
            "Magnetometer sensitivity matrix S32",
            (self.magnetometer_sensitivity_matrix_s32 * 1000.0).round() as i128,
        )?;
        Ok(payload)
    }
}

const ADCS_ESTIMATOR_CONFIG_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Default main estimator mode",
        data_type: DataType::Enum,
        description: "Default main estimator mode. Possible values are in Table 10",
        scale: None,
        unit: None,
        enum_table: Some("table_10"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "Default backup estimator mode",
        data_type: DataType::Enum,
        description: "Default backup estimator mode. Possible values are in Table 10",
        scale: None,
        unit: None,
        enum_table: Some("table_10"),
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 32,
        name: "MAG measurement noise",
        data_type: DataType::Float,
        description: "Magnetometer measurement noise",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 32,
        name: "CSS measurement noise",
        data_type: DataType::Float,
        description: "Coarse sun sensor measurement noise",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 80,
        length_bits: 32,
        name: "FSS measurement noise",
        data_type: DataType::Float,
        description: "Fine sun sensor measurement noise",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 112,
        length_bits: 32,
        name: "HSS measurement noise",
        data_type: DataType::Float,
        description: "Horizon sensor measurement noise",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 144,
        length_bits: 32,
        name: "STR measurement noise",
        data_type: DataType::Float,
        description: "Star tracker measurement noise",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 176,
        length_bits: 32,
        name: "Magnetometer RKF system noise",
        data_type: DataType::Float,
        description: "Magnetometer RKF system noise",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 208,
        length_bits: 32,
        name: "EKF system noise",
        data_type: DataType::Float,
        description: "EKF system noise",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 240,
        length_bits: 32,
        name: "Nutation Epsilon correction",
        data_type: DataType::Float,
        description: "Polar nutation Epsilon correction. (Unit of measure is [rad])",
        scale: None,
        unit: Some("rad"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 272,
        length_bits: 32,
        name: "Nutation Psi correction",
        data_type: DataType::Float,
        description: "Polar nutation Psi correction. (Unit of measure is [rad])",
        scale: None,
        unit: Some("rad"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 304,
        length_bits: 1,
        name: "Use FSS in EKF",
        data_type: DataType::Bool,
        description: "Use fine sun sensor measurements in EKF",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 305,
        length_bits: 1,
        name: "Use CSS in EKF",
        data_type: DataType::Bool,
        description: "Use coarse sun sensor measurements in EKF",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 306,
        length_bits: 1,
        name: "Use HSS in EKF",
        data_type: DataType::Bool,
        description: "Use horizon sensor measurements in EKF",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 307,
        length_bits: 1,
        name: "Use STR in EKF",
        data_type: DataType::Bool,
        description: "Use star tracker measurements in EKF",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 308,
        length_bits: 4,
        name: "Triad Vector 1",
        data_type: DataType::Enum,
        description: "Vector 1 selection for Triad. Possible values are in Table 41",
        scale: None,
        unit: None,
        enum_table: Some("table_41"),
    },
    FieldSpec {
        offset_bits: 312,
        length_bits: 4,
        name: "Triad Vector 2",
        data_type: DataType::Enum,
        description: "Vector 2 selection for Triad. Possible values are in Table 41",
        scale: None,
        unit: None,
        enum_table: Some("table_41"),
    },
];

/// Telecommand input for ADCS Estimator Config.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct AdcsEstimatorConfigCommand {
    /// Default main estimator mode. Default main estimator mode. Possible values are in Table 10
    pub default_main_estimator_mode: u8,
    /// Default backup estimator mode. Default backup estimator mode. Possible values are in Table 10
    pub default_backup_estimator_mode: u8,
    /// MAG measurement noise. Magnetometer measurement noise
    pub mag_measurement_noise: f32,
    /// CSS measurement noise. Coarse sun sensor measurement noise
    pub css_measurement_noise: f32,
    /// FSS measurement noise. Fine sun sensor measurement noise
    pub fss_measurement_noise: f32,
    /// HSS measurement noise. Horizon sensor measurement noise
    pub hss_measurement_noise: f32,
    /// STR measurement noise. Star tracker measurement noise
    pub str_measurement_noise: f32,
    /// Magnetometer RKF system noise. Magnetometer RKF system noise
    pub magnetometer_rkf_system_noise: f32,
    /// EKF system noise. EKF system noise
    pub ekf_system_noise: f32,
    /// Nutation Epsilon correction. Polar nutation Epsilon correction. (Unit of measure is [rad])
    pub nutation_epsilon_correction: f32,
    /// Nutation Psi correction. Polar nutation Psi correction. (Unit of measure is [rad])
    pub nutation_psi_correction: f32,
    /// Use FSS in EKF. Use fine sun sensor measurements in EKF
    pub use_fss_in_ekf: bool,
    /// Use CSS in EKF. Use coarse sun sensor measurements in EKF
    pub use_css_in_ekf: bool,
    /// Use HSS in EKF. Use horizon sensor measurements in EKF
    pub use_hss_in_ekf: bool,
    /// Use STR in EKF. Use star tracker measurements in EKF
    pub use_str_in_ekf: bool,
    /// Triad Vector 1. Vector 1 selection for Triad. Possible values are in Table 41
    pub triad_vector_1: u8,
    /// Triad Vector 2. Vector 2 selection for Triad. Possible values are in Table 41
    pub triad_vector_2: u8,
}

impl Telecommand for AdcsEstimatorConfigCommand {
    const ID: u8 = 67;
    const NAME: &'static str = "ADCS Estimator Config";
    const LENGTH_BYTES: usize = 40;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Default main estimator mode",
            u128::from(self.default_main_estimator_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "Default backup estimator mode",
            u128::from(self.default_backup_estimator_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            32,
            "MAG measurement noise",
            u128::from(self.mag_measurement_noise.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            48,
            32,
            "CSS measurement noise",
            u128::from(self.css_measurement_noise.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            80,
            32,
            "FSS measurement noise",
            u128::from(self.fss_measurement_noise.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            112,
            32,
            "HSS measurement noise",
            u128::from(self.hss_measurement_noise.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            144,
            32,
            "STR measurement noise",
            u128::from(self.str_measurement_noise.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            176,
            32,
            "Magnetometer RKF system noise",
            u128::from(self.magnetometer_rkf_system_noise.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            208,
            32,
            "EKF system noise",
            u128::from(self.ekf_system_noise.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            240,
            32,
            "Nutation Epsilon correction",
            u128::from(self.nutation_epsilon_correction.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            272,
            32,
            "Nutation Psi correction",
            u128::from(self.nutation_psi_correction.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            304,
            1,
            "Use FSS in EKF",
            if self.use_fss_in_ekf { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            305,
            1,
            "Use CSS in EKF",
            if self.use_css_in_ekf { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            306,
            1,
            "Use HSS in EKF",
            if self.use_hss_in_ekf { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            307,
            1,
            "Use STR in EKF",
            if self.use_str_in_ekf { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            308,
            4,
            "Triad Vector 1",
            u128::from(self.triad_vector_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            312,
            4,
            "Triad Vector 2",
            u128::from(self.triad_vector_2),
        )?;
        Ok(payload)
    }
}

const SATELLITE_ORBIT_PARAMETERS_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 64,
        name: "Orbit epoch",
        data_type: DataType::Double,
        description: "Orbit epoch. (Unit of measure is [yyddd.ssssssss])",
        scale: None,
        unit: Some("yyddd.ssssssss"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 64,
        name: "Orbit inclination",
        data_type: DataType::Double,
        description: "Orbit inclination. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 64,
        name: "Orbit RAAN",
        data_type: DataType::Double,
        description: "Orbit RAAN. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 192,
        length_bits: 64,
        name: "Orbit eccentricity",
        data_type: DataType::Double,
        description: "Orbit eccentricity",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 256,
        length_bits: 64,
        name: "Orbit argument of perigee",
        data_type: DataType::Double,
        description: "Orbit argument of perigee. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 320,
        length_bits: 64,
        name: "Orbit mean anomaly",
        data_type: DataType::Double,
        description: "Orbit mean anomaly. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 384,
        length_bits: 64,
        name: "Orbit mean motion",
        data_type: DataType::Double,
        description: "Orbit mean motion. (Unit of measure is [orbits/day])",
        scale: None,
        unit: Some("orbits/day"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 448,
        length_bits: 64,
        name: "Orbit B-star drag term",
        data_type: DataType::Double,
        description: "Orbit B-star drag term",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Satellite Orbit Parameters.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct SatelliteOrbitParametersCommand {
    /// Orbit epoch. Orbit epoch. (Unit of measure is [yyddd.ssssssss])
    pub orbit_epoch: f64,
    /// Orbit inclination. Orbit inclination. (Unit of measure is [deg])
    pub orbit_inclination: f64,
    /// Orbit RAAN. Orbit RAAN. (Unit of measure is [deg])
    pub orbit_raan: f64,
    /// Orbit eccentricity. Orbit eccentricity
    pub orbit_eccentricity: f64,
    /// Orbit argument of perigee. Orbit argument of perigee. (Unit of measure is [deg])
    pub orbit_argument_of_perigee: f64,
    /// Orbit mean anomaly. Orbit mean anomaly. (Unit of measure is [deg])
    pub orbit_mean_anomaly: f64,
    /// Orbit mean motion. Orbit mean motion. (Unit of measure is [orbits/day])
    pub orbit_mean_motion: f64,
    /// Orbit B-star drag term. Orbit B-star drag term
    pub orbit_b_star_drag_term: f64,
}

impl Telecommand for SatelliteOrbitParametersCommand {
    const ID: u8 = 68;
    const NAME: &'static str = "Satellite Orbit Parameters";
    const LENGTH_BYTES: usize = 64;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            64,
            "Orbit epoch",
            u128::from(self.orbit_epoch.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            64,
            "Orbit inclination",
            u128::from(self.orbit_inclination.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            128,
            64,
            "Orbit RAAN",
            u128::from(self.orbit_raan.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            192,
            64,
            "Orbit eccentricity",
            u128::from(self.orbit_eccentricity.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            256,
            64,
            "Orbit argument of perigee",
            u128::from(self.orbit_argument_of_perigee.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            320,
            64,
            "Orbit mean anomaly",
            u128::from(self.orbit_mean_anomaly.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            384,
            64,
            "Orbit mean motion",
            u128::from(self.orbit_mean_motion.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            448,
            64,
            "Orbit B-star drag term",
            u128::from(self.orbit_b_star_drag_term.to_bits()),
        )?;
        Ok(payload)
    }
}

const NODE_SELECTION_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "RWL selection flags",
        data_type: DataType::Uint,
        description: "RWL selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "MAG selection flags",
        data_type: DataType::Uint,
        description: "MAG selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 8,
        name: "FSS selection flags",
        data_type: DataType::Uint,
        description: "FSS selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 24,
        length_bits: 8,
        name: "HSS selection flags",
        data_type: DataType::Uint,
        description: "HSS selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 8,
        name: "GYR selection flags",
        data_type: DataType::Uint,
        description: "GYR selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 40,
        length_bits: 8,
        name: "STR selection flags",
        data_type: DataType::Uint,
        description: "STR selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 8,
        name: "GNSS selection flags",
        data_type: DataType::Uint,
        description: "GNSS selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 56,
        length_bits: 8,
        name: "External sensor selection flags",
        data_type: DataType::Uint,
        description: "External sensor selection flags",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Node Selection Configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct NodeSelectionConfigurationCommand {
    /// RWL selection flags. RWL selection flags
    pub rwl_selection_flags: u8,
    /// MAG selection flags. MAG selection flags
    pub mag_selection_flags: u8,
    /// FSS selection flags. FSS selection flags
    pub fss_selection_flags: u8,
    /// HSS selection flags. HSS selection flags
    pub hss_selection_flags: u8,
    /// GYR selection flags. GYR selection flags
    pub gyr_selection_flags: u8,
    /// STR selection flags. STR selection flags
    pub str_selection_flags: u8,
    /// GNSS selection flags. GNSS selection flags
    pub gnss_selection_flags: u8,
    /// External sensor selection flags. External sensor selection flags
    pub external_sensor_selection_flags: u8,
}

impl Telecommand for NodeSelectionConfigurationCommand {
    const ID: u8 = 69;
    const NAME: &'static str = "Node Selection Configuration";
    const LENGTH_BYTES: usize = 8;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "RWL selection flags",
            u128::from(self.rwl_selection_flags),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "MAG selection flags",
            u128::from(self.mag_selection_flags),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            8,
            "FSS selection flags",
            u128::from(self.fss_selection_flags),
        )?;
        codec::write_unsigned(
            &mut payload,
            24,
            8,
            "HSS selection flags",
            u128::from(self.hss_selection_flags),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            8,
            "GYR selection flags",
            u128::from(self.gyr_selection_flags),
        )?;
        codec::write_unsigned(
            &mut payload,
            40,
            8,
            "STR selection flags",
            u128::from(self.str_selection_flags),
        )?;
        codec::write_unsigned(
            &mut payload,
            48,
            8,
            "GNSS selection flags",
            u128::from(self.gnss_selection_flags),
        )?;
        codec::write_unsigned(
            &mut payload,
            56,
            8,
            "External sensor selection flags",
            u128::from(self.external_sensor_selection_flags),
        )?;
        Ok(payload)
    }
}

const MAGNETORQUER_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "MTQ0 maximum dipole moment",
        data_type: DataType::Float,
        description: "MTQ0 maximum dipole moment. (Unit of measure is [A.m^2])",
        scale: None,
        unit: Some("A.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "MTQ1 maximum dipole moment",
        data_type: DataType::Float,
        description: "MTQ1 maximum dipole moment. (Unit of measure is [A.m^2])",
        scale: None,
        unit: Some("A.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "MTQ2 maximum dipole moment",
        data_type: DataType::Float,
        description: "MTQ2 maximum dipole moment. (Unit of measure is [A.m^2])",
        scale: None,
        unit: Some("A.m^2"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 16,
        name: "Maximum magnetorquer on-time",
        data_type: DataType::Uint,
        description: "Maximum magnetorquer on-time. (Unit of measure is [ms])",
        scale: None,
        unit: Some("ms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 112,
        length_bits: 16,
        name: "Minimum magnetorquer on-time",
        data_type: DataType::Uint,
        description: "Minimum magnetorquer on-time. (Unit of measure is [ms])",
        scale: None,
        unit: Some("ms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 32,
        name: "Magnetic control filter factor",
        data_type: DataType::Float,
        description: "LPF factor for magnetorquer commands. Set to zero for no filtering",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Magnetorquer Configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct MagnetorquerConfigurationCommand {
    /// MTQ0 maximum dipole moment. MTQ0 maximum dipole moment. (Unit of measure is [A.m^2])
    pub mtq0_maximum_dipole_moment: f32,
    /// MTQ1 maximum dipole moment. MTQ1 maximum dipole moment. (Unit of measure is [A.m^2])
    pub mtq1_maximum_dipole_moment: f32,
    /// MTQ2 maximum dipole moment. MTQ2 maximum dipole moment. (Unit of measure is [A.m^2])
    pub mtq2_maximum_dipole_moment: f32,
    /// Maximum magnetorquer on-time. Maximum magnetorquer on-time. (Unit of measure is [ms])
    pub maximum_magnetorquer_on_time: u16,
    /// Minimum magnetorquer on-time. Minimum magnetorquer on-time. (Unit of measure is [ms])
    pub minimum_magnetorquer_on_time: u16,
    /// Magnetic control filter factor. LPF factor for magnetorquer commands. Set to zero for no filtering
    pub magnetic_control_filter_factor: f32,
}

impl Telecommand for MagnetorquerConfigurationCommand {
    const ID: u8 = 70;
    const NAME: &'static str = "Magnetorquer Configuration";
    const LENGTH_BYTES: usize = 20;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "MTQ0 maximum dipole moment",
            u128::from(self.mtq0_maximum_dipole_moment.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "MTQ1 maximum dipole moment",
            u128::from(self.mtq1_maximum_dipole_moment.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            32,
            "MTQ2 maximum dipole moment",
            u128::from(self.mtq2_maximum_dipole_moment.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            96,
            16,
            "Maximum magnetorquer on-time",
            u128::from(self.maximum_magnetorquer_on_time),
        )?;
        codec::write_unsigned(
            &mut payload,
            112,
            16,
            "Minimum magnetorquer on-time",
            u128::from(self.minimum_magnetorquer_on_time),
        )?;
        codec::write_unsigned(
            &mut payload,
            128,
            32,
            "Magnetic control filter factor",
            u128::from(self.magnetic_control_filter_factor.to_bits()),
        )?;
        Ok(payload)
    }
}

const ESTIMATION_MODE_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Main estimator mode",
        data_type: DataType::Enum,
        description: "Main estimator mode. Possible values are in Table 10",
        scale: None,
        unit: None,
        enum_table: Some("table_10"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "Backup estimator mode",
        data_type: DataType::Enum,
        description: "Backup estimator mode. Possible values are in Table 10",
        scale: None,
        unit: None,
        enum_table: Some("table_10"),
    },
];

/// Telecommand input for Estimation Mode.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct EstimationModeCommand {
    /// Main estimator mode. Main estimator mode. Possible values are in Table 10
    pub main_estimator_mode: u8,
    /// Backup estimator mode. Backup estimator mode. Possible values are in Table 10
    pub backup_estimator_mode: u8,
}

impl Telecommand for EstimationModeCommand {
    const ID: u8 = 71;
    const NAME: &'static str = "Estimation Mode";
    const LENGTH_BYTES: usize = 2;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Main estimator mode",
            u128::from(self.main_estimator_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "Backup estimator mode",
            u128::from(self.backup_estimator_mode),
        )?;
        Ok(payload)
    }
}

const OPENLOOPCOMMANDRWL_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "RWL0 open-loop speed command",
        data_type: DataType::Float,
        description: "RWL0 open-loop speed command",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "RWL1 open-loop speed command",
        data_type: DataType::Float,
        description: "RWL1 open-loop speed command",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "RWL2 open-loop speed command",
        data_type: DataType::Float,
        description: "RWL2 open-loop speed command",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 32,
        name: "RWL3 open-loop speed command",
        data_type: DataType::Float,
        description: "RWL3 open-loop speed command",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for OpenLoopCommandRwl.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct OpenloopcommandrwlCommand {
    /// RWL0 open-loop speed command. RWL0 open-loop speed command
    pub rwl0_open_loop_speed_command: f32,
    /// RWL1 open-loop speed command. RWL1 open-loop speed command
    pub rwl1_open_loop_speed_command: f32,
    /// RWL2 open-loop speed command. RWL2 open-loop speed command
    pub rwl2_open_loop_speed_command: f32,
    /// RWL3 open-loop speed command. RWL3 open-loop speed command
    pub rwl3_open_loop_speed_command: f32,
}

impl Telecommand for OpenloopcommandrwlCommand {
    const ID: u8 = 74;
    const NAME: &'static str = "OpenLoopCommandRwl";
    const LENGTH_BYTES: usize = 16;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "RWL0 open-loop speed command",
            u128::from(self.rwl0_open_loop_speed_command.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "RWL1 open-loop speed command",
            u128::from(self.rwl1_open_loop_speed_command.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            32,
            "RWL2 open-loop speed command",
            u128::from(self.rwl2_open_loop_speed_command.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            96,
            32,
            "RWL3 open-loop speed command",
            u128::from(self.rwl3_open_loop_speed_command.to_bits()),
        )?;
        Ok(payload)
    }
}

const OPENLOOPCOMMANDHXYZRW_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 32,
        name: "X-momentum open-loop speed command",
        data_type: DataType::Float,
        description: "X-momentum open-loop speed command. (Unit of measure is [Nms])",
        scale: None,
        unit: Some("Nms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 32,
        length_bits: 32,
        name: "Y-momentum open-loop speed command",
        data_type: DataType::Float,
        description: "Y-momentum open-loop speed command. (Unit of measure is [Nms])",
        scale: None,
        unit: Some("Nms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "Z-momentum open-loop speed command",
        data_type: DataType::Float,
        description: "Z-momentum open-loop speed command. (Unit of measure is [Nms])",
        scale: None,
        unit: Some("Nms"),
        enum_table: None,
    },
];

/// Telecommand input for OpenLoopCommandHxyzRW.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct OpenloopcommandhxyzrwCommand {
    /// X-momentum open-loop speed command. X-momentum open-loop speed command. (Unit of measure is [Nms])
    pub x_momentum_open_loop_speed_command: f32,
    /// Y-momentum open-loop speed command. Y-momentum open-loop speed command. (Unit of measure is [Nms])
    pub y_momentum_open_loop_speed_command: f32,
    /// Z-momentum open-loop speed command. Z-momentum open-loop speed command. (Unit of measure is [Nms])
    pub z_momentum_open_loop_speed_command: f32,
}

impl Telecommand for OpenloopcommandhxyzrwCommand {
    const ID: u8 = 76;
    const NAME: &'static str = "OpenLoopCommandHxyzRW";
    const LENGTH_BYTES: usize = 12;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            32,
            "X-momentum open-loop speed command",
            u128::from(self.x_momentum_open_loop_speed_command.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            32,
            32,
            "Y-momentum open-loop speed command",
            u128::from(self.y_momentum_open_loop_speed_command.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            32,
            "Z-momentum open-loop speed command",
            u128::from(self.z_momentum_open_loop_speed_command.to_bits()),
        )?;
        Ok(payload)
    }
}

const MAG_SENSING_ELEMENT_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 1,
        name: "MAG0 sensing element",
        data_type: DataType::Enum,
        description: "MAG0 sensing element (primary/redundant). Possible values are in Table 51",
        scale: None,
        unit: None,
        enum_table: Some("table_51"),
    },
    FieldSpec {
        offset_bits: 1,
        length_bits: 1,
        name: "MAG1 sensing element",
        data_type: DataType::Enum,
        description: "MAG1 sensing element (primary/redundant). Possible values are in Table 51",
        scale: None,
        unit: None,
        enum_table: Some("table_51"),
    },
];

/// Telecommand input for Mag Sensing Element Configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct MagSensingElementConfigurationCommand {
    /// MAG0 sensing element. MAG0 sensing element (primary/redundant). Possible values are in Table 51
    pub mag0_sensing_element: u8,
    /// MAG1 sensing element. MAG1 sensing element (primary/redundant). Possible values are in Table 51
    pub mag1_sensing_element: u8,
}

impl Telecommand for MagSensingElementConfigurationCommand {
    const ID: u8 = 77;
    const NAME: &'static str = "Mag Sensing Element Configuration";
    const LENGTH_BYTES: usize = 1;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            1,
            "MAG0 sensing element",
            u128::from(self.mag0_sensing_element),
        )?;
        codec::write_unsigned(
            &mut payload,
            1,
            1,
            "MAG1 sensing element",
            u128::from(self.mag1_sensing_element),
        )?;
        Ok(payload)
    }
}

const DATA_FRAME_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 16,
        name: "Frame size",
        data_type: DataType::Uint,
        description: "The effective frame size - number of bytes in FrameBytes populated with data",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 2048,
        name: "Frame bytes",
        data_type: DataType::Array,
        description: "frame bytes",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Data Frame.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct DataFrameCommand {
    /// Frame size. The effective frame size - number of bytes in FrameBytes populated with data
    pub frame_size: u16,
    /// Frame bytes. frame bytes
    pub frame_bytes: Vec<u8>,
}

impl Telecommand for DataFrameCommand {
    const ID: u8 = 78;
    const NAME: &'static str = "Data Frame";
    const LENGTH_BYTES: usize = 258;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            16,
            "Frame size",
            u128::from(self.frame_size),
        )?;
        codec::write_bytes(&mut payload, 16, 2048, "Frame bytes", &self.frame_bytes)?;
        Ok(payload)
    }
}

const REQUEST_TELEMETRY_LOG_TRANSFER_SETUP_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Filter Type",
        data_type: DataType::Enum,
        description: "Filter type to use when reading tlm logs. Possible values are in Table 72",
        scale: None,
        unit: None,
        enum_table: Some("table_72"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 32,
        name: "Unix start time",
        data_type: DataType::Uint,
        description: "Indicate the unix start time in seconds. All entries with a timestamp after this time will be included in the transfer.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 40,
        length_bits: 32,
        name: "Unix end time",
        data_type: DataType::Uint,
        description: "Indicate the unix start end in seconds. All entries with a timestamp before this time will be included in the transfer.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 72,
        length_bits: 32,
        name: "Number of Entries",
        data_type: DataType::Uint,
        description: "If using time next-x or first/last-x or counter next-x filter - indicate the number of entries to transfer.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 104,
        length_bits: 32,
        name: "Write Counter",
        data_type: DataType::Uint,
        description: "If using counter next-x filter - indicate the reference write counter value.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 136,
        length_bits: 4,
        name: "Tlm Log Return Interval",
        data_type: DataType::Enum,
        description: "Indicate how many samples to exclude between transferred samples. Possible values are in Table 64",
        scale: None,
        unit: None,
        enum_table: Some("table_64"),
    },
    FieldSpec {
        offset_bits: 140,
        length_bits: 4,
        name: "Reserved",
        data_type: DataType::Padding,
        description: "Reserved.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 144,
        length_bits: 40,
        name: "Log ID inclusion bitmask",
        data_type: DataType::Array,
        description: "Indicate which log IDs must be included in the transfered Telemetry Log",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Request Telemetry Log Transfer Setup.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct RequestTelemetryLogTransferSetupCommand {
    /// Filter Type. Filter type to use when reading tlm logs. Possible values are in Table 72
    pub filter_type: u8,
    /// Unix start time. Indicate the unix start time in seconds. All entries with a timestamp after this time will be included in the transfer.
    pub unix_start_time: u32,
    /// Unix end time. Indicate the unix start end in seconds. All entries with a timestamp before this time will be included in the transfer.
    pub unix_end_time: u32,
    /// Number of Entries. If using time next-x or first/last-x or counter next-x filter - indicate the number of entries to transfer.
    pub number_of_entries: u32,
    /// Write Counter. If using counter next-x filter - indicate the reference write counter value.
    pub write_counter: u32,
    /// Tlm Log Return Interval. Indicate how many samples to exclude between transferred samples. Possible values are in Table 64
    pub tlm_log_return_interval: u8,
    /// Log ID inclusion bitmask. Indicate which log IDs must be included in the transfered Telemetry Log
    pub log_id_inclusion_bitmask: Vec<u8>,
}

impl Telecommand for RequestTelemetryLogTransferSetupCommand {
    const ID: u8 = 117;
    const NAME: &'static str = "Request Telemetry Log Transfer Setup";
    const LENGTH_BYTES: usize = 23;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Filter Type",
            u128::from(self.filter_type),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            32,
            "Unix start time",
            u128::from(self.unix_start_time),
        )?;
        codec::write_unsigned(
            &mut payload,
            40,
            32,
            "Unix end time",
            u128::from(self.unix_end_time),
        )?;
        codec::write_unsigned(
            &mut payload,
            72,
            32,
            "Number of Entries",
            u128::from(self.number_of_entries),
        )?;
        codec::write_unsigned(
            &mut payload,
            104,
            32,
            "Write Counter",
            u128::from(self.write_counter),
        )?;
        codec::write_unsigned(
            &mut payload,
            136,
            4,
            "Tlm Log Return Interval",
            u128::from(self.tlm_log_return_interval),
        )?;
        codec::write_bytes(
            &mut payload,
            144,
            40,
            "Log ID inclusion bitmask",
            &self.log_id_inclusion_bitmask,
        )?;
        Ok(payload)
    }
}

const INITIATE_FILTERED_EVENT_LOG_TRANSFER_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Filter Type",
        data_type: DataType::Enum,
        description: "Filter type to use when reading event logs. Possible values are in Table 72",
        scale: None,
        unit: None,
        enum_table: Some("table_72"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 32,
        name: "Unix start time",
        data_type: DataType::Uint,
        description: "If using time span/next x filter - indicate the unix start time in seconds. All entries with a timestamp after this time will be included in the transfer.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 40,
        length_bits: 32,
        name: "Unix end time",
        data_type: DataType::Uint,
        description: "If using time span filter - indicate the unix end time in seconds. All entries with a timestamp before this time will be included in the transfer.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 72,
        length_bits: 32,
        name: "Number of Entries",
        data_type: DataType::Uint,
        description: "If using time next-x or first/last-x or counter next-x filter - indicate the number of entries to transfer.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 104,
        length_bits: 32,
        name: "Write Counter",
        data_type: DataType::Uint,
        description: "If using counter next-x filter - indicate the reference write counter value.",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 136,
        length_bits: 1,
        name: "Include critical events",
        data_type: DataType::Bool,
        description: "Include critical events in transfer Event Log",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 137,
        length_bits: 1,
        name: "Include major warning events",
        data_type: DataType::Bool,
        description: "Include major warning events in transfer Event Log",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 138,
        length_bits: 1,
        name: "Include minor warning events",
        data_type: DataType::Bool,
        description: "Include minor warning events in transfer Event Log",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 139,
        length_bits: 1,
        name: "Include info events",
        data_type: DataType::Bool,
        description: "Include info events in transfer Event Log",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 140,
        length_bits: 1,
        name: "Include CubeComputer",
        data_type: DataType::Bool,
        description: "Include the CubeComputer as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 141,
        length_bits: 1,
        name: "Include RWL 0",
        data_type: DataType::Bool,
        description: "Include the RWL 0 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 142,
        length_bits: 1,
        name: "Include RWL 1",
        data_type: DataType::Bool,
        description: "Include the RWL 1 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 143,
        length_bits: 1,
        name: "Include RWL 2",
        data_type: DataType::Bool,
        description: "Include the RWL 2 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 144,
        length_bits: 1,
        name: "Include RWL 3",
        data_type: DataType::Bool,
        description: "Include the RWL 3 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 145,
        length_bits: 1,
        name: "Include Fss 0",
        data_type: DataType::Bool,
        description: "Include the Fss 0 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 146,
        length_bits: 1,
        name: "Include Fss 1",
        data_type: DataType::Bool,
        description: "Include the Fss 1 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 147,
        length_bits: 1,
        name: "Include Fss 2",
        data_type: DataType::Bool,
        description: "Include the Fss 2 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 148,
        length_bits: 1,
        name: "Include Fss 3",
        data_type: DataType::Bool,
        description: "Include the Fss 3 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 149,
        length_bits: 1,
        name: "Include Hss 0",
        data_type: DataType::Bool,
        description: "Include the Hss 0 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 150,
        length_bits: 1,
        name: "Include Hss 1",
        data_type: DataType::Bool,
        description: "Include the Hss 1 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 151,
        length_bits: 1,
        name: "Include Str 0",
        data_type: DataType::Bool,
        description: "Include the Str 0 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 152,
        length_bits: 1,
        name: "Include Str 1",
        data_type: DataType::Bool,
        description: "Include the Str 1 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 153,
        length_bits: 1,
        name: "Include Mag 0",
        data_type: DataType::Bool,
        description: "Include the Mag 0 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 154,
        length_bits: 1,
        name: "Include Mag 1",
        data_type: DataType::Bool,
        description: "Include the Mag 1 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 155,
        length_bits: 1,
        name: "Include External 0",
        data_type: DataType::Bool,
        description: "Include the External 0 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 156,
        length_bits: 1,
        name: "Include External 1",
        data_type: DataType::Bool,
        description: "Include the External 1 as a source for events",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 157,
        length_bits: 3,
        name: "Reserved",
        data_type: DataType::Padding,
        description: "Reserved.",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Initiate Filtered Event Log Transfer.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct InitiateFilteredEventLogTransferCommand {
    /// Filter Type. Filter type to use when reading event logs. Possible values are in Table 72
    pub filter_type: u8,
    /// Unix start time. If using time span/next x filter - indicate the unix start time in seconds. All entries with a timestamp after this time will be included in the transfer.
    pub unix_start_time: u32,
    /// Unix end time. If using time span filter - indicate the unix end time in seconds. All entries with a timestamp before this time will be included in the transfer.
    pub unix_end_time: u32,
    /// Number of Entries. If using time next-x or first/last-x or counter next-x filter - indicate the number of entries to transfer.
    pub number_of_entries: u32,
    /// Write Counter. If using counter next-x filter - indicate the reference write counter value.
    pub write_counter: u32,
    /// Include critical events. Include critical events in transfer Event Log
    pub include_critical_events: bool,
    /// Include major warning events. Include major warning events in transfer Event Log
    pub include_major_warning_events: bool,
    /// Include minor warning events. Include minor warning events in transfer Event Log
    pub include_minor_warning_events: bool,
    /// Include info events. Include info events in transfer Event Log
    pub include_info_events: bool,
    /// Include CubeComputer. Include the CubeComputer as a source for events
    pub include_cubecomputer: bool,
    /// Include RWL 0. Include the RWL 0 as a source for events
    pub include_rwl_0: bool,
    /// Include RWL 1. Include the RWL 1 as a source for events
    pub include_rwl_1: bool,
    /// Include RWL 2. Include the RWL 2 as a source for events
    pub include_rwl_2: bool,
    /// Include RWL 3. Include the RWL 3 as a source for events
    pub include_rwl_3: bool,
    /// Include Fss 0. Include the Fss 0 as a source for events
    pub include_fss_0: bool,
    /// Include Fss 1. Include the Fss 1 as a source for events
    pub include_fss_1: bool,
    /// Include Fss 2. Include the Fss 2 as a source for events
    pub include_fss_2: bool,
    /// Include Fss 3. Include the Fss 3 as a source for events
    pub include_fss_3: bool,
    /// Include Hss 0. Include the Hss 0 as a source for events
    pub include_hss_0: bool,
    /// Include Hss 1. Include the Hss 1 as a source for events
    pub include_hss_1: bool,
    /// Include Str 0. Include the Str 0 as a source for events
    pub include_str_0: bool,
    /// Include Str 1. Include the Str 1 as a source for events
    pub include_str_1: bool,
    /// Include Mag 0. Include the Mag 0 as a source for events
    pub include_mag_0: bool,
    /// Include Mag 1. Include the Mag 1 as a source for events
    pub include_mag_1: bool,
    /// Include External 0. Include the External 0 as a source for events
    pub include_external_0: bool,
    /// Include External 1. Include the External 1 as a source for events
    pub include_external_1: bool,
}

impl Telecommand for InitiateFilteredEventLogTransferCommand {
    const ID: u8 = 120;
    const NAME: &'static str = "Initiate Filtered Event Log Transfer";
    const LENGTH_BYTES: usize = 20;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Filter Type",
            u128::from(self.filter_type),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            32,
            "Unix start time",
            u128::from(self.unix_start_time),
        )?;
        codec::write_unsigned(
            &mut payload,
            40,
            32,
            "Unix end time",
            u128::from(self.unix_end_time),
        )?;
        codec::write_unsigned(
            &mut payload,
            72,
            32,
            "Number of Entries",
            u128::from(self.number_of_entries),
        )?;
        codec::write_unsigned(
            &mut payload,
            104,
            32,
            "Write Counter",
            u128::from(self.write_counter),
        )?;
        codec::write_unsigned(
            &mut payload,
            136,
            1,
            "Include critical events",
            if self.include_critical_events { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            137,
            1,
            "Include major warning events",
            if self.include_major_warning_events {
                1
            } else {
                0
            },
        )?;
        codec::write_unsigned(
            &mut payload,
            138,
            1,
            "Include minor warning events",
            if self.include_minor_warning_events {
                1
            } else {
                0
            },
        )?;
        codec::write_unsigned(
            &mut payload,
            139,
            1,
            "Include info events",
            if self.include_info_events { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            140,
            1,
            "Include CubeComputer",
            if self.include_cubecomputer { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            141,
            1,
            "Include RWL 0",
            if self.include_rwl_0 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            142,
            1,
            "Include RWL 1",
            if self.include_rwl_1 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            143,
            1,
            "Include RWL 2",
            if self.include_rwl_2 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            144,
            1,
            "Include RWL 3",
            if self.include_rwl_3 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            145,
            1,
            "Include Fss 0",
            if self.include_fss_0 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            146,
            1,
            "Include Fss 1",
            if self.include_fss_1 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            147,
            1,
            "Include Fss 2",
            if self.include_fss_2 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            148,
            1,
            "Include Fss 3",
            if self.include_fss_3 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            149,
            1,
            "Include Hss 0",
            if self.include_hss_0 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            150,
            1,
            "Include Hss 1",
            if self.include_hss_1 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            151,
            1,
            "Include Str 0",
            if self.include_str_0 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            152,
            1,
            "Include Str 1",
            if self.include_str_1 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            153,
            1,
            "Include Mag 0",
            if self.include_mag_0 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            154,
            1,
            "Include Mag 1",
            if self.include_mag_1 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            155,
            1,
            "Include External 0",
            if self.include_external_0 { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            156,
            1,
            "Include External 1",
            if self.include_external_1 { 1 } else { 0 },
        )?;
        Ok(payload)
    }
}

const AUGMENTED_SGP4_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 16,
        name: "Batch size",
        data_type: DataType::Uint,
        description: "Number of GNSS measurements before computing updates TLEs",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 32,
        name: "Max time between GNSS measurements",
        data_type: DataType::Uint,
        description: "Maximum time between GNSS measurements before resetting filters. (Unit of measure is [s])",
        scale: None,
        unit: Some("s"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 16,
        name: "Max position error",
        data_type: DataType::Uint,
        description: "Maximum position error for asgp4 to continue working. (Unit of measure is [km])",
        scale: None,
        unit: Some("km"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 64,
        length_bits: 32,
        name: "Time gain",
        data_type: DataType::Float,
        description: "Time offset compensation gain",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 1,
        name: "Update RAAN and Inclination",
        data_type: DataType::Bool,
        description: "Update SGP4 RAAN and Inclination angles from GNSS",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 97,
        length_bits: 1,
        name: "Update Eccentricity",
        data_type: DataType::Bool,
        description: "Update SGP4 Eccentricity from GNSS",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 98,
        length_bits: 1,
        name: "Update AP and MA",
        data_type: DataType::Bool,
        description: "Update SGP4 Arg. Perigee and Mean Anomaly from GNSS",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 99,
        length_bits: 1,
        name: "Update Time",
        data_type: DataType::Bool,
        description: "Update SGP4 Epoch from GNSS",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for Augmented SGP4 configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct AugmentedSgp4ConfigurationCommand {
    /// Batch size. Number of GNSS measurements before computing updates TLEs
    pub batch_size: u16,
    /// Max time between GNSS measurements. Maximum time between GNSS measurements before resetting filters. (Unit of measure is [s])
    pub max_time_between_gnss_measurements: u32,
    /// Max position error. Maximum position error for asgp4 to continue working. (Unit of measure is [km])
    pub max_position_error: u16,
    /// Time gain. Time offset compensation gain
    pub time_gain: f32,
    /// Update RAAN and Inclination. Update SGP4 RAAN and Inclination angles from GNSS
    pub update_raan_and_inclination: bool,
    /// Update Eccentricity. Update SGP4 Eccentricity from GNSS
    pub update_eccentricity: bool,
    /// Update AP and MA. Update SGP4 Arg. Perigee and Mean Anomaly from GNSS
    pub update_ap_and_ma: bool,
    /// Update Time. Update SGP4 Epoch from GNSS
    pub update_time: bool,
}

impl Telecommand for AugmentedSgp4ConfigurationCommand {
    const ID: u8 = 50;
    const NAME: &'static str = "Augmented SGP4 configuration";
    const LENGTH_BYTES: usize = 13;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            16,
            "Batch size",
            u128::from(self.batch_size),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            32,
            "Max time between GNSS measurements",
            u128::from(self.max_time_between_gnss_measurements),
        )?;
        codec::write_unsigned(
            &mut payload,
            48,
            16,
            "Max position error",
            u128::from(self.max_position_error),
        )?;
        codec::write_unsigned(
            &mut payload,
            64,
            32,
            "Time gain",
            u128::from(self.time_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            96,
            1,
            "Update RAAN and Inclination",
            if self.update_raan_and_inclination {
                1
            } else {
                0
            },
        )?;
        codec::write_unsigned(
            &mut payload,
            97,
            1,
            "Update Eccentricity",
            if self.update_eccentricity { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            98,
            1,
            "Update AP and MA",
            if self.update_ap_and_ma { 1 } else { 0 },
        )?;
        codec::write_unsigned(
            &mut payload,
            99,
            1,
            "Update Time",
            if self.update_time { 1 } else { 0 },
        )?;
        Ok(payload)
    }
}

const ADCS_CONTROLLER_CONFIGURATION_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Default control mode",
        data_type: DataType::Enum,
        description: "Default control mode. Possible values are in Table 9",
        scale: None,
        unit: None,
        enum_table: Some("table_9"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 32,
        name: "Detumbling damping gain",
        data_type: DataType::Float,
        description: "Detumbling damping gain (Kd)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 40,
        length_bits: 32,
        name: "Sun-spin control gain - sunlit part",
        data_type: DataType::Float,
        description: "Sun-spin control gain (KDsun)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 72,
        length_bits: 32,
        name: "Sun-spin control gain - eclipse part",
        data_type: DataType::Float,
        description: "Sun-spin control gain (KDecl)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 104,
        length_bits: 32,
        name: "Detumbling spin gain",
        data_type: DataType::Float,
        description: "Detumbling spin gain (Ks)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 136,
        length_bits: 32,
        name: "Fast B-dot detumbling gain",
        data_type: DataType::Float,
        description: "Fast B-dot detumbling gain (Kdf)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 168,
        length_bits: 32,
        name: "Y-momentum nutation damping gain",
        data_type: DataType::Float,
        description: "Y-momentum nutation damping gain (Kn)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 200,
        length_bits: 32,
        name: "Y-momentum nutation damping quaternion gain",
        data_type: DataType::Float,
        description: "Y-momentum nutation damping quaternion gain (Kq)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 232,
        length_bits: 32,
        name: "X-axis GG nutation damping quaternion gain",
        data_type: DataType::Float,
        description: "X-axis GG nutation damping quaternion gain (Kqx)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 264,
        length_bits: 32,
        name: "Y-axis GG nutation damping quaternion gain",
        data_type: DataType::Float,
        description: "Y-axis GG nutation damping quaternion gain (Kqy)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 296,
        length_bits: 32,
        name: "Z-axis GG nutation damping quaternion gain",
        data_type: DataType::Float,
        description: "Z-axis GG nutation damping quaternion gain (Kqz)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 328,
        length_bits: 32,
        name: "Wheel desaturation control gain",
        data_type: DataType::Float,
        description: "Wheel momentum dumping magnetic control gain (Kh)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 360,
        length_bits: 32,
        name: "Y-momentum proportional gain",
        data_type: DataType::Float,
        description: "Y-momentum proportional gain (Kp1)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 392,
        length_bits: 32,
        name: "Y-momentum derivative gain",
        data_type: DataType::Float,
        description: "Y-momentum derivative gain (Kd1)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 424,
        length_bits: 32,
        name: "RWheel proportional gain",
        data_type: DataType::Float,
        description: "RWheel proportional gain (Kp2)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 456,
        length_bits: 32,
        name: "RWheel derivative gain",
        data_type: DataType::Float,
        description: "RWheel derivative gain (Kd2)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 488,
        length_bits: 32,
        name: "Tracking proportional gain",
        data_type: DataType::Float,
        description: "Tracking proportional gain (Kp3)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 520,
        length_bits: 32,
        name: "Tracking derivative gain",
        data_type: DataType::Float,
        description: "Tracking derivative gain (Kd3)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 552,
        length_bits: 32,
        name: "Tracking integral gain",
        data_type: DataType::Float,
        description: "Tracking integral gain (Ki3)",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 584,
        length_bits: 32,
        name: "Reference spin rate",
        data_type: DataType::Float,
        description: "Reference spin rate (wy-ref). (Unit of measure is [degps])",
        scale: None,
        unit: Some("degps"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 616,
        length_bits: 32,
        name: "Reference wheel momentum",
        data_type: DataType::Float,
        description: "Reference wheel momentum (H-ref). Must always be smaller than 0. (Unit of measure is [Nms])",
        scale: None,
        unit: Some("Nms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 648,
        length_bits: 32,
        name: "Y-wheel bias momentum during XYZ-control",
        data_type: DataType::Float,
        description: "Y-wheel bias momentum during XYZ-control (Hy-bias). (Unit of measure is [Nms])",
        scale: None,
        unit: Some("Nms"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 680,
        length_bits: 32,
        name: "Reference spin rate for RW spin control",
        data_type: DataType::Float,
        description: "Reference spin rate for ConSunYawSpin and ConRollSpin control modes. (Unit of measure is [degps])",
        scale: None,
        unit: Some("degps"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 712,
        length_bits: 32,
        name: "Sun keep-out angle",
        data_type: DataType::Float,
        description: "Sun keep-out angle. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 744,
        length_bits: 32,
        name: "Roll limit angle",
        data_type: DataType::Float,
        description: "Limit roll angle in ConRollSun and ConRollTarget. (Unit of measure is [deg])",
        scale: None,
        unit: Some("deg"),
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 776,
        length_bits: 1,
        name: "Yaw compensation for earth rotation",
        data_type: DataType::Bool,
        description: "Perform yaw compensation for earth rotation in 3-axis RPY control",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 777,
        length_bits: 1,
        name: "Enable sun tracking in eclipse",
        data_type: DataType::Bool,
        description: "Enable sun tracking during eclipse when using ConSunTrack",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 778,
        length_bits: 1,
        name: "Enable sun avoidance",
        data_type: DataType::Bool,
        description: "Enable sun avoidance",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 779,
        length_bits: 5,
        name: "Reserved",
        data_type: DataType::Padding,
        description: "Reserved.",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for ADCS controller configuration.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct AdcsControllerConfigurationCommand {
    /// Default control mode. Default control mode. Possible values are in Table 9
    pub default_control_mode: u8,
    /// Detumbling damping gain. Detumbling damping gain (Kd)
    pub detumbling_damping_gain: f32,
    /// Sun-spin control gain - sunlit part. Sun-spin control gain (KDsun)
    pub sun_spin_control_gain_sunlit_part: f32,
    /// Sun-spin control gain - eclipse part. Sun-spin control gain (KDecl)
    pub sun_spin_control_gain_eclipse_part: f32,
    /// Detumbling spin gain. Detumbling spin gain (Ks)
    pub detumbling_spin_gain: f32,
    /// Fast B-dot detumbling gain. Fast B-dot detumbling gain (Kdf)
    pub fast_b_dot_detumbling_gain: f32,
    /// Y-momentum nutation damping gain. Y-momentum nutation damping gain (Kn)
    pub y_momentum_nutation_damping_gain: f32,
    /// Y-momentum nutation damping quaternion gain. Y-momentum nutation damping quaternion gain (Kq)
    pub y_momentum_nutation_damping_quaternion_gain: f32,
    /// X-axis GG nutation damping quaternion gain. X-axis GG nutation damping quaternion gain (Kqx)
    pub x_axis_gg_nutation_damping_quaternion_gain: f32,
    /// Y-axis GG nutation damping quaternion gain. Y-axis GG nutation damping quaternion gain (Kqy)
    pub y_axis_gg_nutation_damping_quaternion_gain: f32,
    /// Z-axis GG nutation damping quaternion gain. Z-axis GG nutation damping quaternion gain (Kqz)
    pub z_axis_gg_nutation_damping_quaternion_gain: f32,
    /// Wheel desaturation control gain. Wheel momentum dumping magnetic control gain (Kh)
    pub wheel_desaturation_control_gain: f32,
    /// Y-momentum proportional gain. Y-momentum proportional gain (Kp1)
    pub y_momentum_proportional_gain: f32,
    /// Y-momentum derivative gain. Y-momentum derivative gain (Kd1)
    pub y_momentum_derivative_gain: f32,
    /// RWheel proportional gain. RWheel proportional gain (Kp2)
    pub rwheel_proportional_gain: f32,
    /// RWheel derivative gain. RWheel derivative gain (Kd2)
    pub rwheel_derivative_gain: f32,
    /// Tracking proportional gain. Tracking proportional gain (Kp3)
    pub tracking_proportional_gain: f32,
    /// Tracking derivative gain. Tracking derivative gain (Kd3)
    pub tracking_derivative_gain: f32,
    /// Tracking integral gain. Tracking integral gain (Ki3)
    pub tracking_integral_gain: f32,
    /// Reference spin rate. Reference spin rate (wy-ref). (Unit of measure is [degps])
    pub reference_spin_rate: f32,
    /// Reference wheel momentum. Reference wheel momentum (H-ref). Must always be smaller than 0. (Unit of measure is [Nms])
    pub reference_wheel_momentum: f32,
    /// Y-wheel bias momentum during XYZ-control. Y-wheel bias momentum during XYZ-control (Hy-bias). (Unit of measure is [Nms])
    pub y_wheel_bias_momentum_during_xyz_control: f32,
    /// Reference spin rate for RW spin control. Reference spin rate for ConSunYawSpin and ConRollSpin control modes. (Unit of measure is [degps])
    pub reference_spin_rate_for_rw_spin_control: f32,
    /// Sun keep-out angle. Sun keep-out angle. (Unit of measure is [deg])
    pub sun_keep_out_angle: f32,
    /// Roll limit angle. Limit roll angle in ConRollSun and ConRollTarget. (Unit of measure is [deg])
    pub roll_limit_angle: f32,
    /// Yaw compensation for earth rotation. Perform yaw compensation for earth rotation in 3-axis RPY control
    pub yaw_compensation_for_earth_rotation: bool,
    /// Enable sun tracking in eclipse. Enable sun tracking during eclipse when using ConSunTrack
    pub enable_sun_tracking_in_eclipse: bool,
    /// Enable sun avoidance. Enable sun avoidance
    pub enable_sun_avoidance: bool,
}

impl Telecommand for AdcsControllerConfigurationCommand {
    const ID: u8 = 62;
    const NAME: &'static str = "ADCS controller configuration";
    const LENGTH_BYTES: usize = 98;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Default control mode",
            u128::from(self.default_control_mode),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            32,
            "Detumbling damping gain",
            u128::from(self.detumbling_damping_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            40,
            32,
            "Sun-spin control gain - sunlit part",
            u128::from(self.sun_spin_control_gain_sunlit_part.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            72,
            32,
            "Sun-spin control gain - eclipse part",
            u128::from(self.sun_spin_control_gain_eclipse_part.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            104,
            32,
            "Detumbling spin gain",
            u128::from(self.detumbling_spin_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            136,
            32,
            "Fast B-dot detumbling gain",
            u128::from(self.fast_b_dot_detumbling_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            168,
            32,
            "Y-momentum nutation damping gain",
            u128::from(self.y_momentum_nutation_damping_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            200,
            32,
            "Y-momentum nutation damping quaternion gain",
            u128::from(self.y_momentum_nutation_damping_quaternion_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            232,
            32,
            "X-axis GG nutation damping quaternion gain",
            u128::from(self.x_axis_gg_nutation_damping_quaternion_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            264,
            32,
            "Y-axis GG nutation damping quaternion gain",
            u128::from(self.y_axis_gg_nutation_damping_quaternion_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            296,
            32,
            "Z-axis GG nutation damping quaternion gain",
            u128::from(self.z_axis_gg_nutation_damping_quaternion_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            328,
            32,
            "Wheel desaturation control gain",
            u128::from(self.wheel_desaturation_control_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            360,
            32,
            "Y-momentum proportional gain",
            u128::from(self.y_momentum_proportional_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            392,
            32,
            "Y-momentum derivative gain",
            u128::from(self.y_momentum_derivative_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            424,
            32,
            "RWheel proportional gain",
            u128::from(self.rwheel_proportional_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            456,
            32,
            "RWheel derivative gain",
            u128::from(self.rwheel_derivative_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            488,
            32,
            "Tracking proportional gain",
            u128::from(self.tracking_proportional_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            520,
            32,
            "Tracking derivative gain",
            u128::from(self.tracking_derivative_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            552,
            32,
            "Tracking integral gain",
            u128::from(self.tracking_integral_gain.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            584,
            32,
            "Reference spin rate",
            u128::from(self.reference_spin_rate.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            616,
            32,
            "Reference wheel momentum",
            u128::from(self.reference_wheel_momentum.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            648,
            32,
            "Y-wheel bias momentum during XYZ-control",
            u128::from(self.y_wheel_bias_momentum_during_xyz_control.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            680,
            32,
            "Reference spin rate for RW spin control",
            u128::from(self.reference_spin_rate_for_rw_spin_control.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            712,
            32,
            "Sun keep-out angle",
            u128::from(self.sun_keep_out_angle.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            744,
            32,
            "Roll limit angle",
            u128::from(self.roll_limit_angle.to_bits()),
        )?;
        codec::write_unsigned(
            &mut payload,
            776,
            1,
            "Yaw compensation for earth rotation",
            if self.yaw_compensation_for_earth_rotation {
                1
            } else {
                0
            },
        )?;
        codec::write_unsigned(
            &mut payload,
            777,
            1,
            "Enable sun tracking in eclipse",
            if self.enable_sun_tracking_in_eclipse {
                1
            } else {
                0
            },
        )?;
        codec::write_unsigned(
            &mut payload,
            778,
            1,
            "Enable sun avoidance",
            if self.enable_sun_avoidance { 1 } else { 0 },
        )?;
        Ok(payload)
    }
}

const ADCS_OPERATIONAL_STATE_COMMAND_FIELDS: &[FieldSpec] = &[
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "ADCS operational state",
        data_type: DataType::Enum,
        description: "ADCS operational state. Possible values are in Table 36",
        scale: None,
        unit: None,
        enum_table: Some("table_36"),
    },
    FieldSpec {
        offset_bits: 0,
        length_bits: 8,
        name: "Node Type: Sensor 1",
        data_type: DataType::Enum,
        description: "Sensor 1 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 8,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 1",
        data_type: DataType::Enum,
        description:
            "Sensor 1 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 16,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 1",
        data_type: DataType::Uint,
        description: "Sensor 1 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 48,
        length_bits: 32,
        name: "Address: Sensor 1",
        data_type: DataType::Uint,
        description: "Sensor 1 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 80,
        length_bits: 8,
        name: "Node Type: Sensor 2",
        data_type: DataType::Enum,
        description: "Sensor 2 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 88,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 2",
        data_type: DataType::Enum,
        description:
            "Sensor 2 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 96,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 2",
        data_type: DataType::Uint,
        description: "Sensor 2 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 128,
        length_bits: 32,
        name: "Address: Sensor 2",
        data_type: DataType::Uint,
        description: "Sensor 2 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 160,
        length_bits: 8,
        name: "Node Type: Sensor 3",
        data_type: DataType::Enum,
        description: "Sensor 3 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 168,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 3",
        data_type: DataType::Enum,
        description:
            "Sensor 3 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 176,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 3",
        data_type: DataType::Uint,
        description: "Sensor 3 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 208,
        length_bits: 32,
        name: "Address: Sensor 3",
        data_type: DataType::Uint,
        description: "Sensor 3 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 240,
        length_bits: 8,
        name: "Node Type: Sensor 4",
        data_type: DataType::Enum,
        description: "Sensor 4 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 248,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 4",
        data_type: DataType::Enum,
        description:
            "Sensor 4 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 256,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 4",
        data_type: DataType::Uint,
        description: "Sensor 4 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 288,
        length_bits: 32,
        name: "Address: Sensor 4",
        data_type: DataType::Uint,
        description: "Sensor 4 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 320,
        length_bits: 8,
        name: "Node Type: Sensor 5",
        data_type: DataType::Enum,
        description: "Sensor 5 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 328,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 5",
        data_type: DataType::Enum,
        description:
            "Sensor 5 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 336,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 5",
        data_type: DataType::Uint,
        description: "Sensor 5 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 368,
        length_bits: 32,
        name: "Address: Sensor 5",
        data_type: DataType::Uint,
        description: "Sensor 5 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 400,
        length_bits: 8,
        name: "Node Type: Sensor 6",
        data_type: DataType::Enum,
        description: "Sensor 6 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 408,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 6",
        data_type: DataType::Enum,
        description:
            "Sensor 6 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 416,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 6",
        data_type: DataType::Uint,
        description: "Sensor 6 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 448,
        length_bits: 32,
        name: "Address: Sensor 6",
        data_type: DataType::Uint,
        description: "Sensor 6 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 480,
        length_bits: 8,
        name: "Node Type: Sensor 7",
        data_type: DataType::Enum,
        description: "Sensor 7 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 488,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 7",
        data_type: DataType::Enum,
        description:
            "Sensor 7 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 496,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 7",
        data_type: DataType::Uint,
        description: "Sensor 7 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 528,
        length_bits: 32,
        name: "Address: Sensor 7",
        data_type: DataType::Uint,
        description: "Sensor 7 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 560,
        length_bits: 8,
        name: "Node Type: Sensor 8",
        data_type: DataType::Enum,
        description: "Sensor 8 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 568,
        length_bits: 8,
        name: "Abstract Node Type: Sensor 8",
        data_type: DataType::Enum,
        description:
            "Sensor 8 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 576,
        length_bits: 32,
        name: "Serial Number Integer: Sensor 8",
        data_type: DataType::Uint,
        description: "Sensor 8 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 608,
        length_bits: 32,
        name: "Address: Sensor 8",
        data_type: DataType::Uint,
        description: "Sensor 8 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 640,
        length_bits: 8,
        name: "Node Type: Wheel 1",
        data_type: DataType::Enum,
        description: "Wheel 1 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 648,
        length_bits: 8,
        name: "Abstract Node Type: Wheel 1",
        data_type: DataType::Enum,
        description:
            "Wheel 1 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 656,
        length_bits: 32,
        name: "Serial Number Integer: Wheel 1",
        data_type: DataType::Uint,
        description: "Wheel 1 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 688,
        length_bits: 32,
        name: "Address: Wheel 1",
        data_type: DataType::Uint,
        description: "Wheel 1 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 720,
        length_bits: 8,
        name: "Node Type: Wheel 2",
        data_type: DataType::Enum,
        description: "Wheel 2 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 728,
        length_bits: 8,
        name: "Abstract Node Type: Wheel 2",
        data_type: DataType::Enum,
        description:
            "Wheel 2 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 736,
        length_bits: 32,
        name: "Serial Number Integer: Wheel 2",
        data_type: DataType::Uint,
        description: "Wheel 2 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 768,
        length_bits: 32,
        name: "Address: Wheel 2",
        data_type: DataType::Uint,
        description: "Wheel 2 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 800,
        length_bits: 8,
        name: "Node Type: Wheel 3",
        data_type: DataType::Enum,
        description: "Wheel 3 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 808,
        length_bits: 8,
        name: "Abstract Node Type: Wheel 3",
        data_type: DataType::Enum,
        description:
            "Wheel 3 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 816,
        length_bits: 32,
        name: "Serial Number Integer: Wheel 3",
        data_type: DataType::Uint,
        description: "Wheel 3 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 848,
        length_bits: 32,
        name: "Address: Wheel 3",
        data_type: DataType::Uint,
        description: "Wheel 3 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 880,
        length_bits: 8,
        name: "Node Type: Wheel 4",
        data_type: DataType::Enum,
        description: "Wheel 4 port - Node type identifier. Possible values are in Table 57",
        scale: None,
        unit: None,
        enum_table: Some("table_57"),
    },
    FieldSpec {
        offset_bits: 888,
        length_bits: 8,
        name: "Abstract Node Type: Wheel 4",
        data_type: DataType::Enum,
        description:
            "Wheel 4 port - Abstract Node type identifier. Possible values are in Table 61",
        scale: None,
        unit: None,
        enum_table: Some("table_61"),
    },
    FieldSpec {
        offset_bits: 896,
        length_bits: 32,
        name: "Serial Number Integer: Wheel 4",
        data_type: DataType::Uint,
        description: "Wheel 4 port - Serial Number Integer Respresentation",
        scale: None,
        unit: None,
        enum_table: None,
    },
    FieldSpec {
        offset_bits: 928,
        length_bits: 32,
        name: "Address: Wheel 4",
        data_type: DataType::Uint,
        description: "Wheel 4 port - CAN Address",
        scale: None,
        unit: None,
        enum_table: None,
    },
];

/// Telecommand input for ADCS operational state.
#[derive(Clone, Debug, Default, PartialEq, InputObject)]
pub struct AdcsOperationalStateCommand {
    /// ADCS operational state. ADCS operational state. Possible values are in Table 36
    pub adcs_operational_state: u8,
    /// Node Type: Sensor 1. Sensor 1 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_1: u8,
    /// Abstract Node Type: Sensor 1. Sensor 1 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_1: u8,
    /// Serial Number Integer: Sensor 1. Sensor 1 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_1: u32,
    /// Address: Sensor 1. Sensor 1 port - CAN Address
    pub address_sensor_1: u32,
    /// Node Type: Sensor 2. Sensor 2 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_2: u8,
    /// Abstract Node Type: Sensor 2. Sensor 2 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_2: u8,
    /// Serial Number Integer: Sensor 2. Sensor 2 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_2: u32,
    /// Address: Sensor 2. Sensor 2 port - CAN Address
    pub address_sensor_2: u32,
    /// Node Type: Sensor 3. Sensor 3 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_3: u8,
    /// Abstract Node Type: Sensor 3. Sensor 3 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_3: u8,
    /// Serial Number Integer: Sensor 3. Sensor 3 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_3: u32,
    /// Address: Sensor 3. Sensor 3 port - CAN Address
    pub address_sensor_3: u32,
    /// Node Type: Sensor 4. Sensor 4 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_4: u8,
    /// Abstract Node Type: Sensor 4. Sensor 4 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_4: u8,
    /// Serial Number Integer: Sensor 4. Sensor 4 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_4: u32,
    /// Address: Sensor 4. Sensor 4 port - CAN Address
    pub address_sensor_4: u32,
    /// Node Type: Sensor 5. Sensor 5 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_5: u8,
    /// Abstract Node Type: Sensor 5. Sensor 5 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_5: u8,
    /// Serial Number Integer: Sensor 5. Sensor 5 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_5: u32,
    /// Address: Sensor 5. Sensor 5 port - CAN Address
    pub address_sensor_5: u32,
    /// Node Type: Sensor 6. Sensor 6 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_6: u8,
    /// Abstract Node Type: Sensor 6. Sensor 6 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_6: u8,
    /// Serial Number Integer: Sensor 6. Sensor 6 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_6: u32,
    /// Address: Sensor 6. Sensor 6 port - CAN Address
    pub address_sensor_6: u32,
    /// Node Type: Sensor 7. Sensor 7 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_7: u8,
    /// Abstract Node Type: Sensor 7. Sensor 7 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_7: u8,
    /// Serial Number Integer: Sensor 7. Sensor 7 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_7: u32,
    /// Address: Sensor 7. Sensor 7 port - CAN Address
    pub address_sensor_7: u32,
    /// Node Type: Sensor 8. Sensor 8 port - Node type identifier. Possible values are in Table 57
    pub node_type_sensor_8: u8,
    /// Abstract Node Type: Sensor 8. Sensor 8 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_sensor_8: u8,
    /// Serial Number Integer: Sensor 8. Sensor 8 port - Serial Number Integer Respresentation
    pub serial_number_integer_sensor_8: u32,
    /// Address: Sensor 8. Sensor 8 port - CAN Address
    pub address_sensor_8: u32,
    /// Node Type: Wheel 1. Wheel 1 port - Node type identifier. Possible values are in Table 57
    pub node_type_wheel_1: u8,
    /// Abstract Node Type: Wheel 1. Wheel 1 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_wheel_1: u8,
    /// Serial Number Integer: Wheel 1. Wheel 1 port - Serial Number Integer Respresentation
    pub serial_number_integer_wheel_1: u32,
    /// Address: Wheel 1. Wheel 1 port - CAN Address
    pub address_wheel_1: u32,
    /// Node Type: Wheel 2. Wheel 2 port - Node type identifier. Possible values are in Table 57
    pub node_type_wheel_2: u8,
    /// Abstract Node Type: Wheel 2. Wheel 2 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_wheel_2: u8,
    /// Serial Number Integer: Wheel 2. Wheel 2 port - Serial Number Integer Respresentation
    pub serial_number_integer_wheel_2: u32,
    /// Address: Wheel 2. Wheel 2 port - CAN Address
    pub address_wheel_2: u32,
    /// Node Type: Wheel 3. Wheel 3 port - Node type identifier. Possible values are in Table 57
    pub node_type_wheel_3: u8,
    /// Abstract Node Type: Wheel 3. Wheel 3 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_wheel_3: u8,
    /// Serial Number Integer: Wheel 3. Wheel 3 port - Serial Number Integer Respresentation
    pub serial_number_integer_wheel_3: u32,
    /// Address: Wheel 3. Wheel 3 port - CAN Address
    pub address_wheel_3: u32,
    /// Node Type: Wheel 4. Wheel 4 port - Node type identifier. Possible values are in Table 57
    pub node_type_wheel_4: u8,
    /// Abstract Node Type: Wheel 4. Wheel 4 port - Abstract Node type identifier. Possible values are in Table 61
    pub abstract_node_type_wheel_4: u8,
    /// Serial Number Integer: Wheel 4. Wheel 4 port - Serial Number Integer Respresentation
    pub serial_number_integer_wheel_4: u32,
    /// Address: Wheel 4. Wheel 4 port - CAN Address
    pub address_wheel_4: u32,
}

impl Telecommand for AdcsOperationalStateCommand {
    const ID: u8 = 72;
    const NAME: &'static str = "ADCS operational state";
    const LENGTH_BYTES: usize = 1;

    fn encode(&self) -> AdcsResult<Vec<u8>> {
        let mut payload = vec![0; Self::LENGTH_BYTES];
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "ADCS operational state",
            u128::from(self.adcs_operational_state),
        )?;
        codec::write_unsigned(
            &mut payload,
            0,
            8,
            "Node Type: Sensor 1",
            u128::from(self.node_type_sensor_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            8,
            8,
            "Abstract Node Type: Sensor 1",
            u128::from(self.abstract_node_type_sensor_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            16,
            32,
            "Serial Number Integer: Sensor 1",
            u128::from(self.serial_number_integer_sensor_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            48,
            32,
            "Address: Sensor 1",
            u128::from(self.address_sensor_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            80,
            8,
            "Node Type: Sensor 2",
            u128::from(self.node_type_sensor_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            88,
            8,
            "Abstract Node Type: Sensor 2",
            u128::from(self.abstract_node_type_sensor_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            96,
            32,
            "Serial Number Integer: Sensor 2",
            u128::from(self.serial_number_integer_sensor_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            128,
            32,
            "Address: Sensor 2",
            u128::from(self.address_sensor_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            160,
            8,
            "Node Type: Sensor 3",
            u128::from(self.node_type_sensor_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            168,
            8,
            "Abstract Node Type: Sensor 3",
            u128::from(self.abstract_node_type_sensor_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            176,
            32,
            "Serial Number Integer: Sensor 3",
            u128::from(self.serial_number_integer_sensor_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            208,
            32,
            "Address: Sensor 3",
            u128::from(self.address_sensor_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            240,
            8,
            "Node Type: Sensor 4",
            u128::from(self.node_type_sensor_4),
        )?;
        codec::write_unsigned(
            &mut payload,
            248,
            8,
            "Abstract Node Type: Sensor 4",
            u128::from(self.abstract_node_type_sensor_4),
        )?;
        codec::write_unsigned(
            &mut payload,
            256,
            32,
            "Serial Number Integer: Sensor 4",
            u128::from(self.serial_number_integer_sensor_4),
        )?;
        codec::write_unsigned(
            &mut payload,
            288,
            32,
            "Address: Sensor 4",
            u128::from(self.address_sensor_4),
        )?;
        codec::write_unsigned(
            &mut payload,
            320,
            8,
            "Node Type: Sensor 5",
            u128::from(self.node_type_sensor_5),
        )?;
        codec::write_unsigned(
            &mut payload,
            328,
            8,
            "Abstract Node Type: Sensor 5",
            u128::from(self.abstract_node_type_sensor_5),
        )?;
        codec::write_unsigned(
            &mut payload,
            336,
            32,
            "Serial Number Integer: Sensor 5",
            u128::from(self.serial_number_integer_sensor_5),
        )?;
        codec::write_unsigned(
            &mut payload,
            368,
            32,
            "Address: Sensor 5",
            u128::from(self.address_sensor_5),
        )?;
        codec::write_unsigned(
            &mut payload,
            400,
            8,
            "Node Type: Sensor 6",
            u128::from(self.node_type_sensor_6),
        )?;
        codec::write_unsigned(
            &mut payload,
            408,
            8,
            "Abstract Node Type: Sensor 6",
            u128::from(self.abstract_node_type_sensor_6),
        )?;
        codec::write_unsigned(
            &mut payload,
            416,
            32,
            "Serial Number Integer: Sensor 6",
            u128::from(self.serial_number_integer_sensor_6),
        )?;
        codec::write_unsigned(
            &mut payload,
            448,
            32,
            "Address: Sensor 6",
            u128::from(self.address_sensor_6),
        )?;
        codec::write_unsigned(
            &mut payload,
            480,
            8,
            "Node Type: Sensor 7",
            u128::from(self.node_type_sensor_7),
        )?;
        codec::write_unsigned(
            &mut payload,
            488,
            8,
            "Abstract Node Type: Sensor 7",
            u128::from(self.abstract_node_type_sensor_7),
        )?;
        codec::write_unsigned(
            &mut payload,
            496,
            32,
            "Serial Number Integer: Sensor 7",
            u128::from(self.serial_number_integer_sensor_7),
        )?;
        codec::write_unsigned(
            &mut payload,
            528,
            32,
            "Address: Sensor 7",
            u128::from(self.address_sensor_7),
        )?;
        codec::write_unsigned(
            &mut payload,
            560,
            8,
            "Node Type: Sensor 8",
            u128::from(self.node_type_sensor_8),
        )?;
        codec::write_unsigned(
            &mut payload,
            568,
            8,
            "Abstract Node Type: Sensor 8",
            u128::from(self.abstract_node_type_sensor_8),
        )?;
        codec::write_unsigned(
            &mut payload,
            576,
            32,
            "Serial Number Integer: Sensor 8",
            u128::from(self.serial_number_integer_sensor_8),
        )?;
        codec::write_unsigned(
            &mut payload,
            608,
            32,
            "Address: Sensor 8",
            u128::from(self.address_sensor_8),
        )?;
        codec::write_unsigned(
            &mut payload,
            640,
            8,
            "Node Type: Wheel 1",
            u128::from(self.node_type_wheel_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            648,
            8,
            "Abstract Node Type: Wheel 1",
            u128::from(self.abstract_node_type_wheel_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            656,
            32,
            "Serial Number Integer: Wheel 1",
            u128::from(self.serial_number_integer_wheel_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            688,
            32,
            "Address: Wheel 1",
            u128::from(self.address_wheel_1),
        )?;
        codec::write_unsigned(
            &mut payload,
            720,
            8,
            "Node Type: Wheel 2",
            u128::from(self.node_type_wheel_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            728,
            8,
            "Abstract Node Type: Wheel 2",
            u128::from(self.abstract_node_type_wheel_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            736,
            32,
            "Serial Number Integer: Wheel 2",
            u128::from(self.serial_number_integer_wheel_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            768,
            32,
            "Address: Wheel 2",
            u128::from(self.address_wheel_2),
        )?;
        codec::write_unsigned(
            &mut payload,
            800,
            8,
            "Node Type: Wheel 3",
            u128::from(self.node_type_wheel_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            808,
            8,
            "Abstract Node Type: Wheel 3",
            u128::from(self.abstract_node_type_wheel_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            816,
            32,
            "Serial Number Integer: Wheel 3",
            u128::from(self.serial_number_integer_wheel_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            848,
            32,
            "Address: Wheel 3",
            u128::from(self.address_wheel_3),
        )?;
        codec::write_unsigned(
            &mut payload,
            880,
            8,
            "Node Type: Wheel 4",
            u128::from(self.node_type_wheel_4),
        )?;
        codec::write_unsigned(
            &mut payload,
            888,
            8,
            "Abstract Node Type: Wheel 4",
            u128::from(self.abstract_node_type_wheel_4),
        )?;
        codec::write_unsigned(
            &mut payload,
            896,
            32,
            "Serial Number Integer: Wheel 4",
            u128::from(self.serial_number_integer_wheel_4),
        )?;
        codec::write_unsigned(
            &mut payload,
            928,
            32,
            "Address: Wheel 4",
            u128::from(self.address_wheel_4),
        )?;
        Ok(payload)
    }
}

/// Static telecommand definitions generated from the CubeSpace matrix.
pub const COMMAND_SPECS: &[CommandSpec] = &[
    CommandSpec {
        id: 1,
        name: "Reset",
        purpose: "Reset CubeComputer/control program",
        length_bytes: 1,
        fields: RESET_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 2,
        name: "Current Unix Time",
        purpose: "Set ADCS Unix time",
        length_bytes: 8,
        fields: CURRENT_UNIX_TIME_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 5,
        name: "Error Log Clear",
        purpose: "Clear ADCS error log",
        length_bytes: 0,
        fields: ERROR_LOG_CLEAR_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 7,
        name: "Persist Config",
        purpose: "Save volatile config to flash",
        length_bytes: 0,
        fields: PERSIST_CONFIG_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 42,
        name: "Control and estimation mode",
        purpose: "Change both Con and Est Mode",
        length_bytes: 5,
        fields: CONTROL_AND_ESTIMATION_MODE_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 49,
        name: "Commanded GNSS Measurements",
        purpose: "Send GNSS data to ADCS",
        length_bytes: 33,
        fields: COMMANDED_GNSS_MEASUREMENTS_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 51,
        name: "Orbit Mode",
        purpose: "Set orbit calculation mode",
        length_bytes: 1,
        fields: ORBIT_MODE_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 52,
        name: "Mag Deploy Command",
        purpose: "Deploy magnetometer boom",
        length_bytes: 1,
        fields: MAG_DEPLOY_COMMAND_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 54,
        name: "Reference RPY Values",
        purpose: "Command roll/pitch/yaw reference",
        length_bytes: 12,
        fields: REFERENCE_RPY_VALUES_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 55,
        name: "OpenLoopCommandMtq",
        purpose: "Command magnetorquers open-loop",
        length_bytes: 6,
        fields: OPENLOOPCOMMANDMTQ_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 56,
        name: "PowerState",
        purpose: "Powering shit on the ADCS",
        length_bytes: 20,
        fields: POWERSTATE_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 57,
        name: "ADCS Run Mode",
        purpose: "Turn ADCS loop on/off",
        length_bytes: 1,
        fields: ADCS_RUN_MODE_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 58,
        name: "Control Mode",
        purpose: "Change Control Mode",
        length_bytes: 3,
        fields: CONTROL_MODE_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 59,
        name: "Wheel Configuration",
        purpose: "Configure reaction wheel hardware/mounting scheme",
        length_bytes: 62,
        fields: WHEEL_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 61,
        name: "ADCS Satellite Config",
        purpose: "Configure satellite inertia and pointing vectors",
        length_bytes: 42,
        fields: ADCS_SATELLITE_CONFIG_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 63,
        name: "Mag0 Orbit Calibration Config",
        purpose: "Upload MAG0 calibration",
        length_bytes: 24,
        fields: MAG0_ORBIT_CALIBRATION_CONFIG_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 64,
        name: "Default Mode Configuration",
        purpose: "Set power-on/default ADCS modes",
        length_bytes: 4,
        fields: DEFAULT_MODE_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 65,
        name: "Mounting Configuration",
        purpose: "Configure sensor/actuator mounting axes and angles",
        length_bytes: 104,
        fields: MOUNTING_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 66,
        name: "Mag1 Orbit Calibration Config",
        purpose: "Upload MAG1 calibration",
        length_bytes: 24,
        fields: MAG1_ORBIT_CALIBRATION_CONFIG_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 67,
        name: "ADCS Estimator Config",
        purpose: "Configure EKF estimator settings",
        length_bytes: 40,
        fields: ADCS_ESTIMATOR_CONFIG_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 68,
        name: "Satellite Orbit Parameters",
        purpose: "Upload own-satellite orbit/TLE parameters",
        length_bytes: 64,
        fields: SATELLITE_ORBIT_PARAMETERS_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 69,
        name: "Node Selection Configuration",
        purpose: "Select which physical sensors/actuators ADCS should use",
        length_bytes: 8,
        fields: NODE_SELECTION_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 70,
        name: "Magnetorquer Configuration",
        purpose: "Configure magnetorquer strength/on-time settings",
        length_bytes: 20,
        fields: MAGNETORQUER_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 71,
        name: "Estimation Mode",
        purpose: "Change Estimation Mode",
        length_bytes: 2,
        fields: ESTIMATION_MODE_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 74,
        name: "OpenLoopCommandRwl",
        purpose: "Directly command the reaction wheels",
        length_bytes: 16,
        fields: OPENLOOPCOMMANDRWL_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 76,
        name: "OpenLoopCommandHxyzRW",
        purpose: "Command wheel momentum vector",
        length_bytes: 12,
        fields: OPENLOOPCOMMANDHXYZRW_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 77,
        name: "Mag Sensing Element Configuration",
        purpose: "Select primary/redundant magnetometer sensing element",
        length_bytes: 1,
        fields: MAG_SENSING_ELEMENT_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 78,
        name: "Data Frame",
        purpose: "Single arbitrary data frame",
        length_bytes: 258,
        fields: DATA_FRAME_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 117,
        name: "Request Telemetry Log Transfer Setup",
        purpose: "Set up telemetry log download",
        length_bytes: 23,
        fields: REQUEST_TELEMETRY_LOG_TRANSFER_SETUP_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 120,
        name: "Initiate Filtered Event Log Transfer",
        purpose: "Set up filtered event log download",
        length_bytes: 20,
        fields: INITIATE_FILTERED_EVENT_LOG_TRANSFER_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 50,
        name: "Augmented SGP4 configuration",
        purpose: "Augmented SGP4 configuration",
        length_bytes: 13,
        fields: AUGMENTED_SGP4_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 62,
        name: "ADCS controller configuration",
        purpose: "Configure ADCS controller gains/limits",
        length_bytes: 98,
        fields: ADCS_CONTROLLER_CONFIGURATION_COMMAND_FIELDS,
    },
    CommandSpec {
        id: 72,
        name: "ADCS operational state",
        purpose: "Set ADCS operational state",
        length_bytes: 1,
        fields: ADCS_OPERATIONAL_STATE_COMMAND_FIELDS,
    },
];

/// Looks up a generated telecommand definition by ID.
pub fn command_spec(id: u8) -> Option<&'static CommandSpec> {
    COMMAND_SPECS.iter().find(|spec| spec.id == id)
}
