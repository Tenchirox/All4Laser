use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct GrblGenericDriver;

impl LaserDriver for GrblGenericDriver {
    fn model_name(&self) -> &'static str {
        "GRBL Generic"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Grbl
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        Ok(job.lines.clone())
    }
}

#[derive(Default)]
pub(crate) struct GrblDeviceSafeDriver;

impl LaserDriver for GrblDeviceSafeDriver {
    fn model_name(&self) -> &'static str {
        "GRBL Device-Safe"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Grbl
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        let mut issues = validate_common_job(job, machine)?;
        if job
            .lines
            .iter()
            .any(|line| line.trim_start().starts_with('$') || line.trim() == "?")
        {
            issues.push(DriverValidationIssue::warning(
                "device-safe mode will strip GRBL realtime/config lines from execution stream",
            ));
        }
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        let mut out: Vec<String> = job
            .lines
            .iter()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed == "?" || trimmed.starts_with('$') {
                    None
                } else {
                    Some(line.clone())
                }
            })
            .collect();
        if !out.iter().any(|line| line.trim().starts_with("G90")) {
            out.insert(0, "G90".to_string());
        }
        Ok(out)
    }
}
