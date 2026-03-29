# All4Laser — TODO

## ✅ Terminé — Mise à jour dépendances (eframe 0.34, ureq 3.3, egui 0.34)

- [x] **eframe::App trait** — Ajout stub `fn ui()` obligatoire (`src/app.rs`)
- [x] **ureq v3 API** — `.set()` → `.header()`, `.into_json()` → `.into_body().read_json()` (`src/ai/llm_provider.rs`, `src/app.rs`)
- [x] **Annotations de type** — Closures ureq avec types explicites (`src/ai/llm_provider.rs`)
- [x] **Signature layouter** — `&str` → `&dyn TextBuffer`, `fonts()` → `fonts_mut()` (`src/ui/gcode_editor.rs`)
- [x] **Renommages egui** — `ctx.style/set_style` → `global_style/set_global_style`, `wants_keyboard_input` → `egui_wants_keyboard_input`, `available_rect` → `content_rect` (`src/theme.rs`, `src/app.rs`)
- [x] **Panels dépréciés** — `SidePanel/TopBottomPanel` → `egui::Panel::`, `.default_width/height` → `.default_size`, `.width_range` → `.size_range` (`src/app.rs`)
- [x] **Menus dépréciés** — `menu::bar` → `MenuBar::new().ui()`, `menu::menu_button` → `ui.menu_button()` (`src/ui/toolbar.rs`)
- [x] **close_menu()** → `ui.close()` (`src/ui/toolbar.rs`, `src/preview/renderer.rs`)
- [x] **popup_below_widget** → `egui::Popup::from_response()` (`src/ui/console.rs`)

## ✅ Terminé — Refactorisation structurelle (migration `update()` → `logic()` + `ui()`)

### Migration `update(ctx)` → `ui(&mut Ui)` + `logic(ctx)`

**Contexte** : eframe 0.34 sépare la logique en deux méthodes :
- `fn logic(&mut self, ctx, frame)` — logique non-UI (polling, réseau, input clavier, repaint)
- `fn ui(&mut self, ui, frame)` — construction de l'interface avec un `&mut Ui` racine

✅ **Migration terminée** — `update()` supprimé, `logic()` et `ui()` en place, 0 `#[allow(deprecated)]`.

---

#### Phase 1 — Séparer la logique non-UI dans `logic(ctx)` `src/app.rs`

Déplacer dans `fn logic()` tout ce qui n'affiche pas de UI :

- [x] **1.1** Polling serial : `self.poll_serial()` — lecture série, mise à jour état machine
- [x] **1.2** Polling caméra : `self.poll_camera_stream(ctx)` — frames caméra
- [x] **1.3** Vérification mise à jour : `self.update_receiver` — check GitHub release
- [x] **1.4** Raccourcis clavier globaux : `ctx.egui_wants_keyboard_input()` + `self.handle_keyboard(ctx)`
- [x] **1.5** Drag & drop fichiers : `self.handle_file_drop(ctx)`
- [x] **1.6** Auto-save : `self.perform_autosave()`
- [x] **1.7** Recovery prompt logic (état seulement, pas le rendu de la fenêtre)
- [x] **1.8** Auto-fit après chargement : `self.needs_auto_fit` + `ctx.content_rect()`
- [x] **1.9** Request repaint : `ctx.request_repaint_after(50ms)`
- [x] **1.10** Sync workspace size depuis machine profile

**Test** : `cargo check` + `cargo test` + exécution manuelle — l'app doit se comporter identiquement.

---

#### Phase 2 — Migrer les panels dans `ui(&mut Ui)` `src/app.rs`

Remplacer `.show(ctx, |ui| ...)` par `.show_inside(ui, |ui| ...)` en utilisant le `&mut Ui` racine reçu par `fn ui()`.

L'ordre est important : les panels latéraux doivent être ajoutés **avant** `CentralPanel`.

- [x] **2.1** Menu bar (Industrial) : `Panel::top("menu_bar_panel").show_inside(ui, ...)`
- [x] **2.2** Toolbar : `Panel::top("toolbar").show_inside(ui, ...)`
- [x] **2.3** Status bar : `Panel::bottom("status_bar").show_inside(ui, ...)`
- [x] **2.4** Left panel : `Panel::left("left_panel").show_inside(ui, ...)`
- [x] **2.5** Right panel : `Panel::right("right_panel").show_inside(ui, ...)`
- [x] **2.6** Bottom console : `Panel::bottom("bottom_console_panel").show_inside(ui, ...)`
- [x] **2.7** Central panel (preview) : `CentralPanel::default().show_inside(ui, ...)` — migrer `update_preview()` pour prendre `&mut Ui` au lieu de `&Context`

**Test** : `cargo check` + `cargo test` + vérifier layout identique visuellement.

---

#### Phase 3 — Migrer les fenêtres modales `src/app.rs`

Les `egui::Window::show(ctx, ...)` restent compatibles mais peuvent utiliser `ui.ctx()` depuis `fn ui()`.

- [x] **3.1** Remplacer le paramètre `ctx` par `ui.ctx()` dans les modales : About, Error, Recovery, Update, Preflight, Settings — ✅ déplacés dans `ui()`
- [x] **3.1b** Migrer les modales restantes dans `update_tool_windows`/`update_modals`/`update_import_modal` (Test Fire, Feed Override, Image Import, Shortcuts) — ✅ signature changée de `ctx: &egui::Context` à `ui: &mut egui::Ui`
- [x] **3.2** Idem pour `egui::Area::show(ctx, ...)` — ✅ utilise `ui.ctx()`

