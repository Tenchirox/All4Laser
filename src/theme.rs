#![allow(dead_code)]

/// Catppuccin Mocha (Dark) & Latte (Light) color palettes
use egui::{Color32, Context};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UiTheme {
    Modern,     // Catppuccin
    Industrial, // LightBurn-style
    Pro,        // Clean, High-contrast, Modern Professional
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UiLayout {
    Modern,  // sidebar left
    Classic, // LightBurn style (sidebar left/right, console right)
    Pro,     // High-end workspace: wider panels, robust layout
}

pub struct AppTheme {
    pub ui_theme: UiTheme,
    pub is_light: bool,
}

/// User-importable custom theme (F66)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomTheme {
    pub name: String,
    pub base: [u8; 3],
    pub mantle: [u8; 3],
    pub crust: [u8; 3],
    pub surface0: [u8; 3],
    pub surface1: [u8; 3],
    pub surface2: [u8; 3],
    pub text: [u8; 3],
    pub subtext: [u8; 3],
    pub accent: [u8; 3],
}

impl CustomTheme {
    fn themes_dir() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("themes")
    }

    pub fn save(&self) -> Result<(), String> {
        if self.name.contains('/') || self.name.contains('\\') || self.name.contains("..") {
            return Err("Invalid theme name".into());
        }
        let dir = Self::themes_dir();
        let _ = std::fs::create_dir_all(&dir);
        let filename = self.name.replace(' ', "_").to_lowercase() + ".json";
        let path = dir.join(filename);
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn load(name: &str) -> Result<CustomTheme, String> {
        if name.contains('/') || name.contains('\\') || name.contains("..") {
            return Err("Invalid theme name".into());
        }
        let dir = Self::themes_dir();
        let path = dir.join(format!("{name}.json"));
        let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    pub fn list() -> Vec<String> {
        let dir = Self::themes_dir();
        let mut names = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.path().file_stem() {
                    names.push(name.to_string_lossy().to_string());
                }
            }
        }
        names
    }
}

// Dark (Mocha)
pub const DARK_BASE: Color32 = Color32::from_rgb(30, 30, 46);
pub const DARK_MANTLE: Color32 = Color32::from_rgb(24, 24, 37);
pub const DARK_CRUST: Color32 = Color32::from_rgb(17, 17, 27);
pub const DARK_SURFACE0: Color32 = Color32::from_rgb(49, 50, 68);
pub const DARK_SURFACE1: Color32 = Color32::from_rgb(69, 71, 90);
pub const DARK_SURFACE2: Color32 = Color32::from_rgb(88, 91, 112);
pub const DARK_TEXT: Color32 = Color32::from_rgb(205, 214, 244);
pub const DARK_SUBTEXT: Color32 = Color32::from_rgb(166, 173, 200);
pub const DARK_OVERLAY0: Color32 = Color32::from_rgb(108, 112, 134); // #6C7086
pub const DARK_OVERLAY2: Color32 = Color32::from_rgb(147, 153, 178); // #9399B2

// Light (Latte)
pub const LIGHT_BASE: Color32 = Color32::from_rgb(248, 249, 252);
pub const LIGHT_MANTLE: Color32 = Color32::from_rgb(241, 243, 247);
pub const LIGHT_CRUST: Color32 = Color32::from_rgb(232, 235, 241);
pub const LIGHT_SURFACE0: Color32 = Color32::from_rgb(223, 227, 236);
pub const LIGHT_SURFACE1: Color32 = Color32::from_rgb(207, 213, 226);
pub const LIGHT_SURFACE2: Color32 = Color32::from_rgb(190, 198, 214);
pub const LIGHT_TEXT: Color32 = Color32::from_rgb(43, 49, 65);
pub const LIGHT_SUBTEXT: Color32 = Color32::from_rgb(80, 88, 109);

