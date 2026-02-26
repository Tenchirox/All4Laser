/// Catppuccin Mocha (Dark) & Latte (Light) color palettes
use egui::{Color32, Context};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UiTheme {
    Modern,    // Catppuccin
    Industrial, // LightBurn-style
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UiLayout {
    Modern, // sidebar left
    Classic, // LightBurn style (sidebar left/right, console right)
}

pub struct AppTheme {
    pub ui_theme: UiTheme,
    pub is_light: bool,
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

// Light (Latte)
pub const LIGHT_BASE: Color32 = Color32::from_rgb(239, 241, 245);
pub const LIGHT_MANTLE: Color32 = Color32::from_rgb(230, 233, 239);
pub const LIGHT_CRUST: Color32 = Color32::from_rgb(220, 224, 232);
pub const LIGHT_SURFACE0: Color32 = Color32::from_rgb(204, 208, 218);
pub const LIGHT_SURFACE1: Color32 = Color32::from_rgb(188, 192, 204);
pub const LIGHT_SURFACE2: Color32 = Color32::from_rgb(172, 176, 190);
pub const LIGHT_TEXT: Color32 = Color32::from_rgb(76, 79, 105);
pub const LIGHT_SUBTEXT: Color32 = Color32::from_rgb(92, 95, 119);

// Accents (approximate for both, but usually vivid enough)
pub const RED: Color32 = Color32::from_rgb(243, 139, 168);     // #F38BA8
pub const PEACH: Color32 = Color32::from_rgb(250, 179, 135);   // #FAB387
pub const YELLOW: Color32 = Color32::from_rgb(249, 226, 175);  // #F9E2AF
pub const GREEN: Color32 = Color32::from_rgb(166, 227, 161);   // #A6E3A1
pub const BLUE: Color32 = Color32::from_rgb(137, 180, 250);    // #89B4FA
pub const LAVENDER: Color32 = Color32::from_rgb(180, 190, 254);// #B4BEFE
pub const MAUVE: Color32 = Color32::from_rgb(203, 166, 247);   // #CBA6F7
pub const TEAL: Color32 = Color32::from_rgb(148, 226, 213);    // #94E2D5

// Generic exports for backwards compatibility (used by other modules directly)
// We will alias these to the currently active theme in `apply_theme` using context, 
// but since they are consts used directly, we can't easily swap them at runtime without 
// passing the theme state everywhere. 
// For now, let's just make the background/surfaces change correctly.
// The constants exported here are the DARK defaults used in other files.
pub const BASE: Color32 = DARK_BASE;
pub const MANTLE: Color32 = DARK_MANTLE;
pub const CRUST: Color32 = DARK_CRUST;
pub const SURFACE0: Color32 = DARK_SURFACE0;
pub const SURFACE1: Color32 = DARK_SURFACE1;
pub const SURFACE2: Color32 = DARK_SURFACE2;
pub const OVERLAY0: Color32 = Color32::from_rgb(108, 112, 134); // #6C7086
pub const TEXT: Color32 = DARK_TEXT;
pub const SUBTEXT: Color32 = DARK_SUBTEXT;

pub fn apply_theme(ctx: &Context, state: &AppTheme) {
    let mut style = (*ctx.style()).clone();
    
    // Industrial Palette (LightBurn-ish)
    let iron = Color32::from_rgb(45, 45, 48);      // Background
    let steel = Color32::from_rgb(60, 60, 65);     // Surface
    let coal = Color32::from_rgb(30, 30, 33);      // Crust
    let cobalt = Color32::from_rgb(0, 122, 204);   // Accent Blue
    let mercury = Color32::from_rgb(220, 220, 220);// Text

    let (base, mantle, crust, surface0, surface1, surface2, text, accent) = match state.ui_theme {
        UiTheme::Modern => {
            if state.is_light {
                (LIGHT_BASE, LIGHT_MANTLE, LIGHT_CRUST, LIGHT_SURFACE0, LIGHT_SURFACE1, LIGHT_SURFACE2, LIGHT_TEXT, BLUE)
            } else {
                (DARK_BASE, DARK_MANTLE, DARK_CRUST, DARK_SURFACE0, DARK_SURFACE1, DARK_SURFACE2, DARK_TEXT, BLUE)
            }
        }
        UiTheme::Industrial => {
            // Primarily a dark theme by nature, but we can offer a light variant if needed.
            // For now, let's treat "Industrial" as the professional dark mode.
            if state.is_light {
                (Color32::from_rgb(240, 240, 242), Color32::from_rgb(225, 225, 230), Color32::from_rgb(210, 210, 215), 
                 Color32::from_rgb(200, 200, 205), Color32::from_rgb(180, 180, 185), Color32::from_rgb(160, 160, 165), 
                 Color32::from_rgb(40, 40, 45), Color32::from_rgb(0, 90, 170))
            } else {
                (iron, steel, coal, steel, Color32::from_rgb(70, 70, 75), Color32::from_rgb(85, 85, 90), mercury, cobalt)
            }
        }
    };

    let border_color = if state.is_light {
        Color32::from_rgb(180, 185, 200)
    } else {
        match state.ui_theme {
            UiTheme::Modern => Color32::from_rgb(60, 63, 82),
            UiTheme::Industrial => Color32::from_rgb(80, 80, 85),
        }
    };

    style.visuals.dark_mode = !state.is_light;
    style.visuals.panel_fill = base;
    style.visuals.window_fill = base;
    style.visuals.extreme_bg_color = crust;
    style.visuals.faint_bg_color = mantle;
    style.visuals.override_text_color = Some(text);

    // Rounding based on theme
    let rounding = if state.ui_theme == UiTheme::Industrial {
        egui::CornerRadius::same(2) // Sharper
    } else {
        egui::CornerRadius::same(6) // Modern/Round
    };

    // Noninteractive
    style.visuals.widgets.noninteractive.bg_fill = surface0;
    style.visuals.widgets.noninteractive.weak_bg_fill = surface0;
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, border_color);
    style.visuals.widgets.noninteractive.fg_stroke.color = text;
    style.visuals.widgets.noninteractive.corner_radius = rounding;

    // Inactive (default button state)
    style.visuals.widgets.inactive.bg_fill = surface0;
    style.visuals.widgets.inactive.weak_bg_fill = if state.is_light { Color32::from_rgb(225, 228, 238) } else { surface0 };
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, border_color);
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.5, text);
    style.visuals.widgets.inactive.corner_radius = rounding;

    // Hovered
    style.visuals.widgets.hovered.bg_fill = surface1;
    style.visuals.widgets.hovered.weak_bg_fill = surface1;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, accent);
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, text);
    style.visuals.widgets.hovered.corner_radius = rounding;

    // Active (clicked)
    style.visuals.widgets.active.bg_fill = surface2;
    style.visuals.widgets.active.weak_bg_fill = surface2;
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, accent);
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, text);
    style.visuals.widgets.active.corner_radius = rounding;

    // Open (combo boxes, menus)
    style.visuals.widgets.open.bg_fill = surface1;
    style.visuals.widgets.open.weak_bg_fill = surface1;
    style.visuals.widgets.open.bg_stroke = egui::Stroke::new(1.5, accent);
    style.visuals.widgets.open.fg_stroke = egui::Stroke::new(1.5, text);
    style.visuals.widgets.open.corner_radius = rounding;

    style.visuals.selection.bg_fill = accent;
    style.visuals.selection.stroke.color = crust;
    
    if state.ui_theme == UiTheme::Industrial {
        style.spacing.button_padding = egui::vec2(6.0, 3.0); // Tighter
    } else {
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
    }
    
    ctx.set_style(style);
}
