use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct MakeBlockXYPlotterDriver;

impl LaserDriver for MakeBlockXYPlotterDriver {
    fn model_name(&self) -> &'static str {
        "MakeBlock XY Plotter"
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
        if job.lines.iter().any(|line| line.contains("S")) {
            issues.push(DriverValidationIssue::warning(
                "MakeBlock profile may ignore spindle power S-values",
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
            let mut normalized = line.replace("M3", "").replace("M4", "").replace("M5", "");
            normalized = normalized.trim().to_string();
            if normalized.is_empty() {
                continue;
            }
            out.push(normalized);
        }
        Ok(out)
    }
}
