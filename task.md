# All4Laser — UX Improvement Audit

> Comprehensive list of all UX improvements identified across the UI codebase.
> Priority: 🔴 High | 🟡 Medium | 🟢 Low

---

## 1. Toolbar (`ui/toolbar.rs`)

### 🔴 1.1 — Toolbar overflow on small screens
The toolbar uses `horizontal_wrapped` with a 900px breakpoint for compact mode. On very narrow windows (< 700px), buttons can still wrap awkwardly. Consider a hamburger/overflow menu beyond a threshold, or make button groups collapsible.

### � 1.2 — No visual indicator for active state on Run/Frame/Hold ✅
When the machine is running, only the Stop button appears. There's no pulsing animation, colored badge, or status LED to remind the user a job is active. Add a subtle animated dot or colored bar.

### � 1.3 — Duplicate "Resume" and "Run" icons (both use ▶) ✅
`Hold` uses ⏸ and `Resume` uses ▶ — same icon as `Run`. This is confusing. Use ▶️ with a different tint or a distinct icon (e.g. ⏯) for Resume.

### � 1.4 — Recent Files dropdown arrow (`▾`) has no label ✅
The small `▾` button next to Open has no text or tooltip until hovered. New users won't discover recent files. Consider merging it into the Open button as a split-button, or adding a "Recent" label.

### 🟢 1.5 — No keyboard shortcut hints in toolbar tooltips ✅
Toolbar buttons have hover text like "Open" but don't mention `Ctrl+O`. Adding shortcut hints to tooltips improves discoverability.

### 🟢 1.6 — View > Theme/Layout menu items don't show active selection ✅
When opening View, themes and layouts are listed as plain labels. The currently active theme/layout is not visually marked (e.g., checkmark or bold). Use `selectable_label(is_current, ...)` instead of `selectable_label(false, ...)`.

### 🟢 1.7 — Missing "New Project" action ✅
There's no "New" / "Clear workspace" option in File or toolbar. Users must close and reopen the app to start fresh.

---

## 2. Cut List / Layers Panel (`ui/cut_list.rs`)

### 🔴 2.1 — No drag-and-drop reordering of layers ✅
Layers are displayed in index order. Users cannot drag to reorder output priority. LightBurn allows drag reordering. Consider implementing drag-to-reorder or at least ↑/↓ buttons per layer.

### 🟡 2.2 — No inline speed/power editing ✅
Speed and power are displayed as a read-only label (`1000/500`). Users must open Cut Settings to change them. Adding inline DragValue fields would speed up workflow significantly.

