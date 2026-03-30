use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct K40NanoBridgeDriver;

impl LaserDriver for K40NanoBridgeDriver {
    fn model_name(&self) -> &'static str {
        "K40 Nano Bridge"
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
        if job.lines.iter().any(|line| line.contains("G2") || line.contains("G3")) {
            issues.push(DriverValidationIssue::warning(
                "K40 bridge may require arc flattening for G2/G3",
            ));
        }
        issues.push(DriverValidationIssue::warning(
            "K40 Nano Bridge is experimental and intended for adapter pipelines",
        ));
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        let mut out = Vec::new();
        out.push("G90".to_string());
        for line in &job.lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('$') || trimmed == "?" {
                continue;
            }
            out.push(line.replace("M4", "M3"));
        }
        Ok(out)
    }
}