**Test** : `cargo check` — les modales restent fonctionnelles.

---

#### Phase 4 — Nettoyage final

- [x] **4.1** Supprimer `fn update()` entièrement et les `#[allow(deprecated)]`
- [x] **4.2** Supprimer les paramètres `ctx: &egui::Context` des méthodes internes — ✅ migré `update_tool_windows`, `update_modals`, `update_import_modal` à `ui: &mut egui::Ui`
- [x] **4.3** Mettre à jour la signature de `update_preview(&mut self, ctx)` → `update_preview(&mut self, ui: &mut egui::Ui)`
- [x] **4.4** `cargo test` — 127 tests passent ✅
- [x] **4.5** `cargo check` — 0 erreurs, 0 warnings (sans `#[allow(deprecated)]`) ✅
- [ ] **4.6** Test fonctionnel complet : lancement, chargement fichier, connexion, preview, modales — à vérifier manuellement

---

#### Points d'attention

- **`ctx` reste accessible** via `ui.ctx()` dans `fn ui()` — pas besoin de tout réécrire
- **`logic()` n'a pas de UI** — ne pas y mettre de `egui::Window` ou panels
- **Ordre des panels** : Top → Bottom → Left → Right → Center (identique à aujourd'hui)
- **Les `egui::Window`** utilisent `ctx` directement, pas un parent `Ui` — ils fonctionnent avec `ui.ctx()` sans changement de signature
- **Risque** : Faible — migration mécanique, pas de changement de comportement

---

## 💡 Améliorations potentielles

### A. Compatibilité eframe 0.35+

- [ ] **A.1** Surveiller la release eframe 0.35 — `update()` sera probablement supprimé
- [ ] **A.2** Compléter la migration Phase 1-4 ci-dessus **avant** de monter en 0.35
- [ ] **A.3** Vérifier si `egui::Window::show(ctx)` change de signature (actuellement fonctionne avec `ui.ctx()`)
- [ ] **A.4** Tester avec `cargo +nightly check` périodiquement pour détecter les breaking changes en amont

### B. Profiling post mise à jour

- [ ] **B.1** Comparer les temps de rendu egui 0.31 → 0.34 (la boucle UI peut avoir changé de perf)
  - Utiliser `ctx.frame_nr()` + `Instant::now()` pour mesurer le frame time moyen
  - Vérifier si `fonts_mut()` (nouveau) est plus coûteux que l'ancien `fonts()`
- [ ] **B.2** Profiler `ureq 3.3` vs ancien `ureq 2.x` — le nouveau runtime est basé sur `http` crate
  - Vérifier latence des appels LLM (Ollama, OpenAI, Gemini)
  - Tester avec des payloads larges (réponses IA longues)
- [ ] **B.3** Mesurer l'impact mémoire de `egui::Panel` unifié vs les anciens `SidePanel`/`TopBottomPanel`
- [ ] **B.4** Vérifier que `request_repaint_after(50ms)` reste le bon intervalle (egui 0.34 a peut-être un repaint plus intelligent)

### C. Nouvelles fonctionnalités egui 0.34 exploitables

#### C.1 — Panels animés (`show_animated_inside`)
- [x] Utiliser `Panel::show_animated_inside()` pour les panels left/right/bottom
  - Animation fluide à l'ouverture/fermeture au lieu d'un toggle instantané
  - **Fichier** : `src/app.rs` — panels left, right, bottom console migrés ✅

#### C.2 — `egui::Popup` unifié
- [x] Remplacer les `context_menu` manuels par `Popup::context_menu()` dans le renderer
  - Meilleur positionnement automatique et gestion du close behavior
  - **Fichier** : `src/preview/renderer.rs` — menu contextuel clic droit migré ✅

#### C.3 — `MenuConfig` et `MenuBar` avec styles
- [x] `MenuBar::new().ui()` déjà utilisé — migration complète depuis l'ancienne API ✅

#### C.4 — `UiBuilder` et `scope_builder`
- [x] Implémenté dans la section Job Preparation (`ui_left_content`)
  - Utilise `ui.scope_builder(egui::UiBuilder::new().id_salt("job_prep").layout(...))`
  - Permet des styles locaux avec justification croisée pour le contenu de la section
  - **Fichier** : `src/app.rs` — scope_builder avec layout personnalisé ✅

#### C.5 — `Theme` natif egui (Dark/Light)
- [x] Optionnel — le système actuel avec `self.light_mode: bool` fonctionne bien ✅

#### C.6 — `Atoms` pour les boutons riches
- [x] Helper `RichButton` créé dans `src/ui/toolbar.rs`
  - `RichButton::new(icon, text).atoms(compact)` retourne `RichText` pour `Button::new()`
  - Supporte le mode compact (icône seule) ou normal (icône + texte)
  - `atoms_colored()` permet de spécifier une couleur personnalisée
  - **Fichier** : `src/ui/toolbar.rs` — helper RichButton pour boutons riches ✅

#### C.7 — `ScrollArea` amélioré
- [x] `content_margin(8.0)` ajouté au ScrollArea de la sidebar left
- [x] `stick_to_bottom(true)` déjà présent dans la console log
  - **Fichiers** : `src/app.rs` ✅, `src/ui/console.rs` ✅
