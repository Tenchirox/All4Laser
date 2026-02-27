use egui::Color32;
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CutMode {
    Line,
    Fill,
    FillAndLine,
    Offset,
}

impl Default for CutMode {
    fn default() -> Self {
        CutMode::Line
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CutLayer {
    pub id: usize,          // 0-29
    pub name: String,       // "C00", "C01", etc.
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub color: Color32,     // Visual color
    pub speed: f32,         // mm/min
    pub power: f32,         // 0-1000 (S-value)
    pub passes: u32,
    pub mode: CutMode,
    pub air_assist: bool,
    pub z_offset: f32,
    pub visible: bool,      // Output enabled?

    // Tabs / Bridges
    pub tab_enabled: bool,
    pub tab_spacing: f32,   // Distance between tabs in mm
    pub tab_size: f32,      // Size of the gap in mm
}

fn serialize_color<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rgba = color.to_array(); // [u8; 4]
    rgba.serialize(serializer)
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color32, D::Error>
where
    D: Deserializer<'de>,
{
    let rgba: [u8; 4] = <[u8; 4]>::deserialize(deserializer)?;
    Ok(Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]))
}

impl CutLayer {
    pub fn default_palette() -> Vec<CutLayer> {
        let mut layers = Vec::with_capacity(30);
        let colors = [
            Color32::from_rgb(0, 0, 0),       // C00 Black
            Color32::from_rgb(0, 0, 255),     // C01 Blue
            Color32::from_rgb(255, 0, 0),     // C02 Red
            Color32::from_rgb(0, 128, 0),     // C03 Green
            Color32::from_rgb(255, 165, 0),   // C04 Orange
            Color32::from_rgb(255, 0, 255),   // C05 Magenta
            Color32::from_rgb(0, 255, 255),   // C06 Cyan
            Color32::from_rgb(128, 0, 128),   // C07 Purple
            Color32::from_rgb(165, 42, 42),   // C08 Brown
            Color32::from_rgb(128, 128, 128), // C09 Gray
            Color32::from_rgb(255, 192, 203), // C10 Pink
            Color32::from_rgb(255, 215, 0),   // C11 Gold
            Color32::from_rgb(0, 100, 0),     // C12 Dark Green
            Color32::from_rgb(0, 0, 139),     // C13 Dark Blue
            Color32::from_rgb(139, 0, 0),     // C14 Dark Red
            Color32::from_rgb(75, 0, 130),    // C15 Indigo
            Color32::from_rgb(255, 20, 147),  // C16 Deep Pink
            Color32::from_rgb(0, 255, 0),     // C17 Lime
            Color32::from_rgb(0, 255, 127),   // C18 Spring Green
            Color32::from_rgb(70, 130, 180),  // C19 Steel Blue
            Color32::from_rgb(210, 105, 30),  // C20 Chocolate
            Color32::from_rgb(255, 99, 71),   // C21 Tomato
            Color32::from_rgb(123, 104, 238), // C22 Medium Slate Blue
            Color32::from_rgb(60, 179, 113),  // C23 Medium Sea Green
            Color32::from_rgb(255, 140, 0),   // C24 Dark Orange
            Color32::from_rgb(148, 0, 211),   // C25 Dark Violet
            Color32::from_rgb(107, 142, 35),  // C26 Olive Drab
            Color32::from_rgb(220, 20, 60),   // C27 Crimson
            Color32::from_rgb(0, 206, 209),   // C28 Dark Turquoise
            Color32::from_rgb(184, 134, 11),  // C29 Dark Goldenrod
        ];

        for (i, &col) in colors.iter().enumerate() {
            layers.push(CutLayer {
                id: i,
                name: format!("C{:02}", i),
                color: col,
                speed: 1000.0,
                power: 500.0,
                passes: 1,
                mode: CutMode::Line,
                air_assist: false,
                z_offset: 0.0,
                visible: true,
                tab_enabled: false,
                tab_spacing: 50.0,
                tab_size: 0.5,
            });
        }
        layers
    }
}
