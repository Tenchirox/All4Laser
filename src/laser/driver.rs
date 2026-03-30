use std::fmt;

use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::job::LaserJob;

mod grbl;
mod dummy;
mod sample;
mod imodela;
mod k3;
mod hpgl;
mod k40;
mod laos;
mod lasersaur;
mod lasertools;
mod makeblock;
mod marlin;
mod epilog;
mod fullspectrum;
mod ruida;
mod smoothie;
mod trocen;

use grbl::{GrblDeviceSafeDriver, GrblGenericDriver};
use dummy::DummyDriver;
use sample::SampleDriver;
use imodela::IModelaMillDriver;
use k3::K3EngraverDriver;
use hpgl::GoldCutHpglDriver;
use k40::K40NanoBridgeDriver;
use laos::LaosCutterBridgeDriver;
use lasersaur::LasersaurGcodeDriver;
use lasertools::LaserToolsTechnicsDriver;
use makeblock::MakeBlockXYPlotterDriver;
use marlin::MarlinLineDriver;
use epilog::{EpilogHelixBridgeDriver, EpilogZingBridgeDriver};
use fullspectrum::FullSpectrumBridgeDriver;
use ruida::RuidaLineBridgeDriver;
use smoothie::SmoothieGcodeDriver;
use trocen::TrocenLineBridgeDriver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverValidationSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct DriverValidationIssue {
    pub severity: DriverValidationSeverity,
    pub message: String,
}

impl DriverValidationIssue {
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: DriverValidationSeverity::Warning,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: DriverValidationSeverity::Error,
            message: message.into(),
        }
    }
}

pub fn effective_driver_profile(
    controller_kind: ControllerKind,
    requested_profile: LaserDriverProfile,
) -> LaserDriverProfile {
    resolve_driver_profile(controller_kind, requested_profile)
}

pub(crate) fn validate_common_job(
    job: &LaserJob,
    machine: &MachineProfile,
) -> Result<Vec<DriverValidationIssue>, DriverError> {
    let mut issues = Vec::new();

    if job.is_empty() {
        return Err(DriverError::EmptyJob);
    }

    if let Some((_, _, max_x, max_y)) = job.bounds_mm()
        && (max_x > machine.workspace_x_mm || max_y > machine.workspace_y_mm)
    {
        return Err(DriverError::JobOutOfBounds {
            max_x_mm: max_x,
            max_y_mm: max_y,
            workspace_x_mm: machine.workspace_x_mm,
            workspace_y_mm: machine.workspace_y_mm,
        });
    }

    if job.rotary_enabled {
        match job.rotary_diameter_mm {
            Some(v) if v > 0.0 => {}
            _ => {
                issues.push(DriverValidationIssue::error(
                    "rotary mode is enabled but diameter is missing or invalid",
                ));
            }
        }
    }

    Ok(issues)
}

#[derive(Debug, Clone)]
pub enum DriverError {
    EmptyJob,
    JobOutOfBounds {
        max_x_mm: f32,
        max_y_mm: f32,
        workspace_x_mm: f32,
        workspace_y_mm: f32,
    },
    UnsupportedController(ControllerKind),
    Unsupported(String),
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyJob => write!(f, "job has no executable GCode"),
            Self::JobOutOfBounds {
                max_x_mm,
                max_y_mm,
                workspace_x_mm,
                workspace_y_mm,
            } => write!(
                f,
                "job exceeds machine workspace ({max_x_mm:.2}x{max_y_mm:.2} mm > {workspace_x_mm:.2}x{workspace_y_mm:.2} mm)"
            ),
            Self::UnsupportedController(kind) => {
                write!(f, "no laser driver available for controller {kind:?}")
            }
            Self::Unsupported(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for DriverError {}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default,
)]
pub enum LaserDriverProfile {
    #[default]
    Auto,
    GrblGeneric,
    GrblDeviceSafe,
    MarlinLineProtocol,
    RuidaLineProtocol,
    TrocenLineProtocol,
    SmoothieGcode,
    LasersaurGcode,
    K40NanoBridge,
    GoldCutHpgl,
    EpilogZingBridge,
    EpilogHelixBridge,
    FullSpectrumBridge,
    LaosCutterBridge,
    MakeBlockXYPlotter,
    LaserToolsTechnics,
    Dummy,
    Sample,
    IModelaMill,
    K3Engraver,
}

