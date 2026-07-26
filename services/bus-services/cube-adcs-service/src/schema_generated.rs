//! Generated GraphQL command and telemetry fields for CubeSpace ADCS.

use async_graphql::{Context, Object, Result};
use cubespace_adcs_api::{self as api};

use crate::schema::map_command_ack;
use crate::subsystem::Subsystem;

/// Generated GraphQL queries, one per telemetry ID.
#[derive(Default)]
pub struct GeneratedQueryRoot;

#[Object]
impl GeneratedQueryRoot {
    /// Requests and decodes telemetry ID 131: Error Log Entry.
    async fn error_log_entry(&self, ctx: &Context<'_>) -> Result<api::ErrorLogEntryTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ErrorLogEntryTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 133: Current Unix Time.
    async fn current_unix_time(&self, ctx: &Context<'_>) -> Result<api::CurrentUnixTimeTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CurrentUnixTimeTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 134: Persist Config Diagnostics.
    async fn persist_config_diagnostics(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::PersistConfigDiagnosticsTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::PersistConfigDiagnosticsTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 136: Version.
    async fn version(&self, ctx: &Context<'_>) -> Result<api::VersionTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::VersionTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 139: Common Error Codes.
    async fn common_error_codes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CommonErrorCodesTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CommonErrorCodesTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 140: Identification2.
    async fn identification2(&self, ctx: &Context<'_>) -> Result<api::Identification2Telemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::Identification2Telemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 161: File Transfer Status.
    async fn file_transfer_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::FileTransferStatusTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::FileTransferStatusTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 167: CubeMag Health.
    async fn cubemag_health(&self, ctx: &Context<'_>) -> Result<api::CubemagHealthTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CubemagHealthTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 168: CubeSense Sun Health.
    async fn cubesense_sun_health(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CubesenseSunHealthTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CubesenseSunHealthTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 169: Torquer Current.
    async fn torquer_current(&self, ctx: &Context<'_>) -> Result<api::TorquerCurrentTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::TorquerCurrentTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 170: FSS CubeSense Sun Raw.
    async fn fss_cubesense_sun_raw(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::FssCubesenseSunRawTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::FssCubesenseSunRawTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 172: Controller.
    async fn controller(&self, ctx: &Context<'_>) -> Result<api::ControllerTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ControllerTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 173: Estimator Backup.
    async fn estimator_backup(&self, ctx: &Context<'_>) -> Result<api::EstimatorBackupTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::EstimatorBackupTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 174: Models.
    async fn models(&self, ctx: &Context<'_>) -> Result<api::ModelsTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ModelsTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 175: Sensor Cal GNSS.
    async fn sensor_cal_gnss(&self, ctx: &Context<'_>) -> Result<api::SensorCalGnssTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorCalGnssTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 176: Sensor Cal HSS.
    async fn sensor_cal_hss(&self, ctx: &Context<'_>) -> Result<api::SensorCalHssTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorCalHssTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 177: Sensor Cal Mag.
    async fn sensor_cal_mag(&self, ctx: &Context<'_>) -> Result<api::SensorCalMagTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorCalMagTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 178: Sensor Cal FSS.
    async fn sensor_cal_fss(&self, ctx: &Context<'_>) -> Result<api::SensorCalFssTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorCalFssTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 179: HSS CubeSense Earth Raw.
    async fn hss_cubesense_earth_raw(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::HssCubesenseEarthRawTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HssCubesenseEarthRawTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 180: Sensor Raw Mag.
    async fn sensor_raw_mag(&self, ctx: &Context<'_>) -> Result<api::SensorRawMagTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorRawMagTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 181: Reference RPY Values.
    async fn reference_rpy_values(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ReferenceRpyValuesTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ReferenceRpyValuesTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 182: OpenLoopCommandMtq.
    async fn openloopcommandmtq(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::OpenloopcommandmtqTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::OpenloopcommandmtqTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 183: PowerState.
    async fn powerstate(&self, ctx: &Context<'_>) -> Result<api::PowerstateTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::PowerstateTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 184: ADCS Run Mode.
    async fn adcs_run_mode(&self, ctx: &Context<'_>) -> Result<api::AdcsRunModeTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::AdcsRunModeTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 185: Control Mode.
    async fn control_mode(&self, ctx: &Context<'_>) -> Result<api::ControlModeTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ControlModeTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 186: Wheel Configuration.
    async fn wheel_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::WheelConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::WheelConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 189: ADCS Satellite Configuration.
    async fn adcs_satellite_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::AdcsSatelliteConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::AdcsSatelliteConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 190: ADCS Controller Configuration.
    async fn adcs_controller_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::AdcsControllerConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::AdcsControllerConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 191: Mag0 Orbit Calibration Config.
    async fn mag0_orbit_calibration_config(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::Mag0OrbitCalibrationConfigTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::Mag0OrbitCalibrationConfigTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 192: Default Mode Configuration.
    async fn default_mode_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::DefaultModeConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::DefaultModeConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 193: Mounting Configuration.
    async fn mounting_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::MountingConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::MountingConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 194: Mag1 Orbit Calibration Config.
    async fn mag1_orbit_calibration_config(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::Mag1OrbitCalibrationConfigTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::Mag1OrbitCalibrationConfigTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 195: ADCS Estimator Configuration.
    async fn adcs_estimator_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::AdcsEstimatorConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::AdcsEstimatorConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 196: Satellite Orbit Parameters.
    async fn satellite_orbit_parameters(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::SatelliteOrbitParametersTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SatelliteOrbitParametersTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 197: Node Selection Configuration.
    async fn node_selection_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::NodeSelectionConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::NodeSelectionConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 198: Magnetorquer Configuration.
    async fn magnetorquer_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::MagnetorquerConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::MagnetorquerConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 199: Estimation Mode.
    async fn estimation_mode(&self, ctx: &Context<'_>) -> Result<api::EstimationModeTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::EstimationModeTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 202: OpenLoopCommandRwl.
    async fn openloopcommandrwl(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::OpenloopcommandrwlTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::OpenloopcommandrwlTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 203: Raw CSS Sensor Telemetry.
    async fn raw_css_sensor_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::RawCssSensorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RawCssSensorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 204: Sensor Raw Gyro.
    async fn sensor_raw_gyro(&self, ctx: &Context<'_>) -> Result<api::SensorRawGyroTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorRawGyroTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 205: Sensor Raw RWL.
    async fn sensor_raw_rwl(&self, ctx: &Context<'_>) -> Result<api::SensorRawRwlTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorRawRwlTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 206: Calibrated CSS Sensor Telemetry.
    async fn calibrated_css_sensor_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CalibratedCssSensorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CalibratedCssSensorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 207: Sensor Cal Gyro.
    async fn sensor_cal_gyro(&self, ctx: &Context<'_>) -> Result<api::SensorCalGyroTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorCalGyroTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 209: Sensor Cal RWL.
    async fn sensor_cal_rwl(&self, ctx: &Context<'_>) -> Result<api::SensorCalRwlTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorCalRwlTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 210: Main Estimator.
    async fn main_estimator(&self, ctx: &Context<'_>) -> Result<api::MainEstimatorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::MainEstimatorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 211: Estimator Main High Res.
    async fn estimator_main_high_res(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::EstimatorMainHighResTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::EstimatorMainHighResTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 212: Sensor Raw GNSS.
    async fn sensor_raw_gnss(&self, ctx: &Context<'_>) -> Result<api::SensorRawGnssTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SensorRawGnssTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 215: OpenLoopCommandHxyzRW.
    async fn openloopcommandhxyzrw(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::OpenloopcommandhxyzrwTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::OpenloopcommandhxyzrwTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 216: CubeComputer Health.
    async fn cubecomputer_health(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CubecomputerHealthTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CubecomputerHealthTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 217: HSS Health.
    async fn hss_health(&self, ctx: &Context<'_>) -> Result<api::HssHealthTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HssHealthTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 218: RWL Health.
    async fn rwl_health(&self, ctx: &Context<'_>) -> Result<api::RwlHealthTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RwlHealthTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 219: Data Frame.
    async fn data_frame(&self, ctx: &Context<'_>) -> Result<api::DataFrameTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::DataFrameTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 220: Image Frame Information.
    async fn image_frame_information(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ImageFrameInformationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ImageFrameInformationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 221: Mag Sensing Element Configuration.
    async fn mag_sensing_element_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::MagSensingElementConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::MagSensingElementConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 227: Telemetry Log Masks.
    async fn telemetry_log_masks(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::TelemetryLogMasksTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::TelemetryLogMasksTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 228: Unsolicited Telemetry Setup.
    async fn unsolicited_telemetry_setup(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::UnsolicitedTelemetrySetupTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::UnsolicitedTelemetrySetupTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 229: Pass Through TCTLM.
    async fn pass_through_tctlm(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::PassThroughTctlmTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::PassThroughTctlmTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 230: Component Error Codes.
    async fn component_error_codes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ComponentErrorCodesTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ComponentErrorCodesTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 231: Image File Info.
    async fn image_file_info(&self, ctx: &Context<'_>) -> Result<api::ImageFileInfoTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ImageFileInfoTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 232: Image Transfer Status.
    async fn image_transfer_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ImageTransferStatusTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ImageTransferStatusTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 233: Unsolicited Event Setup.
    async fn unsolicited_event_setup(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::UnsolicitedEventSetupTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::UnsolicitedEventSetupTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 234: Telemetry Log Status.
    async fn telemetry_log_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::TelemetryLogStatusTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::TelemetryLogStatusTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 235: Event Log Status.
    async fn event_log_status(&self, ctx: &Context<'_>) -> Result<api::EventLogStatusTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::EventLogStatusTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 237: Node Initialization States.
    async fn node_initialization_states(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::NodeInitializationStatesTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::NodeInitializationStatesTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 238: Expected Nodes.
    async fn expected_nodes(&self, ctx: &Context<'_>) -> Result<api::ExpectedNodesTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ExpectedNodesTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 239: Port Map.
    async fn port_map(&self, ctx: &Context<'_>) -> Result<api::PortMapTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::PortMapTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 240: Port Diagnostics.
    async fn port_diagnostics(&self, ctx: &Context<'_>) -> Result<api::PortDiagnosticsTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::PortDiagnosticsTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 241: File Transfer Setup.
    async fn file_transfer_setup(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::FileTransferSetupTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::FileTransferSetupTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 128: Identification.
    async fn identification(&self, ctx: &Context<'_>) -> Result<api::IdentificationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::IdentificationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 129: Serial Number.
    async fn serial_number(&self, ctx: &Context<'_>) -> Result<api::SerialNumberTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SerialNumberTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 135: Communication Status.
    async fn communication_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CommunicationStatusTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CommunicationStatusTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 137: Boot Status.
    async fn boot_status(&self, ctx: &Context<'_>) -> Result<api::BootStatusTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::BootStatusTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 138: Telecommand Acknowledge.
    async fn telecommand_acknowledge(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::TelecommandAcknowledgeTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::TelecommandAcknowledgeTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 149: Reference Rotation Angle.
    async fn reference_rotation_angle(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ReferenceRotationAngleTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ReferenceRotationAngleTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 150: Control and estimation mode.
    async fn control_and_estimation_mode(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ControlAndEstimationModeTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ControlAndEstimationModeTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 153: Health telemetry for CubeAuriga.
    async fn health_telemetry_for_cubeauriga(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::HealthTelemetryForCubeaurigaTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HealthTelemetryForCubeaurigaTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 154: Raw CubeAuriga telemetry.
    async fn raw_cubeauriga_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::RawCubeaurigaTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RawCubeaurigaTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 155: Reference parameters for FMC scan.
    async fn reference_parameters_for_fmc_scan(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ReferenceParametersForFmcScanTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ReferenceParametersForFmcScanTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 156: Reference IRC vector.
    async fn reference_irc_vector(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ReferenceIrcVectorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ReferenceIrcVectorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 157: Reference LLH target command.
    async fn reference_llh_target_command(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::ReferenceLlhTargetCommandTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::ReferenceLlhTargetCommandTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 158: GNSS UART Status.
    async fn gnss_uart_status(&self, ctx: &Context<'_>) -> Result<api::GnssUartStatusTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::GnssUartStatusTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 159: CubeNode-Quad PortMap.
    async fn cubenode_quad_portmap(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CubenodeQuadPortmapTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CubenodeQuadPortmapTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 160: Raw CubeStar telemetry.
    async fn raw_cubestar_telemetry(&self, ctx: &Context<'_>) -> Result<api::RawCubestarTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RawCubestarTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 162: Orbit mode.
    async fn orbit_mode(&self, ctx: &Context<'_>) -> Result<api::OrbitModeTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::OrbitModeTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 163: Current execution point.
    async fn current_execution_point(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CurrentExecutionPointTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CurrentExecutionPointTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 164: HIL telemetry.
    async fn hil_telemetry(&self, ctx: &Context<'_>) -> Result<api::HilTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HilTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 165: Health telemetry for CubeStar.
    async fn health_telemetry_for_cubestar(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::HealthTelemetryForCubestarTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HealthTelemetryForCubestarTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 166: Health telemetry for CubeNode PST3S.
    async fn health_telemetry_for_cubenode_pst3s(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::HealthTelemetryForCubenodePst3sTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HealthTelemetryForCubenodePst3sTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 171: Raw external sensor telemetry.
    async fn raw_external_sensor_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::RawExternalSensorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RawExternalSensorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 187: Target satellite orbit parameter configuration.
    async fn target_satellite_orbit_parameter_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::TargetSatelliteOrbitParameterConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::TargetSatelliteOrbitParameterConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 188: Augmented SGP4 configuration.
    async fn augmented_sgp4_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::AugmentedSgp4ConfigurationTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::AugmentedSgp4ConfigurationTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 200: ADCS operational state.
    async fn adcs_operational_state(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::AdcsOperationalStateTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::AdcsOperationalStateTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 201: Simulation raw sensor telemetry.
    async fn simulation_raw_sensor_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::SimulationRawSensorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::SimulationRawSensorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 208: Calibrated STR sensor telemetry.
    async fn calibrated_str_sensor_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::CalibratedStrSensorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::CalibratedStrSensorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 213: Raw PST3S star tracker telemetry.
    async fn raw_pst3s_star_tracker_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::RawPst3sStarTrackerTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RawPst3sStarTrackerTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 214: ACP execution telemetry.
    async fn acp_execution_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::AcpExecutionTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::AcpExecutionTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 225: Health telemetry for CubeNode NSSRWL.
    async fn health_telemetry_for_cubenode_nssrwl(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::HealthTelemetryForCubenodeNssrwlTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HealthTelemetryForCubenodeNssrwlTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 226: Raw NSSRWL sensor telemetry.
    async fn raw_nssrwl_sensor_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::RawNssrwlSensorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RawNssrwlSensorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 236: RAW LITEF uFORS sensor telemetry.
    async fn raw_litef_ufors_sensor_telemetry(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::RawLitefUforsSensorTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::RawLitefUforsSensorTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 242: File Info.
    async fn file_info(&self, ctx: &Context<'_>) -> Result<api::FileInfoTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::FileInfoTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 243: Health telemetry for CubeNode LITEFUFORS.
    async fn health_telemetry_for_cubenode_litefufors(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::HealthTelemetryForCubenodeLitefuforsTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::HealthTelemetryForCubenodeLitefuforsTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    /// Requests and decodes telemetry ID 244: ASGP4 Orbital Parameters.
    async fn asgp4_orbital_parameters(
        &self,
        ctx: &Context<'_>,
    ) -> Result<api::Asgp4OrbitalParametersTelemetry> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .request_typed_telemetry::<api::Asgp4OrbitalParametersTelemetry>()
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

/// Generated GraphQL mutations, one per telecommand ID.
#[derive(Default)]
pub struct GeneratedMutationRoot;

#[Object]
impl GeneratedMutationRoot {
    /// Sends telecommand ID 1: Reset.
    async fn reset(
        &self,
        ctx: &Context<'_>,
        input: api::ResetCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            1,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 2: Current Unix Time.
    async fn current_unix_time(
        &self,
        ctx: &Context<'_>,
        input: api::CurrentUnixTimeCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            2,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 5: Error Log Clear.
    async fn error_log_clear(&self, ctx: &Context<'_>) -> Result<api::CommandResponse> {
        let input = api::ErrorLogClearCommand::default();
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            5,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 7: Persist Config.
    async fn persist_config(&self, ctx: &Context<'_>) -> Result<api::CommandResponse> {
        let input = api::PersistConfigCommand::default();
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            7,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 42: Control and estimation mode.
    async fn control_and_estimation_mode(
        &self,
        ctx: &Context<'_>,
        input: api::ControlAndEstimationModeCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            42,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 49: Commanded GNSS Measurements.
    async fn commanded_gnss_measurements(
        &self,
        ctx: &Context<'_>,
        input: api::CommandedGnssMeasurementsCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            49,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 51: Orbit Mode.
    async fn orbit_mode(
        &self,
        ctx: &Context<'_>,
        input: api::OrbitModeCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            51,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 52: Mag Deploy Command.
    async fn mag_deploy_command(
        &self,
        ctx: &Context<'_>,
        input: api::MagDeployCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            52,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 54: Reference RPY Values.
    async fn reference_rpy_values(
        &self,
        ctx: &Context<'_>,
        input: api::ReferenceRpyValuesCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            54,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 55: OpenLoopCommandMtq.
    async fn openloopcommandmtq(
        &self,
        ctx: &Context<'_>,
        input: api::OpenloopcommandmtqCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            55,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 56: PowerState.
    async fn powerstate(
        &self,
        ctx: &Context<'_>,
        input: api::PowerstateCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            56,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 57: ADCS Run Mode.
    async fn adcs_run_mode(
        &self,
        ctx: &Context<'_>,
        input: api::AdcsRunModeCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            57,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 58: Control Mode.
    async fn control_mode(
        &self,
        ctx: &Context<'_>,
        input: api::ControlModeCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            58,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 59: Wheel Configuration.
    async fn wheel_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::WheelConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            59,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 61: ADCS Satellite Config.
    async fn adcs_satellite_config(
        &self,
        ctx: &Context<'_>,
        input: api::AdcsSatelliteConfigCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            61,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 63: Mag0 Orbit Calibration Config.
    async fn mag0_orbit_calibration_config(
        &self,
        ctx: &Context<'_>,
        input: api::Mag0OrbitCalibrationConfigCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            63,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 64: Default Mode Configuration.
    async fn default_mode_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::DefaultModeConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            64,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 65: Mounting Configuration.
    async fn mounting_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::MountingConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            65,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 66: Mag1 Orbit Calibration Config.
    async fn mag1_orbit_calibration_config(
        &self,
        ctx: &Context<'_>,
        input: api::Mag1OrbitCalibrationConfigCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            66,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 67: ADCS Estimator Config.
    async fn adcs_estimator_config(
        &self,
        ctx: &Context<'_>,
        input: api::AdcsEstimatorConfigCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            67,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 68: Satellite Orbit Parameters.
    async fn satellite_orbit_parameters(
        &self,
        ctx: &Context<'_>,
        input: api::SatelliteOrbitParametersCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            68,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 69: Node Selection Configuration.
    async fn node_selection_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::NodeSelectionConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            69,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 70: Magnetorquer Configuration.
    async fn magnetorquer_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::MagnetorquerConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            70,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 71: Estimation Mode.
    async fn estimation_mode(
        &self,
        ctx: &Context<'_>,
        input: api::EstimationModeCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            71,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 74: OpenLoopCommandRwl.
    async fn openloopcommandrwl(
        &self,
        ctx: &Context<'_>,
        input: api::OpenloopcommandrwlCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            74,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 76: OpenLoopCommandHxyzRW.
    async fn openloopcommandhxyzrw(
        &self,
        ctx: &Context<'_>,
        input: api::OpenloopcommandhxyzrwCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            76,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 77: Mag Sensing Element Configuration.
    async fn mag_sensing_element_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::MagSensingElementConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            77,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 78: Data Frame.
    async fn data_frame(
        &self,
        ctx: &Context<'_>,
        input: api::DataFrameCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            78,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 117: Request Telemetry Log Transfer Setup.
    async fn request_telemetry_log_transfer_setup(
        &self,
        ctx: &Context<'_>,
        input: api::RequestTelemetryLogTransferSetupCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            117,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 120: Initiate Filtered Event Log Transfer.
    async fn initiate_filtered_event_log_transfer(
        &self,
        ctx: &Context<'_>,
        input: api::InitiateFilteredEventLogTransferCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            120,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 50: Augmented SGP4 configuration.
    async fn augmented_sgp4_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::AugmentedSgp4ConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            50,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 62: ADCS controller configuration.
    async fn adcs_controller_configuration(
        &self,
        ctx: &Context<'_>,
        input: api::AdcsControllerConfigurationCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            62,
            context.subsystem().send_typed_command(&input),
        ))
    }

    /// Sends telecommand ID 72: ADCS operational state.
    async fn adcs_operational_state(
        &self,
        ctx: &Context<'_>,
        input: api::AdcsOperationalStateCommand,
    ) -> Result<api::CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(map_command_ack(
            72,
            context.subsystem().send_typed_command(&input),
        ))
    }
}