// Accents (approximate for both, but usually vivid enough)
pub const RED: Color32 = Color32::from_rgb(243, 139, 168); // #F38BA8
pub const PEACH: Color32 = Color32::from_rgb(250, 179, 135); // #FAB387
pub const YELLOW: Color32 = Color32::from_rgb(249, 226, 175); // #F9E2AF
pub const GREEN: Color32 = Color32::from_rgb(166, 227, 161); // #A6E3A1
pub const BLUE: Color32 = Color32::from_rgb(82, 134, 214);
pub const LAVENDER: Color32 = Color32::from_rgb(180, 190, 254); // #B4BEFE
pub const MAUVE: Color32 = Color32::from_rgb(203, 166, 247); // #CBA6F7
pub const TEAL: Color32 = Color32::from_rgb(148, 226, 213); // #94E2D5

// Generic exports for backwards compatibility (used by other modules directly)
// We will alias these to the currently active theme in `apply_theme` using context,
// but since they are consts used directly, we can't easily swap them at runtime without
// passing the theme state everywhere.
// For now, let's just make the background/surfaces change correctly.
// The constants exported here are the DARK defaults used in other files.
pub const BASE: Color32 = DARK_BASE;
pub const MANTLE: Color32 = DARK_MANTLE;
pub const CRUST: Color32 = DARK_CRUST;
// Legacy constants (always dark) - use get_surface0(), get_surface1(), get_surface2() for theme-aware colors
pub const SURFACE0: Color32 = DARK_SURFACE0;
pub const SURFACE1: Color32 = DARK_SURFACE1;
pub const SURFACE2: Color32 = DARK_SURFACE2;

/// Get theme-aware surface colors based on current app theme
pub fn get_surface0(is_light: bool) -> Color32 {
    if is_light { LIGHT_SURFACE0 } else { DARK_SURFACE0 }
}

pub fn get_surface1(is_light: bool) -> Color32 {
    if is_light { LIGHT_SURFACE1 } else { DARK_SURFACE1 }
}

pub fn get_surface2(is_light: bool) -> Color32 {
    if is_light { LIGHT_SURFACE2 } else { DARK_SURFACE2 }
}

pub fn get_base(is_light: bool) -> Color32 {
    if is_light { LIGHT_BASE } else { DARK_BASE }
}

pub fn get_mantle(is_light: bool) -> Color32 {
    if is_light { LIGHT_MANTLE } else { DARK_MANTLE }
}

pub fn get_crust(is_light: bool) -> Color32 {
    if is_light { LIGHT_CRUST } else { DARK_CRUST }
}

pub fn get_text(is_light: bool) -> Color32 {
    if is_light { LIGHT_TEXT } else { DARK_TEXT }
}

pub fn get_subtext(is_light: bool) -> Color32 {
    if is_light { LIGHT_SUBTEXT } else { DARK_SUBTEXT }
}
pub const OVERLAY0: Color32 = DARK_OVERLAY0;
pub const OVERLAY2: Color32 = DARK_OVERLAY2;
pub const TEXT: Color32 = DARK_TEXT;
pub const SUBTEXT: Color32 = DARK_SUBTEXT;

/// Determine if a color is light (for choosing contrasting text color)
pub fn is_light(c: Color32) -> bool {
    let brightness = (c.r() as f32 * 299.0 + c.g() as f32 * 587.0 + c.b() as f32 * 114.0) / 1000.0;
    brightness > 128.0
}