impl LaserDriverProfile {
    pub fn label(self) -> &'static str {
        match self {
            Self::Auto => "Auto (by controller)",
            Self::GrblGeneric => "GRBL Generic",
            Self::GrblDeviceSafe => "GRBL Device-Safe",
            Self::MarlinLineProtocol => "Marlin Line Protocol",
            Self::RuidaLineProtocol => "Ruida Line Bridge",
            Self::TrocenLineProtocol => "Trocen Line Bridge",
            Self::SmoothieGcode => "Smoothie GCode",
            Self::LasersaurGcode => "Lasersaur GCode",
            Self::K40NanoBridge => "K40 Nano Bridge",
            Self::GoldCutHpgl => "GoldCut HPGL",
            Self::EpilogZingBridge => "Epilog Zing Bridge",
            Self::EpilogHelixBridge => "Epilog Helix Bridge",
            Self::FullSpectrumBridge => "Full Spectrum Bridge",
            Self::LaosCutterBridge => "Laos Cutter Bridge",
            Self::MakeBlockXYPlotter => "MakeBlock XY Plotter",
            Self::LaserToolsTechnics => "LaserTools Technics",
            Self::Dummy => "Dummy Driver",
            Self::Sample => "Sample Driver",
            Self::IModelaMill => "IModela Mill",
            Self::K3Engraver => "K3 Engraver",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Auto => "Automatically picks the recommended driver for the selected controller.",
            Self::GrblGeneric => "Pass-through GCode driver for GRBL-compatible firmware.",
            Self::GrblDeviceSafe => {
                "Safer GRBL mode: strips control lines and ensures absolute mode header."
            }
            Self::MarlinLineProtocol => "Marlin compatibility driver with laser command normalization.",
            Self::RuidaLineProtocol => "Line-oriented bridge for Ruida serial gateways.",
            Self::TrocenLineProtocol => "Line-oriented bridge for Trocen serial gateways.",
            Self::SmoothieGcode => "Smoothie-compatible GCode profile with safety normalization.",
            Self::LasersaurGcode => "Lasersaur-compatible GCode profile for open hardware cutters.",
            Self::K40NanoBridge => "Experimental K40 Nano bridge profile for command adaptation.",
            Self::GoldCutHpgl => "HPGL-oriented profile inspired by GoldCut controller workflows.",
            Self::EpilogZingBridge => "Experimental Epilog Zing bridge profile.",
            Self::EpilogHelixBridge => "Experimental Epilog Helix bridge profile.",
            Self::FullSpectrumBridge => "Experimental Full Spectrum bridge profile.",
            Self::LaosCutterBridge => "Laos cutter command bridge profile.",
            Self::MakeBlockXYPlotter => "MakeBlock XY plotter compatibility profile.",
            Self::LaserToolsTechnics => "LaserTools Technics speed/power normalized profile.",
            Self::Dummy => "No-op debug driver for pipeline dry validation.",
            Self::Sample => "Sample adapter profile for integration testing.",
            Self::IModelaMill => "IModela mill compatibility profile (experimental).",
            Self::K3Engraver => "K3 engraver compatibility profile (experimental).",
        }
    }
}

fn default_profile_for_controller(kind: ControllerKind) -> LaserDriverProfile {
    match kind {
        ControllerKind::Grbl => LaserDriverProfile::GrblDeviceSafe,
        ControllerKind::Marlin => LaserDriverProfile::MarlinLineProtocol,
        ControllerKind::Ruida => LaserDriverProfile::RuidaLineProtocol,
        ControllerKind::Trocen => LaserDriverProfile::TrocenLineProtocol,
    }
}

