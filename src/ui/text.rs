#![allow(dead_code)]

use crate::i18n::tr;
use crate::ui::drawing::{PathData, ShapeKind, ShapeParams};
use egui::{RichText, Ui};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use rusttype::{Font, OutlineBuilder, Scale, point};

const LIBERATION_SANS_REGULAR: &[u8] =
    include_bytes!("../../assets/fonts/LiberationSans-Regular.ttf");
const LIBERATION_SERIF_REGULAR: &[u8] =
    include_bytes!("../../assets/fonts/LiberationSerif-Regular.ttf");
const LIBERATION_MONO_REGULAR: &[u8] =
    include_bytes!("../../assets/fonts/LiberationMono-Regular.ttf");
const NOTO_SANS_REGULAR: &[u8] = include_bytes!("../../assets/fonts/NotoSans-Regular.ttf");
const NOTO_SERIF_REGULAR: &[u8] = include_bytes!("../../assets/fonts/NotoSerif-Regular.ttf");
const NOTO_SANS_MONO_REGULAR: &[u8] = include_bytes!("../../assets/fonts/NotoSansMono-Regular.ttf");
const NOTO_SANS_ARABIC_REGULAR: &[u8] =
    include_bytes!("../../assets/fonts/NotoSansArabic-Regular.ttf");

const BUNDLED_FONTS: [(&str, &[u8]); 7] = [
    ("Liberation Sans (Bundled)", LIBERATION_SANS_REGULAR),
    ("Liberation Serif (Bundled)", LIBERATION_SERIF_REGULAR),
    ("Liberation Mono (Bundled)", LIBERATION_MONO_REGULAR),
    ("Noto Sans (Bundled)", NOTO_SANS_REGULAR),
    ("Noto Serif (Bundled)", NOTO_SERIF_REGULAR),
    ("Noto Sans Mono (Bundled)", NOTO_SANS_MONO_REGULAR),
    ("Noto Sans Arabic (Bundled)", NOTO_SANS_ARABIC_REGULAR),
];

fn bundled_font_names() -> Vec<String> {
    BUNDLED_FONTS
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect()
}

fn bundled_font_data(name: &str) -> Option<&'static [u8]> {
    BUNDLED_FONTS
        .iter()
        .find_map(|(font_name, font_data)| (*font_name == name).then_some(*font_data))
}

#[derive(PartialEq, Clone, Copy)]
pub enum VariableSource {
    Serial,
    Csv,
}

pub struct TextToolState {
    pub is_open: bool,
    pub text: String,
    pub font_size: f32,
    pub font_data: Option<Vec<u8>>,
    pub bundled_fonts: Vec<String>,
    pub system_fonts: Vec<String>,
    pub selected_font: String,
    pub font_source: FontSource,

    // Variable Text
    pub is_variable: bool,
    pub var_prefix: String,
    pub var_suffix: String,
    pub var_start: i32,
    pub var_inc: i32,
    pub var_padding: usize,
    pub var_count: i32,
    pub var_source: VariableSource,
    pub csv_column: usize,
    pub csv_has_header: bool,
    pub csv_delimiter: char,
    pub csv_values: Vec<String>,
    pub csv_path_display: String,
}

#[derive(PartialEq)]
pub enum FontSource {
    Bundled,
    System,
    File,
}

impl Default for TextToolState {
    fn default() -> Self {
        let bundled_fonts = bundled_font_names();

        // Initialize with system fonts
        let mut fonts = Vec::new();
        if let Ok(handles) = SystemSource::new().all_fonts() {
            for handle in handles {
                if let Ok(font) = handle.load() {
                    let family = font.family_name();
                    if !fonts.contains(&family) {
                        fonts.push(family);
                    }
                }
            }
        }
        fonts.sort();

        Self {
            is_open: false,
            text: "All4Laser".to_string(),
            font_size: 40.0,
            font_data: None,
            selected_font: bundled_fonts
                .first()
                .cloned()
                .or_else(|| fonts.first().cloned())
                .unwrap_or_else(|| "Arial".to_string()),
            bundled_fonts,
            system_fonts: fonts,
            font_source: FontSource::Bundled,
            is_variable: false,
            var_prefix: "SN-".to_string(),
            var_suffix: "".to_string(),
            var_start: 1,
            var_inc: 1,
            var_padding: 3,
            var_count: 10,
            var_source: VariableSource::Serial,
            csv_column: 0,
            csv_has_header: true,
            csv_delimiter: ',',
            csv_values: Vec::new(),
            csv_path_display: tr("No CSV loaded").to_string(),
        }
    }
}

