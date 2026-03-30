use crate::config::machine_profile::MachineProfile;
use crate::gcode::file::GCodeFile;

#[derive(Clone, Debug, Default)]
pub struct RasterPartConfig {
    pub padding_mm: f32,
    pub overscan_mm: f32,
}

#[derive(Clone, Debug)]
pub enum LaserJobPart {
    Vector { lines: Vec<String> },
    Raster {
        lines: Vec<String>,
        config: RasterPartConfig,
    },
}

#[derive(Clone, Debug, Default)]
pub struct JobMetadata {
    pub title: String,
    pub source_name: String,
    pub user: String,
}

#[derive(Clone, Debug)]
pub struct LaserJob {
    pub metadata: JobMetadata,
    pub lines: Vec<String>,
    pub parts: Vec<LaserJobPart>,
    pub start_x_mm: f32,
    pub start_y_mm: f32,
    pub rotary_enabled: bool,
    pub rotary_diameter_mm: Option<f32>,
}

impl LaserJob {
    pub fn new(metadata: JobMetadata, lines: Vec<String>) -> Self {
        Self {
            metadata,
            parts: vec![LaserJobPart::Vector {
                lines: lines.clone(),
            }],
            lines,
            start_x_mm: 0.0,
            start_y_mm: 0.0,
            rotary_enabled: false,
            rotary_diameter_mm: None,
        }
    }

    pub fn from_program_lines(lines: &[String], source_name: impl Into<String>) -> Self {
        Self::new(
            JobMetadata {
                title: "Untitled Job".to_string(),
                source_name: source_name.into(),
                user: "unknown".to_string(),
            },
            lines.to_vec(),
        )
    }

    pub fn from_parts(metadata: JobMetadata, parts: Vec<LaserJobPart>) -> Self {
        let mut lines = Vec::new();
        for part in &parts {
            match part {
                LaserJobPart::Vector { lines: part_lines } => lines.extend(part_lines.iter().cloned()),
                LaserJobPart::Raster { lines: part_lines, .. } => {
                    lines.extend(part_lines.iter().cloned())
                }
            }
        }

        Self {
            metadata,
            lines,
            parts,
            start_x_mm: 0.0,
            start_y_mm: 0.0,
            rotary_enabled: false,
            rotary_diameter_mm: None,
        }
    }

    pub fn add_part(&mut self, part: LaserJobPart) {
        match &part {
            LaserJobPart::Vector { lines } => self.lines.extend(lines.iter().cloned()),
            LaserJobPart::Raster { lines, .. } => self.lines.extend(lines.iter().cloned()),
        }
        self.parts.push(part);
    }

    pub fn is_empty(&self) -> bool {
        self.lines
            .iter()
            .all(|line| {
                let trimmed = line.trim();
                trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('(')
            })
    }

    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    pub fn bounds_mm(&self) -> Option<(f32, f32, f32, f32)> {
        let file = GCodeFile::from_lines(&self.metadata.source_name, &self.lines);
        file.bounds()
    }

    pub fn exceeds_workspace(&self, machine: &MachineProfile) -> bool {
        let Some((min_x, min_y, max_x, max_y)) = self.bounds_mm() else {
            return false;
        };
        min_x < 0.0 || min_y < 0.0 || max_x > machine.workspace_x_mm || max_y > machine.workspace_y_mm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_job_detects_comments_only() {
        let job = LaserJob::from_program_lines(
            &[";comment".to_string(), "   ".to_string(), "(meta)".to_string()],
            "test.gcode",
        );
        assert!(job.is_empty());
    }
}