### 🟡 2.3 — No layer renaming inline ✅
The layer name ("C00", "C01") is shown as a static label. Double-clicking should allow inline renaming (like LightBurn's layer name field).

### 🟡 2.4 — No "Select All Objects on Layer" action ✅
There's no way to quickly select all shapes assigned to a layer from the cut list. Add a right-click context menu or a button.

### 🟢 2.5 — No visual pass count indicator ✅
The number of passes is not shown in the cut list summary. Users have to open settings to see it. Show a small "×2" badge when passes > 1.

### 🟢 2.6 — Layer count badge is too subtle ✅
The count in the top-right corner is small and uses SUBTEXT color. Consider a more visible badge or chip.

---

## 3. Cut Settings Dialog (`ui/cut_settings.rs`)

### � 3.1 — Dialog is not resizable ✅
`resizable(false)` makes the window fixed. With many collapsible sections open, the content may overflow. Consider making it resizable or adding a ScrollArea.

### � 3.2 — No "Apply" without closing ✅
There's only "OK" (apply + close) and "Cancel". Adding an "Apply" button that saves changes without closing would allow users to iterate faster (e.g., tweaking power while observing preview).

### 🟡 3.3 — No undo/redo within Cut Settings
If a user changes multiple parameters and wants to revert to the state when the dialog opened, they must click Cancel. There's no per-field undo. The Parameter Snapshot (F91) partially addresses this but is manual.

### 🟢 3.4 — Collapsing headers don't remember open/close state
Each time the dialog opens, sections revert to their defaults (Fill open, Advanced closed, etc.). Persisting the open/close state would improve UX.

### 🟢 3.5 — Kerf Calibration section could use a visual diagram
The kerf calibration process ("cut a square, measure it") would benefit from a small inline illustration showing what "nominal" vs "measured" means.

### 🟢 3.6 — No material preset quick-apply inside Cut Settings
Users must navigate to a separate materials panel. Having a "Load from preset" dropdown within Cut Settings would be convenient.

---

## 4. Preview Panel (`ui/preview_panel.rs`)

### 🟡 4.1 — Simulation slider has no time/percentage label ✅
The simulation progress slider uses `show_value(false)`. Users don't know if they're at 30% or 80%. Show a percentage or estimated time label.

### 🟡 4.2 — Thermal risk threshold slider is unlabeled ✅
The risk threshold slider and cell size DragValue have no labels — only a "⚠N" indicator. New users won't understand what these controls do. Add brief labels or tooltips.

### 🟢 4.3 — Checkbox labels disappear entirely in compact mode ✅
Compact mode replaces "Rapids" with "R", "Fill" with "F", etc. These single letters are cryptic. Consider using icons or very short abbreviations with tooltips.

### 🟢 4.4 — No "play/pause" button for simulation ✅
The simulation is controlled only via a checkbox + slider. A play/pause button that animates the slider automatically would be a significant UX improvement for previewing jobs.

### 🟢 4.5 — Zoom buttons could use scroll wheel integration hint ✅
The zoom buttons (🔍+/🔍−) are small. A tooltip mentioning "or use mouse scroll wheel" would help discoverability.

---

## 5. GCode Editor (`ui/gcode_editor.rs`)

### 🟡 5.1 — No line numbers shown ✅
The editor displays syntax-highlighted GCode but has no line numbers. Line numbers are essential for debugging GCode errors reported by the controller (e.g., "error on line 42").

### 🟡 5.2 — Fixed max height of 420px ✅
The `ScrollArea` has `max_height(420.0)` and the TextEdit has a fixed 400px height. On large monitors this wastes space. Use available height dynamically.

### 🟡 5.3 — No search/find functionality ✅
Users editing large GCode files have no way to search for a specific command (e.g., `M3`, `G28`). A simple Ctrl+F find bar would be valuable.

### 🟢 5.4 — No "unsaved changes" confirmation on close ✅
Closing the editor when `dirty == true` doesn't warn the user. Changes are silently discarded.

### 🟢 5.5 — Window title duplicates internal label ✅
The window title is "📝 GCode Editor" and the first line inside is also "📝 GCode Editor". Remove the internal duplicate label.

---

## 6. Job Queue (`ui/job_queue.rs`)

### � 6.1 — No estimated time per job ✅
Each job shows line count but no time estimate. Even a rough estimate (total distance / average feed rate) would help with planning.

### 🟡 6.2 — No confirmation on job removal ✅
Clicking `✕` on a queued job immediately removes it without confirmation.

### 🟡 6.3 — History entries are not selectable/reloadable ✅
History shows completed/failed jobs but there's no way to re-open or re-queue a completed job from history (only "Retry Last Failed" exists).

### 🟢 6.4 — Scroll areas both use same `max_height(180.0)` ✅
Both the pending queue and history share a fixed 180px height. On large screens this wastes vertical space. Use proportional allocation.

### 🟢 6.5 — Hard-coded strings not internationalized ✅
Strings like "Completed", "Failed:", "Aborted", "Auto-run next queued job" are not wrapped in `tr()`. They won't translate.

### 🟢 6.6 — Batch Import doesn't show feedback ✅
After batch importing files, there's no toast/notification. The empty `if !ids.is_empty() {}` block should provide user feedback.

---

## 7. Nesting (`ui/nesting.rs`)

### 🟡 7.1 — No preview of nesting result before committing
Clicking "Apply Nesting" immediately moves shapes. A preview overlay showing where shapes would be placed (before confirming) would prevent mistakes.

### 🟡 7.2 — No undo integration
Nesting modifies shapes in-place. If the result is unsatisfactory, the user must manually undo (if undo is available). The nesting action should push an undo snapshot.

### 🟢 7.3 — Labels not internationalized ✅
"Spacing (mm):", "Margins (mm):", etc. are hard-coded English strings without `tr()`.

### 🟢 7.4 — No result feedback after nesting
After `apply_nesting` returns a `NestingResult`, the UI doesn't display how many shapes were placed vs. skipped. Show a brief summary.

---

## 8. Camera (`ui/camera.rs`)

### 🔴 8.1 — Overwhelming UI density ✅
The camera panel crams many controls into a single `group`. Calibration wizard steps, marker detection, live stream controls, opacity, and device selection are all visible at once. Group related controls under collapsible headers.

### 🟡 8.2 — Calibration steps are text-only with no visual progress
The 3-step calibration wizard shows text hints but no visual step indicator (1/3, 2/3, 3/3). A progress bar or step dots would clarify the flow.

### 🟡 8.3 — Nudge buttons have no configurable step size
The calibration nudge buttons (←→↓↑) move by exactly 1.0mm. For fine alignment, users need 0.1mm steps. Add a step-size selector or allow Shift+click for fine adjustment.

### 🟡 8.4 — Camera device warning text is very long
The Windows camera privacy warning is a full sentence rendered inline in small text. Consider a warning icon + tooltip instead of inline text.

### 🟢 8.5 — Labels not internationalized ✅
"Device:", "Opacity:", "Nudge:", "Calibration", "Reset Calibration" etc. are not wrapped in `tr()`.

---

## 9. Power/Speed Test Matrix (`ui/power_speed_test.rs`)

### 🟡 9.1 — No visual preview of the test grid
Users set up rows/cols/sizes but cannot see a visual preview of the resulting grid before generating. A miniature schematic would help.

### 🟡 9.2 — No label engraving option
The generated grid of squares has GCode comments identifying each cell's parameters, but those comments aren't visible on the material. An option to engrave small text labels (speed/power values) next to each cell would be extremely useful.

### 🟢 9.3 — Labels not internationalized ✅
All labels ("Rows (speed steps):", "Cols (power steps):", etc.) are hard-coded English.

### 🟢 9.4 — No "Presets" for common test patterns
Users always start from scratch. Offering presets (e.g., "Quick 3×3 Test", "Detailed 10×5 Matrix") would save time.

---

## 10. Tiling (`ui/tiling.rs`)

### 🟡 10.1 — No visual preview of tiled result
Users set cols/rows/spacing but cannot see a preview of the tile layout. A small schematic showing the grid arrangement would help.

### 🟢 10.2 — Labels not internationalized ✅
"Columns:", "Rows:", "Spacing X (mm):", etc. are hard-coded English.

### 🟢 10.3 — Spacing defaults (110mm) may be confusing
Default spacing of 110mm assumes large workspace. Better to auto-calculate based on job bounding box + a small gap.

---

## 11. Alignment Tools (`ui/alignment.rs`)

### 🟡 11.1 — No "Align to Page" explicit toggle
The code uses `selection.len() == 1` as an implicit "align to page" condition. LightBurn has an explicit toggle. This behavior is not discoverable.

### 🟡 11.2 — No "Distribute" operations implemented
`AlignCmd::DistributeHorizontal` and `DistributeVertical` are defined in the enum but the match arm uses `_ => {}` — they're no-ops. Implement even spacing distribution.

### 🟢 11.3 — Unicode alignment symbols may be unclear
Symbols like ⇤ ⇹ ⇥ ⤒ ⇕ ⤓ are technically correct but may not render well on all systems. Consider fallback text or SVG icons.

### 🟢 11.4 — No "Center on Workspace" one-click action
Users must manually select an object, then use center-H + center-V separately. A single "Center on Page" button would be faster.

---

## 12. Drawing Tools (`ui/drawing.rs`)

### 🟡 12.1 — No snapping/grid system
There's no snap-to-grid, snap-to-object, or snap-to-guideline system. Precision placement requires manual coordinate entry.

### 🟡 12.2 — No shape transform panel (position/size/rotation)
Users can draw shapes but there's no visible panel showing X, Y, Width, Height, Rotation with editable fields for the selected shape.

### � 12.3 — Bézier flatten uses fixed 32-step resolution
`PathData::flatten_segments` uses a hard-coded 32-step subdivision for all Bézier curves regardless of curve length. Short curves waste computation; long curves may look faceted. Adaptive subdivision would improve quality and performance.

### 🟢 12.4 — No ruler/measurement tool
There's no way to measure distances on the canvas without exporting. A ruler/measure tool would be useful.

---

## 13. Object Generators (`ui/generators.rs`)

### 🟡 13.1 — QR Code module size is hard-coded to 1mm
The QR code generator uses `let size = 1.0_f32` without exposing it as a parameter. Users should be able to set the overall QR code size or module size.

### 🟡 13.2 — Box Maker has no preview
The finger-joint box generator produces 6 faces but users can't see a preview or 3D rendering before generating.

### 🟢 13.3 — Living Hinge parameters need tooltips ✅
The sliders for cut length, gap, and distance lack explanatory tooltips. New users won't understand these parameters.

### 🟢 13.4 — No "Gear Generator" or "Coaster" generator
Common laser cutter generators (gears, coasters, keychains) are missing. These are popular in other software.

---

## 14. Offset Path (`ui/offset.rs`)

### 🟡 14.1 — No inward/outward direction toggle
The distance field accepts positive values only (implicitly outward). Add an explicit "Inward/Outward" toggle for clarity, or allow negative values with clear labeling.

### 🟢 14.2 — No preview of offset result
The offset is applied immediately. A ghost preview before committing would prevent mistakes.

### 🟢 14.3 — Labels not internationalized ✅
"Distance:", "Style:", "Create Offset", "Close" are hard-coded English.

---

## 14b. Boolean Operations (`ui/boolean_ops.rs`)

### 🟡 14b.1 — Subtract order is ambiguous
Subtract shows "A - B" but there's no indication of which shape is A vs B. The first selected shape is implicitly A, but this is not communicated to the user.

### 🟡 14b.2 — No minimum selection validation message
If fewer than 2 shapes are selected, `apply_boolean` silently returns without doing anything. A warning label in the UI would help.

### 🟢 14b.3 — Labels not internationalized ✅
"Union (Combine)", "Subtract (A - B)", etc. are hard-coded English.

### 🟢 14b.4 — Holes lose semantic meaning
The comment in the code notes: "our drawing state doesn't have a 'hole' concept explicitly". Interior rings from boolean operations become separate paths, which may produce incorrect fill behavior.

---

## 15. Circular Array (`ui/circular_array.rs`)

### 🟡 15.1 — Center defaults to (100, 100) — arbitrary
The default center point is hard-coded. A better UX would auto-calculate the center from the current selection's bounding box or workspace center.

### 🟢 15.2 — No visual preview of array positions
A small schematic showing dots at the calculated positions would help users verify the array before applying.

---

## 16. Grid Array (`ui/grid_array.rs`)

### 🟡 16.1 — Spacing is absolute, not "gap between objects"
`dx`/`dy` represent center-to-center spacing, not gap. This is confusing when objects have different sizes. Show total footprint or offer a "gap" mode.

### 🟢 16.2 — No visual preview
Same as circular array — a simple schematic would help.

---

## 17. Shortcuts Panel (`ui/shortcuts.rs`)

### 🟡 17.1 — Shortcut descriptions are not internationalized ✅
The SHORTCUTS const uses hard-coded English strings like "Run / Pause program", "Toggle framing mode", etc. They should use `tr()`.

### 🟢 17.2 — No search/filter ✅
With 17+ shortcuts, a filter field would help users find specific actions.

### 🟢 17.3 — No way to customize shortcuts
Shortcuts are hard-coded. Power users may want to remap keys.

---

## 18. Text Tool (`ui/text.rs`)

### 🟡 18.1 — No live text preview on canvas
Text is configured in a panel but there's no real-time preview showing how it will look at the specified size and font before clicking "Add Text".

### � 18.2 — Bézier approximation is linear (low quality)
The `GCodeBuilder` `quad_to` and `curve_to` methods use simple linear interpolation (only control + end points) instead of proper subdivision. This produces jagged curves on text glyphs, especially at larger sizes.

### 🟡 18.3 — Variable Text preview shows no sample output ✅
When configuring serial numbers (prefix/suffix/start/inc/padding), there's no preview of what the generated strings will look like (e.g., "SN-001", "SN-002", …).

### 🟢 18.4 — Font combo has no visual font preview
Font selection via `ComboBox` shows font names as plain text. A visual preview showing each font's sample rendering would help users choose.

### 🟢 18.5 — Labels not internationalized ✅
"Text:", "Size:", "Source:", "Font:", "Prefix:", "Suffix:", "Start:", "Inc:", "Pad:", "Batch Count:", "Column:", "Delimiter:" are hard-coded English.

### 🟢 18.6 — No text editing after placement
Once text is added as path shapes, there's no way to edit the text content. It becomes pure geometry. A "Convert to paths" vs "keep editable" option would be useful.

---

## 19. Materials Library (`ui/materials.rs`)

### 🟡 19.1 — No categorization/folders for material presets
All presets appear in a flat `ComboBox`. As the library grows, it becomes hard to find materials. Add categories (Wood, Acrylic, Leather, etc.) or a grouped dropdown.

### 🟡 19.2 — No search/filter for materials
Users scrolling through dozens of presets need a search bar or filter field.

### � 19.3 — Duplicate presets on import without deduplication
Importing a JSON file (`📥 Import`) extends `state.presets` without checking for duplicate names. Repeated imports create duplicate entries.

### � 19.4 — `is_favorite` field exists but no favorite UI
`MaterialPreset` has an `is_favorite: bool` field, but the UI doesn't show a star/favorite toggle or sort favorites to the top. The feature is partially implemented but not exposed.

### 19.5 — Labels not internationalized ✅
"Material Presets", "Name:", "Thickness (mm):", "Engrave Speed:", "Cut Speed:", "Recommended Passes:", "Machine Profile:", "Operation:" are hard-coded English.

---

## 20. Wizard (`ui/wizard.rs`)

*Moved to §28 with detailed code-based findings.*

---

## 21. Image Import Dialog (`ui/image_dialog.rs`)

*Moved to §27 with detailed code-based findings.*

---

## 22. Settings Dialog / GRBL Settings (`ui/settings_dialog.rs`)

### 🟡 22.1 — Descriptions are inline but not in tooltips
The dialog already has a Description column via `get_setting_description(id)`, but descriptions are only shown in the grid column. Adding them as tooltips on the `$id` label would be more discoverable when the Description column is off-screen.

### 🟡 22.2 — No "Export/Import GRBL settings" for backup
Users should be able to export their machine settings to a file and re-import them after a firmware flash.

### 🟡 22.3 — "Save to Board" writes ALL settings indiscriminately
Clicking "Save to Board" writes every single setting back to the controller, even unchanged ones. This is slow and risks overwriting values the firmware may have auto-adjusted. Only write changed values.

### 🟢 22.4 — No validation of entered values
Users can enter nonsensical values (e.g., negative steps/mm, non-numeric strings) without warning or input filtering.

### 🟢 22.5 — Labels not internationalized ✅
"Machine Firmware Settings", "Save to Board", "Refresh", "Waiting for settings..." are hard-coded English.

---

## 23. Status Bar (`ui/status_bar.rs`)

### 🟡 23.1 — Override buttons have no reset-to-100% action ✅
Feed/Rapid/Spindle override buttons only have +/- controls. There's no quick "reset to 100%" button. Users must click repeatedly to return to default.

### 🟡 23.2 — Badge text truncation in compact mode is crude ✅
`badge_text[..3.min(badge_text.len())]` truncates status to 3 chars (e.g., "DIS" for DISCONNECTED, "CON" for CONNECTING). These abbreviations are cryptic. Use meaningful abbreviations or just icons.

### 🟢 23.3 — Progress bar shows percentage but no ETA countdown
The estimated time from `file_info` is shown as total but not as remaining time. During a job, showing "~2m 30s remaining" is more useful than the total estimate.

### 🟢 23.4 — Cost estimate not explained ✅
The cost estimate `~0.35€` appears with no tooltip explaining how it's calculated. Users may be confused by the number.

---

## 24. Theme System (`theme.rs`)

### 🟡 24.1 — Accent colors (RED, GREEN, etc.) are global constants
Theme accent colors are defined as `const` and shared across dark/light modes. In light mode, some Catppuccin accent colors (e.g., RED = #F38BA8) have poor contrast against white backgrounds. Consider light-mode-specific accent variants.

### 🟢 24.2 — Custom theme import UI not exposed
The `CustomTheme` struct supports save/load/list but there's no visible UI for importing/managing custom themes. The feature is implemented backend-side but not exposed.

### 🟢 24.3 — No theme preview before applying
Changing themes is instant but there's no preview. A small swatch panel in the View menu showing what each theme looks like would help.

---

## 25. Connection Panel (`ui/connection.rs`)

### 🟡 25.1 — No auto-detection of serial ports
Users must manually select the port. Auto-detecting common laser controller ports (CH340, FTDI, etc.) and highlighting them would reduce friction.

### 🟢 25.2 — No connection history / last-used-port memory
Each time the app opens, users must re-select their port. Remembering the last successful connection would save time.

### 🟢 25.3 — Labels not internationalized ✅
"Port:", "Baud:", "Connect", "Disconnect", "Connection" are hard-coded English. Should use `tr()`.

### 🟢 25.4 — No connection status indicator (LED / icon)
The panel shows Connect/Disconnect buttons but no persistent green/red dot or icon indicating connection status at a glance. Users must look at button text to know if they're connected.

---

## 26. Macros Panel (`ui/macros.rs`)

### 🟡 26.1 — No drag-and-drop reordering of macros ✅
Macros are listed in creation order. Users cannot rearrange them. Add drag reorder or ↑/↓ buttons.

### 🟡 26.2 — No macro categories or folders
All macros appear in a flat list. For users with many macros, grouping (e.g., "Probing", "Homing", "Calibration") would improve navigation.

### 🟡 26.3 — No confirmation before executing a macro ✅
Clicking a macro button immediately sends GCode to the machine. For potentially dangerous macros (e.g., probing), a confirmation or "hold-to-run" pattern would add safety.

### 🟢 26.4 — No macro import/export
Users cannot share macros with others. An import/export to JSON feature would be useful.

### 🟢 26.5 — Edit mode labels not internationalized ✅
"Name:", "GCode (multiline):" are hard-coded English.

### 🟢 26.6 — No GCode syntax highlighting in macro editor
The macro edit field is a plain `text_edit_multiline`. Unlike the GCode Editor which has syntax coloring, macros are edited without highlighting.

---

## 27. Image Import Dialog (`ui/image_dialog.rs`)

### 🔴 27.1 — Preview only updates on parameter change, no immediate feedback ✅
The preview texture updates via `needs_texture_update` flag but some slider changes (smoothing, skeleton) don't trigger it. Users see stale previews for some parameter combinations.

### 🟡 27.2 — No aspect ratio lock for size fields
Width and Height are independent DragValues. Users can distort the image accidentally. Add an aspect ratio lock toggle (🔗 icon).

### 🟡 27.3 — Import button uses hard-coded green color ✅
The Import button uses `Color32::from_rgb(64, 160, 43)` instead of `theme::GREEN`. This breaks theme consistency.

### 🟡 27.4 — SVG layer scroll area is small (150px)
The SVG layer list is limited to 150px. For SVGs with many color groups, this is cramped. Use dynamic height.

### 🟢 27.5 — No "Reset to defaults" button
After experimenting with brightness/contrast/dithering, there's no one-click reset.

### 🟢 27.6 — Labels not internationalized ✅
"Bitmap Import Mode:", "Raster / Photo Settings", "Size:", "Resolution:", "Image Adjustments:", "Laser Settings:", "Cutting Frame (Outline)", "Vector / SVG Settings", "Scaling:", "Layers / Color Mapping:" — all hard-coded English.

### 🟢 27.7 — No drag-and-drop support
Users must use toolbar Open to import images. Drag-and-drop from file explorer is not mentioned as supported.

---

## 28. Startup Wizard (`ui/wizard.rs`)

### 🟡 28.1 — Step 3 title is not internationalized ✅
"Step 3/3 — Controller" is a hard-coded English string, unlike Steps 1 and 2 which use `tr()`.

### 🟡 28.2 — No visual step progress indicator
Steps are labeled "Step 1/3", "Step 2/3", "Step 3/3" as text. A graphical progress bar or dots (●○○ → ●●○ → ●●●) would be clearer.

### 🟡 28.3 — No machine presets / quick profiles
Step 2 requires manually entering workspace dimensions. Common machines (K40, Ortur, xTool, etc.) should have presets that auto-fill dimensions.

### 🟢 28.4 — Navigation labels not internationalized ✅
"Next →", "← Back", "✅ Finish", "Skip wizard" are hard-coded English. Should use `tr()`.

### 🟢 28.5 — No "Don't show again" option with settings access
The "Skip wizard" button finishes the wizard. There's no explicit "Don't show this on startup" preference accessible from settings later.

---

## 29. General / Cross-cutting UX Issues

### 🔴 29.1 — Inconsistent internationalization coverage
Many UI files use `tr()` for labels, but others have hard-coded English strings (camera, nesting, tiling, power test, job queue, shortcuts, generators, arrays, offset). Audit and wrap all user-visible strings.

### 🔴 29.2 — No global toast/notification system
Many actions (batch import, nesting applied, file saved, settings written) complete silently. A toast notification system showing brief success/error messages would improve feedback.

### 🟡 29.3 — No global undo/redo for all operations
Undo/redo exists for shape edits (node_edit.rs) but not for layer parameter changes, nesting, array operations, or other destructive actions.

### 🟡 29.4 — Modal dialogs block all interaction
Most tool windows (Cut Settings, Nesting, Tiling, etc.) are non-modal egui windows, which is good. But some workflows (like calibration wizard) block interaction with other panels. Consider making all wizards non-blocking with clear status indicators.

### 🟡 29.5 — No contextual right-click menus
Right-clicking on shapes, layers, or the canvas doesn't open a context menu. Context menus are standard UX for cut/copy/paste, layer assignment, alignment, etc.

### 🟡 29.6 — No onboarding / first-run tutorial
New users are dropped into a complex interface with no guidance. A brief interactive tutorial or welcome dialog highlighting key areas would reduce the learning curve.

### 🟢 29.7 — No "What's New" changelog display
After updates, users don't know what changed. A brief changelog dialog on first launch after update would help.

### 🟢 29.8 — No workspace auto-save / crash recovery
If the app crashes mid-project, all work is lost. Auto-saving the workspace periodically would prevent data loss.

### 🟢 29.9 — Beginner Mode not fully leveraged
The beginner mode toggle exists in the toolbar but may not actually hide advanced controls. Audit all panels to conditionally hide advanced features (corner power, ramping, kerf calibration, etc.) in beginner mode.

---

---

## 30. Console (`ui/console.rs`)

### 🟡 30.1 — No log filtering/search ✅
The console shows all messages in a single scrolling list. There's no way to filter by type (errors, commands, info) or search for specific text.

### 🟡 30.2 — No log export to file ✅
Users cannot save the console log to a text file for debugging or sharing with support.

### 🟢 30.3 — Command history not persisted across sessions
`ConsoleState::history` is in-memory only. Restarting the app loses all previous commands.

### 🟢 30.4 — No auto-complete for common GCode commands ✅
The input field is plain text. Tab-completion for common commands (G0, G1, M3, M5, $H, $$, etc.) would speed up manual interaction.

---

## 31. Jog Control (`ui/jog.rs`)

### 🟡 31.1 — No keyboard jog support indicated ✅
The jog panel uses on-screen buttons only. There's no indication that keyboard arrow keys can be used for jogging (if implemented elsewhere). Mentioning this in the panel would improve discoverability.

### 🟡 31.2 — No continuous jog mode
Jog buttons send discrete step commands. A continuous jog (hold-to-move) mode is standard in CNC software for fast positioning.

### 🟢 31.3 — Diagonal buttons are smaller than cardinal ✅
Diagonal buttons (↖↗↙↘) use 32×32px vs 38×38px for cardinal directions. This makes diagonals harder to click. Consider uniform sizing.

---

## 32. Machine State Panel (`ui/machine_state.rs`)

### 🟡 32.1 — Laser Focus toggle has no safety confirmation ✅
Clicking "Laser Focus" immediately fires the laser at low power. A confirmation or "hold to activate" pattern would prevent accidental laser activation.

### 🟡 32.2 — Quick Move buttons don't indicate target coordinates ✅
The buttons (TL, TR, BL, BR, C) move the laser head but don't show the target coordinates. Adding tooltip text like "Move to (0, 0)" would help.

### 🟢 32.3 — Focus label not internationalized ✅
"Laser Focus (ON/OFF)" and quick move labels ("⌜ TL", "⌝ TR", etc.) are hard-coded English.

---

## 33. Preflight Check (`ui/preflight.rs`)

### 🟡 33.1 — Issues list is not actionable
Preflight issues are displayed as text only. Users cannot click on an issue to navigate to the problematic shape or layer. Adding "Go to shape" or "Open layer settings" links would make issues actionable.

### 🟡 33.2 — Open path detection has confusing messaging for beginners
"Detected N open path(s). Close contours before launch." — beginners may not understand what "close contours" means. Adding a brief explanation or "Learn more" link would help.

### 🟢 33.3 — O(n²) overlap detection for shapes
The overlap check in `build_preflight_report` is O(n²) over all shapes. For large projects (hundreds of shapes), this could be slow. Consider spatial indexing.

### 🟢 33.4 — Issue messages not internationalized ✅
All preflight messages ("No program loaded.", "Shape #{} uses missing layer index {}.", etc.) are hard-coded English.

---

## 34. Cut Palette (`ui/cut_palette.rs`)

### 🟢 34.1 — Palette shows all 30 layers always
The palette iterates over all layers regardless of whether they're in use. Showing only used layers (like `cut_list.rs` does) would reduce clutter.

### 🟢 34.2 — Tooltip doesn't show layer name ✅
The hover tooltip shows "Layer C00\nSpeed: ...\nPower: ..." but not the user-assigned layer name.

### 🟢 34.3 — Duplicate `is_light` function ✅
Both `cut_palette.rs` and `cut_list.rs` define identical `is_light(c: Color32) -> bool` helper functions. Extract to a shared utility.

---

## 35. Legacy Layers Panel (`ui/layers.rs`)

### 🟡 35.1 — Appears to be dead/legacy code
The file has `#![allow(dead_code)]` and operates on a different `LayerSettings` struct than the main `CutLayer` system. If unused, it should be removed to avoid confusion. If used for GCode-imported files, it should be documented.

### 🟢 35.2 — Labels not internationalized ✅
"Project Layers", "No layers detected.", "Passes:", "Pwr:" are hard-coded English.

---

### 🟡 29.10 — No consistent "Close" button placement across dialogs
Some dialogs have Close on the left, some on the right, some use window X, some use both. Standardize placement.

### 🟡 29.11 — No loading/busy indicator for long operations
Operations like nesting, offset computation, GCode generation have no spinner or progress indication. The UI appears frozen during heavy operations.

### 🟢 29.12 — No zoom level display in status bar ✅
The current zoom level of the preview canvas is not shown anywhere. Users don't know their current magnification.

### 🟢 29.13 — No "Fit to selection" view option ✅
The preview has "Fit to job" but not "Fit to selection" — useful when working on a specific part of a complex project.

### 🟢 29.14 — No panel show/hide toggle ✅
Users cannot hide panels they don't need (e.g., hide the console, hide the camera section) to maximize canvas space.

---

## Summary Statistics

| Priority | Count |
|----------|-------|
| 🔴 High | 6 |
| 🟡 Medium | 62 |
| 🟢 Low | 56 |
| **Total** | **124** |
| ✅ Fixed | 60 |
