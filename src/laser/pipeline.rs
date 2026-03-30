use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::driver::{
    DriverError, DriverValidationIssue, LaserDriverProfile, create_driver,
    effective_driver_profile,
};
use super::job::LaserJob;

#[derive(Debug, Clone)]
pub struct PreparedProgram {
    pub driver_name: &'static str,
    pub driver_profile: LaserDriverProfile,
    pub lines: Vec<String>,
    pub validation_issues: Vec<DriverValidationIssue>,
}

pub fn prepare_program(
    controller_kind: ControllerKind,
    machine: &MachineProfile,
    job: &LaserJob,
) -> Result<PreparedProgram, DriverError> {
    let resolved_profile =
        effective_driver_profile(controller_kind, machine.laser_driver_profile);
    let driver = create_driver(controller_kind, machine.laser_driver_profile)?;

    let validation_issues = driver.validate_job(job, machine)?;
    if validation_issues
        .iter()
        .any(|issue| issue.severity == super::driver::DriverValidationSeverity::Error)
    {
        return Err(DriverError::Unsupported(
            "job validation contains blocking issues".to_string(),
        ));
    }

    let lines = driver.prepare_program(job, machine)?;

    Ok(PreparedProgram {
        driver_name: driver.model_name(),
        driver_profile: resolved_profile,
        lines,
        validation_issues,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::laser::driver::LaserDriverProfile;

    #[test]
    fn pipeline_prepares_grbl_program() {
        let machine = MachineProfile::default();
        let job = LaserJob::from_program_lines(
            &["G90".to_string(), "G1 X10 Y5 F1000".to_string()],
            "sample.gcode",
        );

        let prepared =
            prepare_program(ControllerKind::Grbl, &machine, &job).expect("pipeline should succeed");

        assert_eq!(prepared.driver_name, "GRBL Device-Safe");
        assert_eq!(prepared.driver_profile, LaserDriverProfile::GrblDeviceSafe);
        assert_eq!(prepared.lines.len(), 2);
    }

    #[test]
    fn pipeline_grbl_safe_strips_control_lines_golden() {
        let mut machine = MachineProfile::default();
        machine.laser_driver_profile = LaserDriverProfile::GrblDeviceSafe;
        let job = LaserJob::from_program_lines(
            &[
                "$X".to_string(),
                "?".to_string(),
                "G1 X10 Y5 F1000".to_string(),
            ],
            "sample.gcode",
        );

        let prepared =
            prepare_program(ControllerKind::Grbl, &machine, &job).expect("pipeline should succeed");

        let expected = vec!["G90".to_string(), "G1 X10 Y5 F1000".to_string()];
        assert_eq!(prepared.lines, expected);
    }

    #[test]
    fn pipeline_marlin_normalizes_m4_golden() {
        let mut machine = MachineProfile::default();
        machine.controller_kind = ControllerKind::Marlin;
        machine.laser_driver_profile = LaserDriverProfile::MarlinLineProtocol;
        let job = LaserJob::from_program_lines(
            &["M4 S500".to_string(), "G1 X10 Y5 F1000".to_string()],
            "sample.gcode",
        );

        let prepared = prepare_program(ControllerKind::Marlin, &machine, &job)
            .expect("pipeline should succeed");

        let expected = vec![
            "G21".to_string(),
            "M3 S500".to_string(),
            "G1 X10 Y5 F1000".to_string(),
        ];
        assert_eq!(prepared.lines, expected);
    }
}
