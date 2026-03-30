use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;

use super::{DriverError, DriverValidationIssue, LaserDriver, validate_common_job};
use crate::laser::job::LaserJob;

#[derive(Default)]
pub(crate) struct DummyDriver;

impl LaserDriver for DummyDriver {
    fn model_name(&self) -> &'static str {
        "Dummy Driver"
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
            "Dummy driver is for debug/testing and does not target real hardware",
        ));
        Ok(issues)
    }

    fn prepare_program(
        &self,
        job: &LaserJob,
        _machine: &MachineProfile,
    ) -> Result<Vec<String>, DriverError> {
        Ok(job.lines.clone())
    }
}