fn split_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '"' {
            if in_quotes && chars.peek() == Some(&'"') {
                current.push('"');
                let _ = chars.next();
            } else {
                in_quotes = !in_quotes;
            }
            continue;
        }

        if ch == delimiter && !in_quotes {
            out.push(current.trim().to_string());
            current.clear();
            continue;
        }

        current.push(ch);
    }

    out.push(current.trim().to_string());
    out
}

fn collect_csv_column(
    content: &str,
    column: usize,
    has_header: bool,
    delimiter: char,
) -> Vec<String> {
    let mut values = Vec::new();

    for (idx, raw) in content.lines().enumerate() {
        if has_header && idx == 0 {
            continue;
        }
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let cols = split_csv_line(line, delimiter);
        if let Some(value) = cols.get(column) {
            let clean = value.trim();
            if !clean.is_empty() {
                values.push(clean.to_string());
            }
        }
    }

    values
}

fn load_csv_column(
    path: &std::path::Path,
    column: usize,
    has_header: bool,
    delimiter: char,
) -> Result<Vec<String>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read CSV: {e}"))?;
    let values = collect_csv_column(&content, column, has_header, delimiter);
    if values.is_empty() {
        return Err("No values found in selected CSV column.".to_string());
    }
    Ok(values)
}

pub struct GCodeBuilder {
    pub paths: Vec<Vec<(f32, f32)>>,
    current_path: Vec<(f32, f32)>,
    pub scale: f32,
}

impl GCodeBuilder {
    fn new(scale: f32) -> Self {
        Self {
            paths: Vec::new(),
            current_path: Vec::new(),
            scale,
        }
    }
}

impl OutlineBuilder for GCodeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        if !self.current_path.is_empty() {
            self.paths.push(std::mem::take(&mut self.current_path));
        }
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // Simple linear approximation of quadratic Bezier
        self.current_path.push((x1 * self.scale, -y1 * self.scale));
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // Simple linear approximation of cubic Bezier
        self.current_path.push((x1 * self.scale, -y1 * self.scale));
        self.current_path.push((x2 * self.scale, -y2 * self.scale));
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn close(&mut self) {
        if let Some(&first) = self.current_path.first() {
            self.current_path.push(first);
        }
        if !self.current_path.is_empty() {
            self.paths.push(std::mem::take(&mut self.current_path));
        }
    }
}

pub struct TextAction {
    pub add_shapes: Option<Vec<ShapeParams>>,
}

