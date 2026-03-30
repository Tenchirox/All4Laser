use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct RuidaLineBridgeDriver;

fn sanitize_line(raw: &str) -> Option<String> {
    let without_comment = raw.split(';').next().unwrap_or_default().trim();
    if without_comment.is_empty() || without_comment == "?" || without_comment.starts_with('$') {
        return None;
    }
    Some(without_comment.to_string())
}

impl LaserDriver for RuidaLineBridgeDriver {
    fn model_name(&self) -> &'static str {
        "Ruida Line Bridge"
    }

    fn supports(&self, kind: ControllerKind) -> bool {
        kind == ControllerKind::Ruida
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        let mut issues = validate_common_job(job, machine)?;

        if job.lines.iter().any(|line| line.contains("$") || line.contains("?")) {
            issues.push(DriverValidationIssue::warning(
                "Ruida bridge strips GRBL realtime/config lines",
            ));
        }

        if job
            .lines
            .iter()
            .any(|line| line.contains("M7") || line.contains("M8") || line.contains("M9"))
        {
            issues.push(DriverValidationIssue::warning(
                "Ruida bridge may ignore air-assist M7/M8/M9 commands",
            ));
        }

        if job
            .lines
            .iter()
            .any(|line| line.contains("G2") || line.contains("G3"))
        {
            issues.push(DriverValidationIssue::warning(
                "Ruida bridge forwards arc moves (G2/G3) unchanged; verify controller support",
            ));
        }

        if job.lines.iter().any(|line| line.contains("G91")) {
            issues.push(DriverValidationIssue::warning(
                "Ruida bridge recommends absolute positioning; incremental mode G91 may be unsafe",
            ));
        }

        if job.lines.iter().any(|line| line.contains("M220") || line.contains("M221")) {
            issues.push(DriverValidationIssue::warning(
                "Ruida bridge strips feed/spindle override M220/M221 commands",
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
        let mut has_g90 = false;

        for line in &job.lines {
            let Some(clean) = sanitize_line(line) else {
                continue;
            };

            let mut tokens = Vec::new();
            for token in clean.split_whitespace() {
                let mapped = match token {
                    "M4" => "M3",
                    "M7" | "M8" | "M9" | "M220" | "M221" => continue,
                    _ => token,
                };
                if mapped == "G90" {
                    has_g90 = true;
                }
                tokens.push(mapped.to_string());
            }

            if tokens.is_empty() {
                continue;
            }

            out.push(tokens.join(" "));
        }

        if !has_g90 {
            out.insert(0, "G90".to_string());
        }

        Ok(out)
    }
}
