use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct IModelaMillDriver;

impl LaserDriver for IModelaMillDriver {
    fn model_name(&self) -> &'static str {
        "IModela Mill"
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
        issues.push(DriverValidationIssue::warning(
            "IModela profile is milling-oriented; spindle/laser semantics may differ",
        ));
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
