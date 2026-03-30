use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct SampleDriver;

impl LaserDriver for SampleDriver {
    fn model_name(&self) -> &'static str {
        "Sample Driver"
    }

    fn supports(&self, _kind: ControllerKind) -> bool {
        true
    }

    fn validate_job(
        &self,
        job: &LaserJob,
        machine: &MachineProfile,
    ) -> Result<Vec<DriverValidationIssue>, DriverError> {
        let mut issues = validate_common_job(job, machine)?;
        issues.push(DriverValidationIssue::warning(
            "Sample driver is a template profile and may not match production hardware",
        ));
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        let mut out = vec![";SAMPLE-DRIVER".to_string()];
        out.extend(job.lines.iter().cloned());
        Ok(out)
    }
}
