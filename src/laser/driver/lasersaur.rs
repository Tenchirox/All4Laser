use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct LasersaurGcodeDriver;

impl LaserDriver for LasersaurGcodeDriver {
    fn model_name(&self) -> &'static str {
        "Lasersaur GCode"
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
        if job.lines.iter().any(|line| line.contains("M4")) {
            issues.push(DriverValidationIssue::warning(
                "Lasersaur profile normalizes M4 to M3",
            ));
        }
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        Ok(job
            .lines
            .iter()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('$') || trimmed == "?" {
                    None
                } else {
                    Some(line.replace("M4", "M3"))
                }
            })
            .collect())
    }
}
