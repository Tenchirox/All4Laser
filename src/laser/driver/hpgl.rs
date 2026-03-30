use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct GoldCutHpglDriver;

impl LaserDriver for GoldCutHpglDriver {
    fn model_name(&self) -> &'static str {
        "GoldCut HPGL"
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
        issues.push(DriverValidationIssue::warning(
            "GoldCut HPGL profile converts moves into HPGL-like pen commands",
        ));
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        let mut out = vec!["IN;".to_string(), "PA;".to_string()];
        for line in &job.lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('$') || trimmed == "?" {
                continue;
            }
            if trimmed.starts_with("G0") {
                out.push(format!("PU ; {}", trimmed));
            } else if trimmed.starts_with("G1") {
                out.push(format!("PD ; {}", trimmed));
            }
        }
        out.push("PU;".to_string());
        Ok(out)
    }
}