pub fn available_driver_profiles(kind: ControllerKind) -> Vec<LaserDriverProfile> {
    let mut profiles = vec![LaserDriverProfile::Auto];
    match kind {
        ControllerKind::Grbl => {
            profiles.push(LaserDriverProfile::GrblGeneric);
            profiles.push(LaserDriverProfile::GrblDeviceSafe);
            profiles.push(LaserDriverProfile::SmoothieGcode);
            profiles.push(LaserDriverProfile::LasersaurGcode);
            profiles.push(LaserDriverProfile::K40NanoBridge);
            profiles.push(LaserDriverProfile::MakeBlockXYPlotter);
            profiles.push(LaserDriverProfile::IModelaMill);
            profiles.push(LaserDriverProfile::K3Engraver);
            profiles.push(LaserDriverProfile::Dummy);
            profiles.push(LaserDriverProfile::Sample);
        }
        ControllerKind::Marlin => {
            profiles.push(LaserDriverProfile::MarlinLineProtocol);
            profiles.push(LaserDriverProfile::MakeBlockXYPlotter);
            profiles.push(LaserDriverProfile::LaserToolsTechnics);
            profiles.push(LaserDriverProfile::IModelaMill);
            profiles.push(LaserDriverProfile::K3Engraver);
            profiles.push(LaserDriverProfile::Dummy);
            profiles.push(LaserDriverProfile::Sample);
        }
        ControllerKind::Ruida => {
            profiles.push(LaserDriverProfile::RuidaLineProtocol);
            profiles.push(LaserDriverProfile::EpilogZingBridge);
            profiles.push(LaserDriverProfile::EpilogHelixBridge);
            profiles.push(LaserDriverProfile::FullSpectrumBridge);
            profiles.push(LaserDriverProfile::LaosCutterBridge);
            profiles.push(LaserDriverProfile::GoldCutHpgl);
            profiles.push(LaserDriverProfile::Dummy);
            profiles.push(LaserDriverProfile::Sample);
        }
        ControllerKind::Trocen => {
            profiles.push(LaserDriverProfile::TrocenLineProtocol);
            profiles.push(LaserDriverProfile::EpilogZingBridge);
            profiles.push(LaserDriverProfile::EpilogHelixBridge);
            profiles.push(LaserDriverProfile::FullSpectrumBridge);
            profiles.push(LaserDriverProfile::LaosCutterBridge);
            profiles.push(LaserDriverProfile::GoldCutHpgl);
            profiles.push(LaserDriverProfile::LaserToolsTechnics);
            profiles.push(LaserDriverProfile::Dummy);
            profiles.push(LaserDriverProfile::Sample);
        }
    }
    profiles
}

fn resolve_driver_profile(
    controller_kind: ControllerKind,
    requested_profile: LaserDriverProfile,
) -> LaserDriverProfile {
    let requested = if requested_profile == LaserDriverProfile::Auto {
        default_profile_for_controller(controller_kind)
    } else {
        requested_profile
    };

    if available_driver_profiles(controller_kind).contains(&requested) {
        requested
    } else {
        default_profile_for_controller(controller_kind)
    }
}

pub trait LaserDriver: Send + Sync {
    fn model_name(&self) -> &'static str;

    fn supports(&self, kind: ControllerKind) -> bool;

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        validate_common_job(job, machine)
    }

    fn prepare_program(&self, job: &LaserJob, machine: &MachineProfile) -> Result<Vec<String>, DriverError>;

    fn send_program(
        &self,
        lines: &[String],
        sender: &mut dyn DriverProgramSender,
    ) -> Result<(), DriverError> {
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('(') {
                continue;
            }
            sender.send_line(trimmed);
        }
        Ok(())
    }
}

pub trait DriverProgramSender {
    fn send_line(&mut self, line: &str);
}