pub fn show(ui: &mut Ui, state: &mut TextToolState, active_layer_idx: usize) -> TextAction {
    let mut action = TextAction { add_shapes: None };

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("✍ Text Tool")
                    .color(crate::theme::LAVENDER)
                    .strong(),
            );
        });
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.checkbox(&mut state.is_variable, &format!("🔢 {}", tr("Variable Text (Serial Numbers)")));
        });

        if state.is_variable {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", tr("Source")));
                    ui.selectable_value(&mut state.var_source, VariableSource::Serial, tr("Serial"));
                    ui.selectable_value(&mut state.var_source, VariableSource::Csv, tr("CSV Column"));
                });

                if state.var_source == VariableSource::Serial {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", tr("Prefix")));
                        ui.text_edit_singleline(&mut state.var_prefix);
                        ui.label(format!("{}:", tr("Suffix")));
                        ui.text_edit_singleline(&mut state.var_suffix);
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", tr("Start")));
                        ui.add(egui::DragValue::new(&mut state.var_start));
                        ui.label(format!("{}:", tr("Inc")));
                        ui.add(egui::DragValue::new(&mut state.var_inc));
                        ui.label(format!("{}:", tr("Pad")));
                        ui.add(egui::DragValue::new(&mut state.var_padding).range(0..=10));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", tr("Batch Count")));
                        ui.add(egui::DragValue::new(&mut state.var_count).range(1..=100));
                    });
                    
                    // Preview of generated serial numbers
                    ui.separator();
                    ui.label(format!("{}:", tr("Preview")));
                    let preview_count = state.var_count.min(5) as i32;
                    for i in 0..preview_count {
                        let val = state.var_start + (i * state.var_inc);
                        let val_str = format!("{:0>width$}", val, width = state.var_padding);
                        let preview = format!("{}{}{}", state.var_prefix, val_str, state.var_suffix);
                        ui.label(RichText::new(format!("  {}: {}", i + 1, preview)).small().color(theme::SUBTEXT));
                    }
                    if state.var_count > 5 {
                        ui.label(RichText::new(format!("  ... {} {} ...", state.var_count - 5, tr("more"))).small().color(theme::SUBTEXT));
                    }
                } else {
                    let mut reload_csv = false;
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", tr("Column")));
                        if ui
                            .add(egui::DragValue::new(&mut state.csv_column).range(0..=100))
                            .changed()
                        {
                            reload_csv = true;
                        }
                        if ui
                            .checkbox(&mut state.csv_has_header, tr("Header row"))
                            .changed()
                        {
                            reload_csv = true;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", tr("Delimiter")));
                        let mut delimiter_text = state.csv_delimiter.to_string();
                        if ui.text_edit_singleline(&mut delimiter_text).changed() {
                            if let Some(ch) = delimiter_text.chars().next() {
                                state.csv_delimiter = ch;
                                reload_csv = true;
                            }
                        }
                        if ui.button(format!("📁 {}", tr("Load CSV"))).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("CSV", &["csv", "txt"])
                                .pick_file()
                            {
                                state.csv_path_display = path.display().to_string();
                                match load_csv_column(
                                    &path,
                                    state.csv_column,
                                    state.csv_has_header,
                                    state.csv_delimiter,
                                ) {
                                    Ok(values) => state.csv_values = values,
                                    Err(err) => {
                                        state.csv_values.clear();
                                        state.csv_path_display = format!("CSV error: {err}");
                                    }
                                }
                            }
                        }
                    });

                    if reload_csv
                        && !state.csv_path_display.is_empty()
                        && !state.csv_path_display.starts_with("No CSV loaded")
                        && !state.csv_path_display.starts_with("CSV error:")
                    {
                        let csv_path = std::path::Path::new(&state.csv_path_display).to_path_buf();
                        match load_csv_column(
                            &csv_path,
                            state.csv_column,
                            state.csv_has_header,
                            state.csv_delimiter,
                        ) {
                            Ok(values) => state.csv_values = values,
                            Err(err) => {
                                state.csv_values.clear();
                                state.csv_path_display = format!("CSV error: {err}");
                            }
                        }
                    }

                    ui.label(format!("{}: {}", tr("CSV"), state.csv_path_display));
                    ui.label(format!("{}: {}", tr("Loaded rows"), state.csv_values.len()));
                }
            });
        } else {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Text")));
                ui.text_edit_singleline(&mut state.text);
            });
        }

        ui.horizontal(|ui| {
            ui.label(format!("{}:", tr("Size")));
            ui.add(
                egui::DragValue::new(&mut state.font_size)
                    .range(1.0..=300.0)
                    .suffix(" pt"),
            );
        });

        ui.horizontal(|ui| {
            ui.label(format!("{}:", tr("Source")));
            ui.selectable_value(&mut state.font_source, FontSource::Bundled, tr("Bundled"));
            ui.selectable_value(&mut state.font_source, FontSource::System, tr("System"));
            ui.selectable_value(&mut state.font_source, FontSource::File, tr("File"));
        });

        if state.font_source == FontSource::Bundled {
            if !state.bundled_fonts.contains(&state.selected_font)
                && !state.bundled_fonts.is_empty()
            {
                state.selected_font = state.bundled_fonts[0].clone();
            }

            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Font")));
                egui::ComboBox::from_id_salt("bundled_font_combo")
                    .selected_text(&state.selected_font)
                    .width(240.0)
                    .show_ui(ui, |ui| {
                        for font in &state.bundled_fonts {
                            ui.selectable_value(&mut state.selected_font, font.clone(), font);
                        }
                    });
            });
            ui.label(
                RichText::new(
                    tr("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use)."),
                )
                .small(),
            );
        } else if state.font_source == FontSource::System {
            if !state.system_fonts.contains(&state.selected_font) && !state.system_fonts.is_empty()
            {
                state.selected_font = state.system_fonts[0].clone();
            }

            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Font")));
                egui::ComboBox::from_id_salt("font_combo")
                    .selected_text(&state.selected_font)
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        for font in &state.system_fonts {
                            ui.selectable_value(&mut state.selected_font, font.clone(), font);
                        }
                    });
            });
            if state.system_fonts.is_empty() {
                ui.label(
                    RichText::new(tr("No system fonts detected. Use Bundled or File source.")).small(),
                );
            }
        } else {
            ui.horizontal(|ui| {
                if ui.button(format!("📁 {}", tr("Load Font File"))).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("TrueType Font", &["ttf", "otf"])
                        .pick_file()
                    {
                        if let Ok(data) = std::fs::read(&path) {
                            state.font_data = Some(data);
                            state.selected_font = path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                        }
                    }
                }
                ui.label(format!("{}: {}", tr("File"), state.selected_font));
            });
        }
        if ui.button(format!("➕ {}", tr("Add Text to Drawing"))).clicked() {
            let mut resolved_font_data: Option<Vec<u8>> = match state.font_source {
                FontSource::Bundled => {
                    bundled_font_data(&state.selected_font).map(|data| data.to_vec())
                }
                FontSource::System => {
                    let mut loaded = None;
                    if let Ok(handle) = SystemSource::new().select_best_match(
                        &[FamilyName::Title(state.selected_font.clone())],
                        &Properties::new(),
                    ) {
                        if let Ok(font) = handle.load() {
                            loaded = Some(font.copy_font_data().unwrap_or_default().to_vec());
                        }
                    }
                    if loaded.is_none() {
                        if let Ok(handle) =
                            SystemSource::new().select_by_postscript_name(&state.selected_font)
                        {
                            if let Ok(font) = handle.load() {
                                loaded = Some(font.copy_font_data().unwrap_or_default().to_vec());
                            }
                        }
                    }
                    loaded
                }
                FontSource::File => state.font_data.clone(),
            };

            if let Some(data) = resolved_font_data.take() {
                if let Some(font) = Font::try_from_vec(data) {
                    let scale = Scale::uniform(state.font_size);
                    let v_metrics = font.v_metrics(scale);

                    let texts_to_gen = if state.is_variable {
                        if state.var_source == VariableSource::Serial {
                            let mut v = Vec::new();
                            for i in 0..state.var_count {
                                let val = state.var_start + (i * state.var_inc);
                                let val_str =
                                    format!("{:0>width$}", val, width = state.var_padding);
                                v.push(format!(
                                    "{}{}{}",
                                    state.var_prefix, val_str, state.var_suffix
                                ));
                            }
                            v
                        } else {
                            state.csv_values.clone()
                        }
                    } else {
                        vec![state.text.clone()]
                    };

                    let mut created_shapes = Vec::new();
                    let mut current_y_offset = 0.0;
                    for text_str in texts_to_gen {
                        let mut final_paths = Vec::new();
                        for glyph in font.layout(&text_str, scale, point(0.0, v_metrics.ascent)) {
                            let pos = glyph.position();
                            let mut g_builder = GCodeBuilder::new(1.0);
                            glyph.unpositioned().build_outline(&mut g_builder);
                            for mut path in g_builder.paths {
                                for p in &mut path {
                                    p.0 += pos.x;
                                    p.1 -= pos.y - v_metrics.ascent + current_y_offset;
                                }
                                if path.len() >= 2 {
                                    final_paths.push(path);
                                }
                            }
                        }

                        for path in final_paths {
                            created_shapes.push(ShapeParams {
                                shape: ShapeKind::Path(PathData::from_points(path)),
                                layer_idx: active_layer_idx,
                                font_size_mm: state.font_size,
                                ..ShapeParams::default()
                            });
                        }

                        current_y_offset += state.font_size * 1.5;
                    }

                    if !created_shapes.is_empty() {
                        action.add_shapes = Some(created_shapes);
                    }
                }
            }
        }
    });

    action
}

#[cfg(test)]
mod tests {
    use super::{collect_csv_column, split_csv_line};

    #[test]
    fn split_csv_line_supports_quoted_delimiters() {
        let cols = split_csv_line("A,\"B,C\",D", ',');
        assert_eq!(cols, vec!["A", "B,C", "D"]);
    }

    #[test]
    fn collect_csv_column_skips_header_and_empties() {
        let data = "id,name\n1,Alpha\n2,\n3,Gamma\n";
        let values = collect_csv_column(data, 1, true, ',');
        assert_eq!(values, vec!["Alpha", "Gamma"]);
    }
}
