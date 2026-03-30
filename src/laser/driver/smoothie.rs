use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct SmoothieGcodeDriver;

impl LaserDriver for SmoothieGcodeDriver {
    fn model_name(&self) -> &'static str {
        "Smoothie GCode"
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
        if !job.lines.iter().any(|line| line.contains("G21")) {
            issues.push(DriverValidationIssue::warning(
                "Smoothie profile recommends explicit metric mode with G21",
            ));
        }
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        let mut out = Vec::new();
        let mut has_g21 = false;

        for line in &job.lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('$') || trimmed == "?" {
                continue;
            }
            let normalized = line.replace("M4", "M3");
            if normalized.contains("G21") {
                has_g21 = true;
            }
            out.push(normalized);
        }

        if !has_g21 {
            out.insert(0, "G21".to_string());
        }

        Ok(out)
    }
}
