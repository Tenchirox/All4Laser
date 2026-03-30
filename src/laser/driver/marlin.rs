use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct MarlinLineDriver;

impl LaserDriver for MarlinLineDriver {
    fn model_name(&self) -> &'static str {
        "Marlin Line Protocol"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Marlin
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        let mut issues = validate_common_job(job, machine)?;

        if job.lines.iter().any(|line| line.contains("$") || line.contains("?")) {
            issues.push(DriverValidationIssue::warning(
                "Marlin driver ignores GRBL realtime/config lines",
            ));
        }

        if job.lines.iter().any(|line| line.contains("M4")) {
            issues.push(DriverValidationIssue::warning(
                "Marlin driver will normalize M4 dynamic mode to M3",
            ));
        }

        if !job.lines.iter().any(|line| line.contains("G21")) {
            issues.push(DriverValidationIssue::warning(
                "Marlin driver recommends explicit mm units (G21)",
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
            if trimmed.is_empty() || trimmed == "?" || trimmed.starts_with('$') {
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
