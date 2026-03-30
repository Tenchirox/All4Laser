use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct K3EngraverDriver;

impl LaserDriver for K3EngraverDriver {
    fn model_name(&self) -> &'static str {
        "K3 Engraver"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Grbl || kind == ControllerKind::Marlin
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        let mut issues = validate_common_job(job, machine)?;
        if job.lines.iter().any(|line| line.contains("G2") || line.contains("G3")) {
            issues.push(DriverValidationIssue::warning(
                "K3 profile may require arc flattening for G2/G3",
            ));
        }
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        let mut out = vec!["G90".to_string()];
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