pub fn apply_theme(ctx: &Context, state: &AppTheme) {
    let mut style = (*ctx.style()).clone();

    // Industrial Palette (LightBurn-style - plus fidèle à l'original)
    let iron = Color32::from_rgb(48, 49, 52); // Background principal
    let steel = Color32::from_rgb(58, 59, 63); // Surface panels
    let coal = Color32::from_rgb(39, 40, 43); // Fond sombre
    let cobalt = Color32::from_rgb(0, 122, 204); // Accent Blue LightBurn
    let mercury = Color32::from_rgb(230, 231, 234); // Texte principal
    let light_gray = Color32::from_rgb(186, 188, 194); // Texte secondaire
    let dark_steel = Color32::from_rgb(71, 72, 77); // Surface active

    let (base, mantle, crust, surface0, surface1, surface2, text, accent) = match state.ui_theme {
        UiTheme::Modern => {
            if state.is_light {
                (
                    LIGHT_BASE,
                    LIGHT_MANTLE,
                    LIGHT_CRUST,
                    LIGHT_SURFACE0,
                    LIGHT_SURFACE1,
                    LIGHT_SURFACE2,
                    LIGHT_TEXT,
                    BLUE,
                )
            } else {
                (
                    DARK_BASE,
                    DARK_MANTLE,
                    DARK_CRUST,
                    DARK_SURFACE0,
                    DARK_SURFACE1,
                    DARK_SURFACE2,
                    DARK_TEXT,
                    BLUE,
                )
            }
        }
        UiTheme::Pro => {
            if state.is_light {
                (
                    Color32::from_rgb(250, 250, 250), // Base
                    Color32::from_rgb(240, 240, 240), // Mantle
                    Color32::from_rgb(230, 230, 230), // Crust
                    Color32::from_rgb(255, 255, 255), // Surface0
                    Color32::from_rgb(245, 245, 245), // Surface1
                    Color32::from_rgb(220, 220, 220), // Surface2
                    Color32::from_rgb(17, 24, 39),    // Text (very dark gray)
                    Color32::from_rgb(14, 165, 233),  // Accent (Sky Blue)
                )
            } else {
                (
                    Color32::from_rgb(15, 23, 42),    // Base (Slate 900)
                    Color32::from_rgb(2, 6, 23),      // Mantle (Slate 950)
                    Color32::from_rgb(0, 0, 0),       // Crust (Black)
                    Color32::from_rgb(30, 41, 59),    // Surface0 (Slate 800)
                    Color32::from_rgb(51, 65, 85),    // Surface1 (Slate 700)
                    Color32::from_rgb(71, 85, 105),   // Surface2 (Slate 600)
                    Color32::from_rgb(248, 250, 252), // Text (Slate 50)
                    Color32::from_rgb(56, 189, 248),  // Accent (Sky 400)
                )
            }
        }
        UiTheme::Industrial => {
            // LightBurn utilise un thème sombre professionnel avec des accents bleus
            if state.is_light {
                // Version claire inspirée de LightBurn
                (
                    Color32::from_rgb(242, 243, 245),
                    Color32::from_rgb(232, 233, 236),
                    Color32::from_rgb(221, 223, 227),
                    Color32::from_rgb(248, 249, 251),
                    Color32::from_rgb(236, 238, 242),
                    Color32::from_rgb(221, 224, 230),
                    Color32::from_rgb(38, 40, 44),
                    Color32::from_rgb(0, 114, 193),
                )
            } else {
                // Version sombre - couleurs LightBurn authentiques
                (
                    iron,
                    steel,
                    coal,
                    steel,
                    dark_steel,
                    Color32::from_rgb(86, 88, 94),
                    mercury,
                    cobalt,
                )
            }
        }
    };

    let border_color = match state.ui_theme {
        UiTheme::Industrial => {
            if state.is_light {
                Color32::from_rgb(176, 181, 190)
            } else {
                Color32::from_rgb(93, 95, 101)
            }
        }
        UiTheme::Modern => {
            if state.is_light {
                Color32::from_rgb(163, 172, 191)
            } else {
                Color32::from_rgb(60, 63, 82)
            }
        }
        UiTheme::Pro => {
            if state.is_light {
                Color32::from_rgb(226, 232, 240) // Slate 200
            } else {
                Color32::from_rgb(51, 65, 85) // Slate 700
            }
        }
    };

    style.visuals.dark_mode = !state.is_light;
    style.visuals.panel_fill = base;
    style.visuals.window_fill = base;
    style.visuals.extreme_bg_color = crust;
    style.visuals.faint_bg_color = mantle;
    style.visuals.override_text_color = Some(text);
    style.visuals.window_stroke = egui::Stroke::new(1.0, border_color);

    // Rounding based on theme
    let rounding = match state.ui_theme {
        UiTheme::Industrial => egui::CornerRadius::same(1), // Quasi plat
        UiTheme::Pro => egui::CornerRadius::same(4),        // Slightly rounded, crisp
        UiTheme::Modern => egui::CornerRadius::same(6),     // Modern/Round
    };

    // Noninteractive
    style.visuals.widgets.noninteractive.bg_fill = surface0;
    style.visuals.widgets.noninteractive.weak_bg_fill = surface0;
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, border_color);
    style.visuals.widgets.noninteractive.fg_stroke.color = text;
    style.visuals.widgets.noninteractive.corner_radius = rounding;

    // Inactive (default button state)
    style.visuals.widgets.inactive.bg_fill = surface0;
    style.visuals.widgets.inactive.weak_bg_fill = if state.ui_theme == UiTheme::Industrial {
        mantle
    } else if state.is_light {
        if state.ui_theme == UiTheme::Pro {
            Color32::from_rgb(241, 245, 249)
        } else {
            Color32::from_rgb(225, 228, 238)
        }
    } else {
        surface0
    };
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, border_color);
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(
        if state.ui_theme == UiTheme::Industrial {
            1.2
        } else {
            1.5
        },
        text,
    );
    style.visuals.widgets.inactive.corner_radius = rounding;

    // Hovered
    style.visuals.widgets.hovered.bg_fill = surface1;
    style.visuals.widgets.hovered.weak_bg_fill = surface1;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(
        if state.ui_theme == UiTheme::Industrial {
            1.2
        } else {
            1.5
        },
        accent,
    );
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, text);
    style.visuals.widgets.hovered.corner_radius = rounding;

    // Active (clicked)
    style.visuals.widgets.active.bg_fill = surface2;
    style.visuals.widgets.active.weak_bg_fill = surface2;
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(
        if state.ui_theme == UiTheme::Industrial {
            1.6
        } else {
            2.0
        },
        accent,
    );
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, text);
    style.visuals.widgets.active.corner_radius = rounding;

    // Open (combo boxes, menus)
    style.visuals.widgets.open.bg_fill = surface1;
    style.visuals.widgets.open.weak_bg_fill = surface1;
    style.visuals.widgets.open.bg_stroke = egui::Stroke::new(
        if state.ui_theme == UiTheme::Industrial {
            1.2
        } else {
            1.5
        },
        accent,
    );
    style.visuals.widgets.open.fg_stroke = egui::Stroke::new(1.5, text);
    style.visuals.widgets.open.corner_radius = rounding;

    style.visuals.selection.bg_fill = accent;
    style.visuals.selection.stroke.color = if state.ui_theme == UiTheme::Industrial {
        if state.is_light {
            Color32::from_rgb(250, 250, 252)
        } else {
            light_gray
        }
    } else {
        crust
    };

    match state.ui_theme {
        UiTheme::Industrial => {
            style.spacing.button_padding = egui::vec2(5.5, 3.0);
            style.spacing.item_spacing = egui::vec2(3.5, 3.5);
            style.spacing.indent = 10.0;
            style.visuals.window_stroke = egui::Stroke::new(1.0, Color32::from_rgb(83, 85, 92));
        }
        UiTheme::Pro => {
            style.spacing.button_padding = egui::vec2(10.0, 5.0);
            style.spacing.item_spacing = egui::vec2(6.0, 6.0);
            style.spacing.indent = 12.0;
        }
        UiTheme::Modern => {
            style.spacing.button_padding = egui::vec2(12.0, 6.0);
            style.spacing.item_spacing = egui::vec2(8.0, 7.0);
        }
    }

    ctx.set_style(style);
}