pub fn create_driver(
    controller_kind: ControllerKind,
    profile: LaserDriverProfile,
) -> Result<Box<dyn LaserDriver>, DriverError> {
    let resolved = resolve_driver_profile(controller_kind, profile);
    let driver: Box<dyn LaserDriver> = match resolved {
        LaserDriverProfile::Auto => unreachable!(),
        LaserDriverProfile::GrblGeneric => Box::new(GrblGenericDriver),
        LaserDriverProfile::GrblDeviceSafe => Box::new(GrblDeviceSafeDriver),
        LaserDriverProfile::MarlinLineProtocol => Box::new(MarlinLineDriver),
        LaserDriverProfile::RuidaLineProtocol => Box::new(RuidaLineBridgeDriver),
        LaserDriverProfile::TrocenLineProtocol => Box::new(TrocenLineBridgeDriver),
        LaserDriverProfile::SmoothieGcode => Box::new(SmoothieGcodeDriver),
        LaserDriverProfile::LasersaurGcode => Box::new(LasersaurGcodeDriver),
        LaserDriverProfile::K40NanoBridge => Box::new(K40NanoBridgeDriver),
        LaserDriverProfile::GoldCutHpgl => Box::new(GoldCutHpglDriver),
        LaserDriverProfile::EpilogZingBridge => Box::new(EpilogZingBridgeDriver),
        LaserDriverProfile::EpilogHelixBridge => Box::new(EpilogHelixBridgeDriver),
        LaserDriverProfile::FullSpectrumBridge => Box::new(FullSpectrumBridgeDriver),
        LaserDriverProfile::LaosCutterBridge => Box::new(LaosCutterBridgeDriver),
        LaserDriverProfile::MakeBlockXYPlotter => Box::new(MakeBlockXYPlotterDriver),
        LaserDriverProfile::LaserToolsTechnics => Box::new(LaserToolsTechnicsDriver),
        LaserDriverProfile::Dummy => Box::new(DummyDriver),
        LaserDriverProfile::Sample => Box::new(SampleDriver),
        LaserDriverProfile::IModelaMill => Box::new(IModelaMillDriver),
        LaserDriverProfile::K3Engraver => Box::new(K3EngraverDriver),
    };

    if driver.supports(controller_kind) {
        Ok(driver)
    } else {
        Err(DriverError::UnsupportedController(controller_kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn machine() -> MachineProfile {
        MachineProfile {
            workspace_x_mm: 100.0,
            workspace_y_mm: 100.0,
            ..MachineProfile::default()
        }
    }

    #[test]
    fn validation_fails_on_empty_job() {
        let driver = create_driver(ControllerKind::Grbl, LaserDriverProfile::Auto).expect("driver");
        let job = LaserJob::new(Default::default(), vec![]);

        let err = driver
            .validate_job(&job, &machine())
            .expect_err("empty job must fail");
        assert!(matches!(err, DriverError::EmptyJob));
    }

    #[test]
    fn validation_warns_invalid_rotary_diameter() {
        let driver =
            create_driver(ControllerKind::Grbl, LaserDriverProfile::GrblGeneric).expect("driver");
        let mut job = LaserJob::from_program_lines(&["G1 X10 Y10".to_string()], "job.gcode");
        job.rotary_enabled = true;

        let issues = driver.validate_job(&job, &machine()).expect("validation should pass with issue");
        assert!(issues
            .iter()
            .any(|i| i.severity == DriverValidationSeverity::Error));
    }

    #[test]
    fn safe_grbl_driver_strips_control_lines() {
        let driver =
            create_driver(ControllerKind::Grbl, LaserDriverProfile::GrblDeviceSafe).expect("driver");
        let job = LaserJob::from_program_lines(
            &[
                "$X".to_string(),
                "?".to_string(),
                "G1 X10 Y10".to_string(),
            ],
            "job.gcode",
        );

        let lines = driver.prepare_program(&job, &machine()).expect("prepare should work");
        assert!(lines.iter().all(|line| !line.trim_start().starts_with('$')));
        assert!(lines.iter().all(|line| line.trim() != "?"));
    }

    #[test]
    fn marlin_driver_injects_units_and_normalizes_m4() {
        let driver =
            create_driver(ControllerKind::Marlin, LaserDriverProfile::MarlinLineProtocol)
                .expect("driver");
        let job = LaserJob::from_program_lines(
            &["M4 S300".to_string(), "G1 X10 Y10 F1200".to_string()],
            "job.gcode",
        );

        let lines = driver.prepare_program(&job, &machine()).expect("prepare should work");
        assert_eq!(lines.first().map(String::as_str), Some("G21"));
        assert!(lines.iter().any(|line| line.contains("M3 S300")));
    }

    #[test]
    fn ruida_driver_removes_air_assist_m_codes() {
        let driver =
            create_driver(ControllerKind::Ruida, LaserDriverProfile::RuidaLineProtocol)
                .expect("driver");
        let job = LaserJob::from_program_lines(
            &[
                "G1 X0 Y0 ; warmup move".to_string(),
                "M8".to_string(),
                "G1 X10 Y10".to_string(),
                "M220 S120".to_string(),
                "M9".to_string(),
            ],
            "job.gcode",
        );

        let lines = driver.prepare_program(&job, &machine()).expect("prepare should work");
        assert_eq!(lines.first().map(String::as_str), Some("G90"));
        assert!(lines.iter().all(|line| !line.contains("M8")));
        assert!(lines.iter().all(|line| !line.contains("M9")));
        assert!(lines.iter().all(|line| !line.contains("M220")));
        assert!(lines.iter().any(|line| line.contains("G1 X10 Y10")));
        assert!(lines.iter().all(|line| !line.contains(';')));
    }

    #[test]
    fn trocen_driver_maps_spindle_to_laser_commands() {
        let driver =
            create_driver(ControllerKind::Trocen, LaserDriverProfile::TrocenLineProtocol)
                .expect("driver");
        let job = LaserJob::from_program_lines(
            &[
                "M3 S400".to_string(),
                "M220 S120".to_string(),
                "M5".to_string(),
            ],
            "job.gcode",
        );

        let lines = driver.prepare_program(&job, &machine()).expect("prepare should work");
        assert_eq!(lines.first().map(String::as_str), Some("G90"));
        assert!(lines.iter().any(|line| line.contains("M106 S400")));
        assert!(lines.iter().any(|line| line.trim() == "M107"));
        assert!(lines.iter().all(|line| !line.contains("M220")));
    }

    #[test]
    fn additional_liblasercut_profiles_are_constructible() {
        let sample = LaserJob::from_program_lines(
            &["G1 X10 Y10 F1000".to_string(), "M4 S300".to_string()],
            "job.gcode",
        );

        let profiles = vec![
            (ControllerKind::Grbl, LaserDriverProfile::SmoothieGcode),
            (ControllerKind::Grbl, LaserDriverProfile::LasersaurGcode),
            (ControllerKind::Grbl, LaserDriverProfile::K40NanoBridge),
            (ControllerKind::Grbl, LaserDriverProfile::IModelaMill),
            (ControllerKind::Marlin, LaserDriverProfile::K3Engraver),
            (ControllerKind::Ruida, LaserDriverProfile::Dummy),
            (ControllerKind::Trocen, LaserDriverProfile::Sample),
            (ControllerKind::Ruida, LaserDriverProfile::GoldCutHpgl),
            (ControllerKind::Ruida, LaserDriverProfile::EpilogZingBridge),
            (ControllerKind::Trocen, LaserDriverProfile::EpilogHelixBridge),
            (ControllerKind::Ruida, LaserDriverProfile::FullSpectrumBridge),
            (ControllerKind::Trocen, LaserDriverProfile::LaosCutterBridge),
            (ControllerKind::Marlin, LaserDriverProfile::MakeBlockXYPlotter),
            (ControllerKind::Trocen, LaserDriverProfile::LaserToolsTechnics),
        ];

        for (kind, profile) in profiles {
            let driver = create_driver(kind, profile).expect("driver profile should be constructible");
            let lines = driver
                .prepare_program(&sample, &machine())
                .expect("prepare should succeed");
            assert!(!lines.is_empty(), "{profile:?} should emit non-empty output");
        }
    }
}
