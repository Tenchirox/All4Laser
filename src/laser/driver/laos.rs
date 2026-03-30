use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct LaosCutterBridgeDriver;

impl LaserDriver for LaosCutterBridgeDriver {
    fn model_name(&self) -> &'static str {
        "Laos Cutter Bridge"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Ruida || kind == ControllerKind::Trocen
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        let mut issues = validate_common_job(job, machine)?;
        if !job.lines.iter().any(|line| line.contains("G90")) {
            issues.push(DriverValidationIssue::warning(
                "Laos bridge expects absolute mode and will inject G90",
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
