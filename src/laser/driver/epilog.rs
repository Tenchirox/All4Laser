use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct EpilogZingBridgeDriver;

#[derive(Default)]
pub(crate) struct EpilogHelixBridgeDriver;

fn prepare_epilog_lines(job: &LaserJob) -> Vec<String> {
    let mut out = vec![";EPILOG-BEGIN".to_string()];
    for line in &job.lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('$') || trimmed == "?" {
            continue;
        }
        out.push(line.replace("M4", "M3"));
    }
    out.push(";EPILOG-END".to_string());
    out
}

fn epilog_validate(
    job: &LaserJob,
    machine: &MachineProfile,
    model: &str,
) -> Result<Vec<DriverValidationIssue>, DriverError> {
    let mut issues = validate_common_job(job, machine)?;
    issues.push(DriverValidationIssue::warning(format!(
        "{model} bridge is experimental and expects a dedicated spooler",
    )));
    Ok(issues)
}

impl LaserDriver for EpilogZingBridgeDriver {
    fn model_name(&self) -> &'static str {
        "Epilog Zing Bridge"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Ruida || kind == ControllerKind::Trocen
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        epilog_validate(job, machine, "Epilog Zing")
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        Ok(prepare_epilog_lines(job))
    }
}

impl LaserDriver for EpilogHelixBridgeDriver {
    fn model_name(&self) -> &'static str {
        "Epilog Helix Bridge"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Ruida || kind == ControllerKind::Trocen
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        epilog_validate(job, machine, "Epilog Helix")
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        Ok(prepare_epilog_lines(job))
    }
}
