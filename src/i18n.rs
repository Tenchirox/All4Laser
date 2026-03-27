use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    English,
    French,
    Japanese,
    German,
    Italian,
    Arabic,
    Spanish,
    Portuguese,
    Chinese,
    Russian,
    Turkish,
    Korean,
    Polish,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub fn name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Japanese => "日本語",
            Language::German => "Deutsch",
            Language::Italian => "Italiano",
            Language::Arabic => "العربية",
            Language::Spanish => "Español",
            Language::Portuguese => "Português",
            Language::Chinese => "中文",
            Language::Russian => "Русский",
            Language::Turkish => "Türkçe",
            Language::Korean => "한국어",
            Language::Polish => "Polski",
        }
    }
}

// Global localization store
static DICTIONARY: LazyLock<HashMap<Language, HashMap<&'static str, &'static str>>> = LazyLock::new(
    || {
        let mut m = HashMap::new();

        // English (Base)
        m.insert(Language::English, HashMap::new()); // English is the key itself usually, or fallback

        // French
        let mut fr = HashMap::new();
        fr.insert("Connect", "Connecter");
        fr.insert("Disconnect", "Déconnecter");
        fr.insert("Open", "Ouvrir");
        fr.insert("Save", "Enregistrer");
        fr.insert("Run", "Lancer");
        fr.insert("Stop", "Arrêter");
        fr.insert("Hold", "Pause");
        fr.insert("Resume", "Reprendre");
        fr.insert("Home", "Origine");
        fr.insert("Unlock", "Débloquer");
        fr.insert("Reset", "Réinitialiser");
        fr.insert("Settings", "Paramètres");
        fr.insert("Machine Profile", "Profil Machine");
        fr.insert("Material Library", "Bibliothèque Matériaux");
        fr.insert("Preview", "Aperçu");
        fr.insert("Console", "Console");
        fr.insert("Left Panel", "Panneau Gauche");
        fr.insert("Right Panel", "Panneau Droite");
        fr.insert("Toggle drawing tools and shapes panel", "Afficher/Masquer le panneau des outils de dessin et formes");
        fr.insert("Toggle layers, cuts, and settings panel", "Afficher/Masquer le panneau des calques, coupes et paramètres");
        fr.insert("Toggle console output panel", "Afficher/Masquer le panneau de la console");
        fr.insert("Drawing Tools", "Outils de Dessin");
        fr.insert("Job Transformation", "Transformation Job");
        fr.insert("Z-Probe", "Sonde Z");
        fr.insert("View", "Affichage");
        fr.insert("Theme", "Thème");
        fr.insert("Layout", "Disposition");
        fr.insert("Language", "Langue");
        fr.insert("Controller", "Contrôleur");
        fr.insert("Modern (recommended)", "Moderne (recommandé)");
        fr.insert("Pro (new)", "Pro (nouveau)");
        fr.insert("Industrial (advanced)", "Industriel (avancé)");
        fr.insert("Modern layout (simple)", "Disposition moderne (simple)");
        fr.insert("Classic layout (expert)", "Disposition classique (expert)");
        fr.insert(
            "Pro layout (aesthetic & practical)",
            "Disposition Pro (esthétique & pratique)",
        );
        fr.insert("Beginner Mode", "Mode débutant");
        fr.insert("Connection & Control", "Connexion & Contrôle");
        fr.insert("Job Preparation", "Préparation du job");
        fr.insert("Creation & Editing", "Création & édition");
        fr.insert("Advanced Tools", "Outils avancés");
        fr.insert(
        "Beginner mode active: interface simplified. Disable it in View to show all tools.",
        "Mode débutant actif : interface simplifiée. Désactivez-le dans Affichage pour voir tous les outils.",
    );
        fr.insert("Cuts", "Coupes");
        fr.insert("Move", "Déplacer");
        fr.insert("Laser", "Laser");
        fr.insert("Layers", "Couches");
        fr.insert("Notes", "Notes");
        fr.insert("Project Notes", "Notes du projet");
        fr.insert("Measure", "Mesurer");
        fr.insert("Group", "Grouper");
        fr.insert("Ungroup", "Dégrouper");
        fr.insert("Copy", "Copier");
        fr.insert("Cut", "Couper");
        fr.insert("Paste", "Coller");
        fr.insert("Duplicate", "Dupliquer");
        fr.insert("Select All", "Tout sélectionner");
        fr.insert("Air Assist", "Air Assist");
        fr.insert("Exhaust Fan", "Ventilation");
        fr.insert("Power Ramping", "Rampe de puissance");
        fr.insert("Perforation", "Perforation");
        fr.insert("Construction Layer", "Couche de construction");
        fr.insert("Maintenance", "Maintenance");
        fr.insert("Cost Estimate", "Estimation du coût");
        fr.insert("Export SVG", "Exporter SVG");
        fr.insert("Startup Wizard", "Assistant de démarrage");
        // Toolbar & menus
        fr.insert("File", "Fichier");
        fr.insert("Edit", "Édition");
        fr.insert("Undo", "Annuler");
        fr.insert("Redo", "Rétablir");
        fr.insert("Zoom In", "Zoom avant");
        fr.insert("Zoom Out", "Zoom arrière");
        fr.insert("Recent Files", "Fichiers récents");
        fr.insert("No recent files", "Aucun fichier récent");
        fr.insert("Project", "Projet");
        fr.insert("Open Project", "Ouvrir un projet");
        fr.insert("Save Project", "Enregistrer le projet");
        fr.insert("Export Job Report", "Exporter le rapport");
        fr.insert("Frame", "Cadrage");
        fr.insert("Dry Run", "Test à vide");
        fr.insert("Set Zero", "Définir zéro");
        fr.insert("Zero", "Zéro");
        fr.insert("Tools", "Outils");
        fr.insert("Power/Speed Test", "Test puissance/vitesse");
        fr.insert("Test Fire", "Tir d'essai");
        fr.insert("GCode Editor", "Éditeur GCode");
        fr.insert("Tiling", "Pavage");
        fr.insert("Auto Nesting", "Imbrication auto");
        fr.insert("Job Queue", "File d'attente");
        fr.insert("Shortcuts", "Raccourcis");
        fr.insert("Dark UI", "Interface sombre");
        fr.insert("Light UI", "Interface claire");
        fr.insert("Save Layer Template", "Enregistrer modèle de couche");
        fr.insert("Load Layer Template", "Charger modèle de couche");
        fr.insert("Help", "Aide");
        fr.insert("About", "À propos");
        // Jog panel
        fr.insert("Jog Control", "Contrôle de déplacement");
        fr.insert("Step:", "Pas :");
        fr.insert("Feed:", "Avance :");
        // Preview panel
        fr.insert("Rapids", "Rapides");
        fr.insert("Fill", "Remplissage");
        fr.insert("Risk", "Risque");
        fr.insert("Realistic", "Réaliste");
        fr.insert("Simulation", "Simulation");
        fr.insert("Zoom in", "Zoom avant");
        fr.insert("Zoom out", "Zoom arrière");
        fr.insert("Fit", "Ajuster");
        // Machine state
        fr.insert("Quick Move (Bounds)", "Déplacement rapide (Limites)");
        fr.insert("Spindle:", "Broche :");
        // Macros
        fr.insert("Macros", "Macros");
        fr.insert("New Macro", "Nouvelle macro");
        fr.insert("Delete", "Supprimer");
        // Job queue
        fr.insert("Pending Queue", "File d'attente");
        fr.insert("No queued jobs.", "Aucun job en attente.");
        fr.insert("Execution History", "Historique d'exécution");
        fr.insert("No history yet.", "Aucun historique.");
        // Alignment
        fr.insert("Align:", "Aligner :");
        // Preflight
        fr.insert("Launch Anyway", "Lancer quand même");
        fr.insert("Cannot launch job with critical errors.", "Impossible de lancer le job avec des erreurs critiques.");
        // Shortcuts dialog
        fr.insert("Key", "Touche");
        fr.insert("Action", "Action");
        // Generators
        fr.insert("Object Generators", "Générateurs d'objets");
        fr.insert("QR Code Generator", "Générateur QR Code");
        fr.insert("Box Maker (Finger Joints)", "Générateur de boîte (joints à doigts)");
        fr.insert("Living Hinge", "Charnière vivante");
        fr.insert("Print & Cut Fiducials", "Repères Print & Cut");
        // Drawing
        fr.insert("Add Shape", "Ajouter forme");
        fr.insert("Clear", "Effacer");
        // Connection
        fr.insert("Port:", "Port :");
        fr.insert("Baud:", "Débit :");
        fr.insert("Refresh", "Actualiser");
        // Camera
        fr.insert("Camera", "Caméra");
        fr.insert("Live stream active", "Flux vidéo actif");
        // Status bar
        fr.insert("Unit Toggle", "Changer unité");
        // Nesting
        fr.insert("No selection: fallback to all shapes.", "Aucune sélection : toutes les formes seront utilisées.");
        // General
        fr.insert("Cancel", "Annuler");
        fr.insert("Apply", "Appliquer");
        fr.insert("Close", "Fermer");
        fr.insert("Generate", "Générer");
        fr.insert("Export", "Exporter");
        fr.insert("Import", "Importer");
        m.insert(Language::French, fr);

        // Japanese
        let mut ja = HashMap::new();
        ja.insert("Connect", "接続");
        ja.insert("Disconnect", "切断");
        ja.insert("Open", "開く");
        ja.insert("Save", "保存");
        ja.insert("Run", "実行");
        ja.insert("Stop", "停止");
        ja.insert("Hold", "一時停止");
        ja.insert("Resume", "再開");
        ja.insert("Home", "原点復帰");
        ja.insert("Unlock", "ロック解除");
        ja.insert("Reset", "リセット");
        ja.insert("Settings", "設定");
        ja.insert("Machine Profile", "マシン設定");
        ja.insert("Material Library", "素材ライブラリ");
        ja.insert("Preview", "プレビュー");
        ja.insert("Console", "コンソール");
        ja.insert("Left Panel", "左パネル");
        ja.insert("Right Panel", "右パネル");
        ja.insert("Toggle drawing tools and shapes panel", "描画ツールと図形パネルの表示/非表示");
        ja.insert("Toggle layers, cuts, and settings panel", "レイヤー、カット、設定パネルの表示/非表示");
        ja.insert("Toggle console output panel", "コンソール出力パネルの表示/非表示");
        ja.insert("Drawing Tools", "描画ツール");
        ja.insert("Job Transformation", "ジョブ変換");
        ja.insert("Z-Probe", "Zプローブ");
        ja.insert("View", "表示");
        ja.insert("Theme", "テーマ");
        ja.insert("Layout", "レイアウト");
        ja.insert("Language", "言語");
        ja.insert("Controller", "コントローラー");
        ja.insert("Modern (recommended)", "モダン（推奨）");
        ja.insert("Pro (new)", "プロ (新規)");
        ja.insert("Industrial (advanced)", "インダストリアル（上級者向け）");
        ja.insert("Modern layout (simple)", "モダンレイアウト（シンプル）");
        ja.insert(
            "Classic layout (expert)",
            "クラシックレイアウト（上級者向け）",
        );
        ja.insert(
            "Pro layout (aesthetic & practical)",
            "プロレイアウト（美的で実用的）",
        );
        ja.insert("Beginner Mode", "初心者モード");
        ja.insert("Connection & Control", "接続と操作");
        ja.insert("Job Preparation", "ジョブ準備");
        ja.insert("Creation & Editing", "作成と編集");
        ja.insert("Advanced Tools", "上級ツール");
        ja.insert(
        "Beginner mode active: interface simplified. Disable it in View to show all tools.",
        "初心者モードが有効です：UIは簡略化されています。すべてのツールを表示するには表示メニューで無効化してください。",
    );
        ja.insert("Cuts", "カット");
        ja.insert("Move", "移動");
        ja.insert("Laser", "レーザー");
        ja.insert("Layers", "レイヤー");
        ja.insert("Notes", "メモ");
        ja.insert("Project Notes", "プロジェクトメモ");
        ja.insert("Measure", "計測");
        ja.insert("Group", "グループ化");
        ja.insert("Ungroup", "グループ解除");
        ja.insert("Copy", "コピー");
        ja.insert("Cut", "切り取り");
        ja.insert("Paste", "貼り付け");
        ja.insert("Duplicate", "複製");
        ja.insert("Select All", "すべて選択");
        ja.insert("Air Assist", "エアアシスト");
        ja.insert("Exhaust Fan", "排気ファン");
        ja.insert("Power Ramping", "パワーランピング");
        ja.insert("Perforation", "穴あけ");
        ja.insert("Construction Layer", "補助レイヤー");
        ja.insert("Maintenance", "メンテナンス");
        ja.insert("Cost Estimate", "コスト見積");
        ja.insert("Export SVG", "SVGエクスポート");
        ja.insert("Startup Wizard", "セットアップウィザード");
        ja.insert("File", "ファイル");
        ja.insert("Edit", "編集");
        ja.insert("Undo", "元に戻す");
        ja.insert("Redo", "やり直し");
        ja.insert("Zoom In", "拡大");
        ja.insert("Zoom Out", "縮小");
        ja.insert("Recent Files", "最近のファイル");
        ja.insert("No recent files", "最近のファイルはありません");
        ja.insert("Project", "プロジェクト");
        ja.insert("Open Project", "プロジェクトを開く");
        ja.insert("Save Project", "プロジェクトを保存");
        ja.insert("Export Job Report", "ジョブレポートをエクスポート");
        ja.insert("Frame", "フレーム");
        ja.insert("Dry Run", "テスト実行");
        ja.insert("Set Zero", "原点設定");
        ja.insert("Zero", "ゼロ");
        ja.insert("Tools", "ツール");
        ja.insert("Power/Speed Test", "出力/速度テスト");
        ja.insert("Test Fire", "テスト発射");
        ja.insert("GCode Editor", "GCodeエディター");
        ja.insert("Tiling", "タイリング");
        ja.insert("Auto Nesting", "自動ネスティング");
        ja.insert("Job Queue", "ジョブキュー");
        ja.insert("Shortcuts", "ショートカット");
        ja.insert("Dark UI", "ダークUI");
        ja.insert("Light UI", "ライトUI");
        ja.insert("Save Layer Template", "レイヤーテンプレートを保存");
        ja.insert("Load Layer Template", "レイヤーテンプレートを読込");
        ja.insert("Help", "ヘルプ");
        ja.insert("About", "バージョン情報");
        ja.insert("Jog Control", "ジョグ操作");
        ja.insert("Step:", "ステップ:");
        ja.insert("Feed:", "送り速度:");
        ja.insert("Rapids", "早送り");
        ja.insert("Fill", "塗りつぶし");
        ja.insert("Risk", "リスク");
        ja.insert("Realistic", "リアル");
        ja.insert("Simulation", "シミュレーション");
        ja.insert("Zoom in", "拡大");
        ja.insert("Zoom out", "縮小");
        ja.insert("Fit", "全体表示");
        ja.insert("Quick Move (Bounds)", "クイック移動（範囲）");
        ja.insert("Spindle:", "スピンドル:");
        ja.insert("Macros", "マクロ");
        ja.insert("New Macro", "新規マクロ");
        ja.insert("Delete", "削除");
        ja.insert("Pending Queue", "待機キュー");
        ja.insert("No queued jobs.", "キューにジョブはありません。");
        ja.insert("Execution History", "実行履歴");
        ja.insert("No history yet.", "履歴はまだありません。");
        ja.insert("Align:", "配置:");
        ja.insert("Launch Anyway", "強制実行");
        ja.insert("Cannot launch job with critical errors.", "重大なエラーがあるため実行できません。");
        ja.insert("Key", "キー");
        ja.insert("Action", "アクション");
        ja.insert("Object Generators", "オブジェクト生成");
        ja.insert("QR Code Generator", "QRコード生成");
        ja.insert("Add Shape", "図形を追加");
        ja.insert("Clear", "クリア");
        ja.insert("Cancel", "キャンセル");
        ja.insert("Apply", "適用");
        ja.insert("Close", "閉じる");
        m.insert(Language::Japanese, ja);

        // German
        let mut de = HashMap::new();
        de.insert("Connect", "Verbinden");
        de.insert("Disconnect", "Trennen");
        de.insert("Open", "Öffnen");
        de.insert("Save", "Speichern");
        de.insert("Run", "Start");
        de.insert("Stop", "Stopp");
        de.insert("Hold", "Pause");
        de.insert("Resume", "Fortsetzen");
        de.insert("Home", "Referenzfahrt");
        de.insert("Unlock", "Entsperren");
        de.insert("Reset", "Zurücksetzen");
        de.insert("Settings", "Einstellungen");
        de.insert("Machine Profile", "Maschinenprofil");
        de.insert("Material Library", "Materialbibliothek");
        de.insert("Preview", "Vorschau");
        de.insert("Console", "Konsole");
        de.insert("Left Panel", "Linke Leiste");
        de.insert("Right Panel", "Rechte Leiste");
        de.insert("Toggle drawing tools and shapes panel", "Zeichenwerkzeuge und Formen-Leiste ein/ausblenden");
        de.insert("Toggle layers, cuts, and settings panel", "Ebenen, Schnitte und Einstellungen-Leiste ein/ausblenden");
        de.insert("Toggle console output panel", "Konsolenausgabe-Leiste ein/ausblenden");
        de.insert("Drawing Tools", "Zeichenwerkzeuge");
        de.insert("Job Transformation", "Job-Transformation");
        de.insert("Z-Probe", "Z-Sonde");
        de.insert("View", "Ansicht");
        de.insert("Theme", "Thema");
        de.insert("Layout", "Layout");
        de.insert("Language", "Sprache");
        de.insert("Controller", "Controller");
        de.insert("Modern (recommended)", "Modern (empfohlen)");
        de.insert("Pro (new)", "Pro (neu)");
        de.insert("Industrial (advanced)", "Industriell (erweitert)");
        de.insert("Modern layout (simple)", "Modernes Layout (einfach)");
        de.insert("Classic layout (expert)", "Klassisches Layout (Experte)");
        de.insert(
            "Pro layout (aesthetic & practical)",
            "Pro Layout (ästhetisch & praktisch)",
        );
        de.insert("Beginner Mode", "Anfängermodus");
        de.insert("Connection & Control", "Verbindung & Steuerung");
        de.insert("Job Preparation", "Jobvorbereitung");
        de.insert("Creation & Editing", "Erstellung & Bearbeitung");
        de.insert("Advanced Tools", "Erweiterte Werkzeuge");
        de.insert(
        "Beginner mode active: interface simplified. Disable it in View to show all tools.",
        "Anfängermodus aktiv: Die Oberfläche ist vereinfacht. Deaktivieren Sie ihn in Ansicht, um alle Werkzeuge zu sehen.",
    );
        de.insert("Cuts", "Schnitte");
        de.insert("Move", "Bewegen");
        de.insert("Laser", "Laser");
        de.insert("Layers", "Ebenen");
        de.insert("Notes", "Notizen");
        de.insert("Project Notes", "Projektnotizen");
        de.insert("Measure", "Messen");
        de.insert("Group", "Gruppieren");
        de.insert("Ungroup", "Gruppierung aufheben");
        de.insert("Copy", "Kopieren");
        de.insert("Cut", "Ausschneiden");
        de.insert("Paste", "Einfügen");
        de.insert("Duplicate", "Duplizieren");
        de.insert("Select All", "Alles auswählen");
        de.insert("Air Assist", "Luftunterstützung");
        de.insert("Exhaust Fan", "Absaugventilator");
        de.insert("Power Ramping", "Leistungsrampe");
        de.insert("Perforation", "Perforation");
        de.insert("Construction Layer", "Konstruktionsebene");
        de.insert("Maintenance", "Wartung");
        de.insert("Cost Estimate", "Kostenschätzung");
        de.insert("Export SVG", "SVG exportieren");
        de.insert("Startup Wizard", "Einrichtungsassistent");
        de.insert("File", "Datei");
        de.insert("Edit", "Bearbeiten");
        de.insert("Undo", "Rückgängig");
        de.insert("Redo", "Wiederholen");
        de.insert("Zoom In", "Vergrößern");
        de.insert("Zoom Out", "Verkleinern");
        de.insert("Recent Files", "Zuletzt geöffnet");
        de.insert("No recent files", "Keine zuletzt geöffneten Dateien");
        de.insert("Project", "Projekt");
        de.insert("Open Project", "Projekt öffnen");
        de.insert("Save Project", "Projekt speichern");
        de.insert("Export Job Report", "Jobbericht exportieren");
        de.insert("Frame", "Rahmen");
        de.insert("Dry Run", "Testlauf");
        de.insert("Set Zero", "Nullpunkt setzen");
        de.insert("Zero", "Null");
        de.insert("Tools", "Werkzeuge");
        de.insert("Power/Speed Test", "Leistung/Geschwindigkeit Test");
        de.insert("Test Fire", "Testschuss");
        de.insert("GCode Editor", "GCode-Editor");
        de.insert("Tiling", "Kachelung");
        de.insert("Auto Nesting", "Auto-Verschachtelung");
        de.insert("Job Queue", "Auftragswarteschlange");
        de.insert("Shortcuts", "Tastenkürzel");
        de.insert("Dark UI", "Dunkle Oberfläche");
        de.insert("Light UI", "Helle Oberfläche");
        de.insert("Save Layer Template", "Ebenenvorlage speichern");
        de.insert("Load Layer Template", "Ebenenvorlage laden");
        de.insert("Help", "Hilfe");
        de.insert("About", "Über");
        de.insert("Jog Control", "Jogsteuerung");
        de.insert("Step:", "Schritt:");
        de.insert("Feed:", "Vorschub:");
        de.insert("Rapids", "Eilgang");
        de.insert("Fill", "Füllung");
        de.insert("Risk", "Risiko");
        de.insert("Realistic", "Realistisch");
        de.insert("Simulation", "Simulation");
        de.insert("Zoom in", "Vergrößern");
        de.insert("Zoom out", "Verkleinern");
        de.insert("Fit", "Einpassen");
        de.insert("Quick Move (Bounds)", "Schnellfahrt (Grenzen)");
        de.insert("Spindle:", "Spindel:");
        de.insert("Macros", "Makros");
        de.insert("New Macro", "Neues Makro");
        de.insert("Delete", "Löschen");
        de.insert("Pending Queue", "Warteschlange");
        de.insert("No queued jobs.", "Keine Aufträge in der Warteschlange.");
        de.insert("Execution History", "Ausführungsverlauf");
        de.insert("No history yet.", "Noch kein Verlauf.");
        de.insert("Align:", "Ausrichten:");
        de.insert("Launch Anyway", "Trotzdem starten");
        de.insert("Cannot launch job with critical errors.", "Job kann nicht mit kritischen Fehlern gestartet werden.");
        de.insert("Key", "Taste");
        de.insert("Action", "Aktion");
        de.insert("Object Generators", "Objektgeneratoren");
        de.insert("QR Code Generator", "QR-Code-Generator");
        de.insert("Add Shape", "Form hinzufügen");
        de.insert("Clear", "Löschen");
        de.insert("Cancel", "Abbrechen");
        de.insert("Apply", "Anwenden");
        de.insert("Close", "Schließen");
        m.insert(Language::German, de);

        // Italian
        let mut it = HashMap::new();
        it.insert("Connect", "Connetti");
        it.insert("Disconnect", "Disconnetti");
        it.insert("Open", "Apri");
        it.insert("Save", "Salva");
        it.insert("Run", "Avvia");
        it.insert("Stop", "Ferma");
        it.insert("Hold", "Pausa");
        it.insert("Resume", "Riprendi");
        it.insert("Home", "Home");
        it.insert("Unlock", "Sblocca");
        it.insert("Reset", "Resetta");
        it.insert("Settings", "Impostazioni");
        it.insert("Machine Profile", "Profilo Macchina");
        it.insert("Material Library", "Libreria Materiali");
        it.insert("Preview", "Anteprima");
        it.insert("Console", "Console");
        it.insert("Left Panel", "Pannello Sinistro");
        it.insert("Right Panel", "Pannello Destro");
        it.insert("Toggle drawing tools and shapes panel", "Mostra/Nascondi pannello strumenti di disegno e forme");
        it.insert("Toggle layers, cuts, and settings panel", "Mostra/Nascondi pannello livelli, tagli e impostazioni");
        it.insert("Toggle console output panel", "Mostra/Nascondi pannello output console");
        it.insert("Drawing Tools", "Strumenti Disegno");
        it.insert("Job Transformation", "Trasformazione Lavoro");
        it.insert("Z-Probe", "Sonda Z");
        it.insert("View", "Vista");
        it.insert("Theme", "Tema");
        it.insert("Layout", "Layout");
        it.insert("Language", "Lingua");
        it.insert("Controller", "Controller");
        it.insert("Modern (recommended)", "Moderno (consigliato)");
        it.insert("Pro (new)", "Pro (nuovo)");
        it.insert("Industrial (advanced)", "Industriale (avanzato)");
        it.insert("Modern layout (simple)", "Layout moderno (semplice)");
        it.insert("Classic layout (expert)", "Layout classico (esperto)");
        it.insert(
            "Pro layout (aesthetic & practical)",
            "Layout Pro (estetico & pratico)",
        );
        it.insert("Beginner Mode", "Modalità principiante");
        it.insert("Connection & Control", "Connessione e Controllo");
        it.insert("Job Preparation", "Preparazione lavoro");
        it.insert("Creation & Editing", "Creazione e modifica");
        it.insert("Advanced Tools", "Strumenti avanzati");
        it.insert(
        "Beginner mode active: interface simplified. Disable it in View to show all tools.",
        "Modalità principiante attiva: interfaccia semplificata. Disattivala in Vista per mostrare tutti gli strumenti.",
    );
        it.insert("Cuts", "Tagli");
        it.insert("Move", "Sposta");
        it.insert("Laser", "Laser");
        it.insert("Layers", "Livelli");
        it.insert("Notes", "Note");
        it.insert("Project Notes", "Note del progetto");
        it.insert("Measure", "Misura");
        it.insert("Group", "Raggruppa");
        it.insert("Ungroup", "Separa");
        it.insert("Copy", "Copia");
        it.insert("Cut", "Taglia");
        it.insert("Paste", "Incolla");
        it.insert("Duplicate", "Duplica");
        it.insert("Select All", "Seleziona tutto");
        it.insert("Air Assist", "Aria Assistita");
        it.insert("Exhaust Fan", "Ventilatore aspirazione");
        it.insert("Power Ramping", "Rampa di potenza");
        it.insert("Perforation", "Perforazione");
        it.insert("Construction Layer", "Livello costruzione");
        it.insert("Maintenance", "Manutenzione");
        it.insert("Cost Estimate", "Stima dei costi");
        it.insert("Export SVG", "Esporta SVG");
        it.insert("Startup Wizard", "Assistente di avvio");
        it.insert("File", "File");
        it.insert("Edit", "Modifica");
        it.insert("Undo", "Annulla");
        it.insert("Redo", "Ripeti");
        it.insert("Zoom In", "Ingrandisci");
        it.insert("Zoom Out", "Riduci");
        it.insert("Recent Files", "File recenti");
        it.insert("No recent files", "Nessun file recente");
        it.insert("Project", "Progetto");
        it.insert("Open Project", "Apri progetto");
        it.insert("Save Project", "Salva progetto");
        it.insert("Export Job Report", "Esporta rapporto lavoro");
        it.insert("Frame", "Cornice");
        it.insert("Dry Run", "Prova a vuoto");
        it.insert("Set Zero", "Imposta zero");
        it.insert("Zero", "Zero");
        it.insert("Tools", "Strumenti");
        it.insert("Power/Speed Test", "Test potenza/velocità");
        it.insert("Test Fire", "Tiro di prova");
        it.insert("GCode Editor", "Editor GCode");
        it.insert("Tiling", "Piastrellatura");
        it.insert("Auto Nesting", "Nesting automatico");
        it.insert("Job Queue", "Coda lavori");
        it.insert("Shortcuts", "Scorciatoie");
        it.insert("Dark UI", "Interfaccia scura");
        it.insert("Light UI", "Interfaccia chiara");
        it.insert("Save Layer Template", "Salva modello livello");
        it.insert("Load Layer Template", "Carica modello livello");
        it.insert("Help", "Aiuto");
        it.insert("About", "Informazioni");
        it.insert("Jog Control", "Controllo Jog");
        it.insert("Step:", "Passo:");
        it.insert("Feed:", "Avanzamento:");
        it.insert("Rapids", "Rapidi");
        it.insert("Fill", "Riempimento");
        it.insert("Risk", "Rischio");
        it.insert("Realistic", "Realistico");
        it.insert("Simulation", "Simulazione");
        it.insert("Zoom in", "Ingrandisci");
        it.insert("Zoom out", "Riduci");
        it.insert("Fit", "Adatta");
        it.insert("Quick Move (Bounds)", "Spostamento rapido (Limiti)");
        it.insert("Spindle:", "Mandrino:");
        it.insert("Macros", "Macro");
        it.insert("New Macro", "Nuova macro");
        it.insert("Delete", "Elimina");
        it.insert("Pending Queue", "Coda in attesa");
        it.insert("No queued jobs.", "Nessun lavoro in coda.");
        it.insert("Execution History", "Cronologia esecuzione");
        it.insert("No history yet.", "Nessuna cronologia.");
        it.insert("Align:", "Allinea:");
        it.insert("Launch Anyway", "Avvia comunque");
        it.insert("Cannot launch job with critical errors.", "Impossibile avviare il lavoro con errori critici.");
        it.insert("Key", "Tasto");
        it.insert("Action", "Azione");
        it.insert("Object Generators", "Generatori oggetti");
        it.insert("QR Code Generator", "Generatore QR Code");
        it.insert("Add Shape", "Aggiungi forma");
        it.insert("Clear", "Cancella");
        it.insert("Cancel", "Annulla");
        it.insert("Apply", "Applica");
        it.insert("Close", "Chiudi");
        m.insert(Language::Italian, it);

        // Spanish
        let mut es = HashMap::new();
        es.insert("Connect", "Conectar");
        es.insert("Disconnect", "Desconectar");
        es.insert("Open", "Abrir");
        es.insert("Save", "Guardar");
        es.insert("Run", "Ejecutar");
        es.insert("Stop", "Detener");
        es.insert("Hold", "Pausa");
        es.insert("Resume", "Reanudar");
        es.insert("Home", "Inicio");
        es.insert("Unlock", "Desbloquear");
        es.insert("Reset", "Reiniciar");
        es.insert("Settings", "Ajustes");
        es.insert("Machine Profile", "Perfil de Máquina");
        es.insert("Material Library", "Biblioteca de Materiales");
        es.insert("Preview", "Vista Previa");
        es.insert("Console", "Consola");
        es.insert("Left Panel", "Panel Izquierdo");
        es.insert("Right Panel", "Panel Derecho");
        es.insert("Toggle drawing tools and shapes panel", "Mostrar/Ocultar panel de herramientas de dibujo y formas");
        es.insert("Toggle layers, cuts, and settings panel", "Mostrar/Ocultar panel de capas, cortes y configuraciones");
        es.insert("Toggle console output panel", "Mostrar/Ocultar panel de salida de consola");
        es.insert("Drawing Tools", "Herramientas de Dibujo");
        es.insert("Job Transformation", "Transformación Trabajo");
        es.insert("Z-Probe", "Sonda Z");
        es.insert("View", "Ver");
        es.insert("Theme", "Tema");
        es.insert("Layout", "Diseño");
        es.insert("Language", "Idioma");
        es.insert("Controller", "Controlador");
        es.insert("Modern (recommended)", "Moderno (recomendado)");
        es.insert("Pro (new)", "Pro (nuevo)");
        es.insert("Industrial (advanced)", "Industrial (avanzado)");
        es.insert("Modern layout (simple)", "Diseño moderno (simple)");
        es.insert("Classic layout (expert)", "Diseño clásico (experto)");
        es.insert(
            "Pro layout (aesthetic & practical)",
            "Diseño Pro (estético y práctico)",
        );
        es.insert("Beginner Mode", "Modo principiante");
        es.insert("Connection & Control", "Conexión y control");
        es.insert("Job Preparation", "Preparación del trabajo");
        es.insert("Creation & Editing", "Creación y edición");
        es.insert("Advanced Tools", "Herramientas avanzadas");
        es.insert(
        "Beginner mode active: interface simplified. Disable it in View to show all tools.",
        "Modo principiante activo: interfaz simplificada. Desactívalo en Ver para mostrar todas las herramientas.",
    );
        es.insert("Cuts", "Cortes");
        es.insert("Move", "Mover");
        es.insert("Laser", "Láser");
        es.insert("Layers", "Capas");
        es.insert("Notes", "Notas");
        es.insert("Project Notes", "Notas del proyecto");
        es.insert("Measure", "Medir");
        es.insert("Group", "Agrupar");
        es.insert("Ungroup", "Desagrupar");
        es.insert("Copy", "Copiar");
        es.insert("Cut", "Cortar");
        es.insert("Paste", "Pegar");
        es.insert("Duplicate", "Duplicar");
        es.insert("Select All", "Seleccionar todo");
        es.insert("Air Assist", "Asistencia de aire");
        es.insert("Exhaust Fan", "Ventilador de extracción");
        es.insert("Power Ramping", "Rampa de potencia");
        es.insert("Perforation", "Perforación");
        es.insert("Construction Layer", "Capa de construcción");
        es.insert("Maintenance", "Mantenimiento");
        es.insert("Cost Estimate", "Estimación de costos");
        es.insert("Export SVG", "Exportar SVG");
        es.insert("Startup Wizard", "Asistente de inicio");
        es.insert("File", "Archivo");
        es.insert("Edit", "Editar");
        es.insert("Undo", "Deshacer");
        es.insert("Redo", "Rehacer");
        es.insert("Zoom In", "Acercar");
        es.insert("Zoom Out", "Alejar");
        es.insert("Recent Files", "Archivos recientes");
        es.insert("No recent files", "Sin archivos recientes");
        es.insert("Project", "Proyecto");
        es.insert("Open Project", "Abrir proyecto");
        es.insert("Save Project", "Guardar proyecto");
        es.insert("Export Job Report", "Exportar informe");
        es.insert("Frame", "Encuadre");
        es.insert("Dry Run", "Prueba en seco");
        es.insert("Set Zero", "Establecer cero");
        es.insert("Zero", "Cero");
        es.insert("Tools", "Herramientas");
        es.insert("Power/Speed Test", "Test potencia/velocidad");
        es.insert("Test Fire", "Disparo de prueba");
        es.insert("GCode Editor", "Editor GCode");
        es.insert("Tiling", "Mosaico");
        es.insert("Auto Nesting", "Anidación automática");
        es.insert("Job Queue", "Cola de trabajos");
        es.insert("Shortcuts", "Atajos");
        es.insert("Dark UI", "Interfaz oscura");
        es.insert("Light UI", "Interfaz clara");
        es.insert("Save Layer Template", "Guardar plantilla de capa");
        es.insert("Load Layer Template", "Cargar plantilla de capa");
        es.insert("Help", "Ayuda");
        es.insert("About", "Acerca de");
        es.insert("Jog Control", "Control de movimiento");
        es.insert("Step:", "Paso:");
        es.insert("Feed:", "Avance:");
        es.insert("Rapids", "Rápidos");
        es.insert("Fill", "Relleno");
        es.insert("Risk", "Riesgo");
        es.insert("Realistic", "Realista");
        es.insert("Simulation", "Simulación");
        es.insert("Zoom in", "Acercar");
        es.insert("Zoom out", "Alejar");
        es.insert("Fit", "Ajustar");
        es.insert("Quick Move (Bounds)", "Movimiento rápido (Límites)");
        es.insert("Spindle:", "Husillo:");
        es.insert("Macros", "Macros");
        es.insert("New Macro", "Nueva macro");
        es.insert("Delete", "Eliminar");
        es.insert("Pending Queue", "Cola pendiente");
        es.insert("No queued jobs.", "Sin trabajos en cola.");
        es.insert("Execution History", "Historial de ejecución");
        es.insert("No history yet.", "Sin historial.");
        es.insert("Align:", "Alinear:");
        es.insert("Launch Anyway", "Lanzar de todos modos");
        es.insert("Cannot launch job with critical errors.", "No se puede lanzar el trabajo con errores críticos.");
        es.insert("Key", "Tecla");
        es.insert("Action", "Acción");
        es.insert("Object Generators", "Generadores de objetos");
        es.insert("QR Code Generator", "Generador QR");
        es.insert("Add Shape", "Añadir forma");
        es.insert("Clear", "Borrar");
        es.insert("Cancel", "Cancelar");
        es.insert("Apply", "Aplicar");
        es.insert("Close", "Cerrar");
        m.insert(Language::Spanish, es);

        // Portuguese
        let mut pt = HashMap::new();
        pt.insert("Connect", "Conectar");
        pt.insert("Disconnect", "Desconectar");
        pt.insert("Open", "Abrir");
        pt.insert("Save", "Salvar");
        pt.insert("Run", "Executar");
        pt.insert("Stop", "Parar");
        pt.insert("Hold", "Pausar");
        pt.insert("Resume", "Retomar");
        pt.insert("Home", "Início");
        pt.insert("Unlock", "Desbloquear");
        pt.insert("Reset", "Reiniciar");
        pt.insert("Settings", "Configurações");
        pt.insert("Machine Profile", "Perfil da Máquina");
        pt.insert("Material Library", "Biblioteca de Materiais");
        pt.insert("Preview", "Pré-visualização");
        pt.insert("Console", "Console");
        pt.insert("Left Panel", "Painel Esquerdo");
        pt.insert("Right Panel", "Painel Direito");
        pt.insert("Toggle drawing tools and shapes panel", "Mostrar/Ocultar painel de ferramentas de desenho e formas");
        pt.insert("Toggle layers, cuts, and settings panel", "Mostrar/Ocultar painel de camadas, cortes e configurações");
        pt.insert("Toggle console output panel", "Mostrar/Ocultar painel de saída do console");
        pt.insert("Drawing Tools", "Ferramentas de Desenho");
        pt.insert("Job Transformation", "Transformação Job");
        pt.insert("Z-Probe", "Sonda Z");
        pt.insert("View", "Visualizar");
        pt.insert("Theme", "Tema");
        pt.insert("Layout", "Layout");
        pt.insert("Language", "Idioma");
        pt.insert("Controller", "Controlador");
        pt.insert("Modern (recommended)", "Moderno (recomendado)");
        pt.insert("Pro (new)", "Pro (novo)");
        pt.insert("Industrial (advanced)", "Industrial (avançado)");
        pt.insert("Modern layout (simple)", "Layout moderno (simples)");
        pt.insert("Classic layout (expert)", "Layout clássico (especialista)");
        pt.insert(
            "Pro layout (aesthetic & practical)",
            "Layout Pro (estético e prático)",
        );
        pt.insert("Beginner Mode", "Modo iniciante");
        pt.insert("Connection & Control", "Conexão e Controle");
        pt.insert("Job Preparation", "Preparação do trabalho");
        pt.insert("Creation & Editing", "Criação e edição");
        pt.insert("Advanced Tools", "Ferramentas avançadas");
        pt.insert(
        "Beginner mode active: interface simplified. Disable it in View to show all tools.",
        "Modo iniciante ativo: interface simplificada. Desative em Visualizar para mostrar todas as ferramentas.",
    );
        pt.insert("Cuts", "Cortes");
        pt.insert("Move", "Mover");
        pt.insert("Laser", "Laser");
        pt.insert("Layers", "Camadas");
        pt.insert("Notes", "Notas");
        pt.insert("Project Notes", "Notas do projeto");
        pt.insert("Measure", "Medir");
        pt.insert("Group", "Agrupar");
        pt.insert("Ungroup", "Desagrupar");
        pt.insert("Copy", "Copiar");
        pt.insert("Cut", "Recortar");
        pt.insert("Paste", "Colar");
        pt.insert("Duplicate", "Duplicar");
        pt.insert("Select All", "Selecionar tudo");
        pt.insert("Air Assist", "Assistência de ar");
        pt.insert("Exhaust Fan", "Ventilador de exaustão");
        pt.insert("Power Ramping", "Rampa de potência");
        pt.insert("Perforation", "Perfuração");
        pt.insert("Construction Layer", "Camada de construção");
        pt.insert("Maintenance", "Manutenção");
        pt.insert("Cost Estimate", "Estimativa de custo");
        pt.insert("Export SVG", "Exportar SVG");
        pt.insert("Startup Wizard", "Assistente de início");
        pt.insert("File", "Arquivo");
        pt.insert("Edit", "Editar");
        pt.insert("Undo", "Desfazer");
        pt.insert("Redo", "Refazer");
        pt.insert("Zoom In", "Ampliar");
        pt.insert("Zoom Out", "Reduzir");
        pt.insert("Recent Files", "Arquivos recentes");
        pt.insert("No recent files", "Nenhum arquivo recente");
        pt.insert("Project", "Projeto");
        pt.insert("Open Project", "Abrir projeto");
        pt.insert("Save Project", "Salvar projeto");
        pt.insert("Export Job Report", "Exportar relatório");
        pt.insert("Frame", "Enquadramento");
        pt.insert("Dry Run", "Teste a seco");
        pt.insert("Set Zero", "Definir zero");
        pt.insert("Zero", "Zero");
        pt.insert("Tools", "Ferramentas");
        pt.insert("Power/Speed Test", "Teste potência/velocidade");
        pt.insert("Test Fire", "Disparo de teste");
        pt.insert("GCode Editor", "Editor GCode");
        pt.insert("Tiling", "Ladrilhamento");
        pt.insert("Auto Nesting", "Encaixe automático");
        pt.insert("Job Queue", "Fila de trabalhos");
        pt.insert("Shortcuts", "Atalhos");
        pt.insert("Dark UI", "Interface escura");
        pt.insert("Light UI", "Interface clara");
        pt.insert("Save Layer Template", "Salvar modelo de camada");
        pt.insert("Load Layer Template", "Carregar modelo de camada");
        pt.insert("Help", "Ajuda");
        pt.insert("About", "Sobre");
        pt.insert("Jog Control", "Controle de movimento");
        pt.insert("Step:", "Passo:");
        pt.insert("Feed:", "Avanço:");
        pt.insert("Rapids", "Rápidos");
        pt.insert("Fill", "Preenchimento");
        pt.insert("Risk", "Risco");
        pt.insert("Realistic", "Realista");
        pt.insert("Simulation", "Simulação");
        pt.insert("Zoom in", "Ampliar");
        pt.insert("Zoom out", "Reduzir");
        pt.insert("Fit", "Ajustar");
        pt.insert("Quick Move (Bounds)", "Movimento rápido (Limites)");
        pt.insert("Spindle:", "Fuso:");
        pt.insert("Macros", "Macros");
        pt.insert("New Macro", "Nova macro");
        pt.insert("Delete", "Excluir");
        pt.insert("Pending Queue", "Fila pendente");
        pt.insert("No queued jobs.", "Nenhum trabalho na fila.");
        pt.insert("Execution History", "Histórico de execução");
        pt.insert("No history yet.", "Sem histórico.");
        pt.insert("Align:", "Alinhar:");
        pt.insert("Launch Anyway", "Lançar mesmo assim");
        pt.insert("Cannot launch job with critical errors.", "Não é possível iniciar o trabalho com erros críticos.");
        pt.insert("Key", "Tecla");
        pt.insert("Action", "Ação");
        pt.insert("Object Generators", "Geradores de objetos");
        pt.insert("QR Code Generator", "Gerador QR Code");
        pt.insert("Add Shape", "Adicionar forma");
        pt.insert("Clear", "Limpar");
        pt.insert("Cancel", "Cancelar");
        pt.insert("Apply", "Aplicar");
        pt.insert("Close", "Fechar");
        m.insert(Language::Portuguese, pt);

        // Arabic
        let mut ar = HashMap::new();
        ar.insert("Connect", "اتصال");
        ar.insert("Disconnect", "قطع الاتصال");
        ar.insert("Open", "فتح");
        ar.insert("Save", "حفظ");
        ar.insert("Run", "تشغيل");
        ar.insert("Stop", "إيقاف");
        ar.insert("Hold", "تعليق");
        ar.insert("Resume", "استئناف");
        ar.insert("Home", "الصفحة الرئيسية");
        ar.insert("Unlock", "فك القفل");
        ar.insert("Reset", "إعادة تعيين");
        ar.insert("Settings", "الإعدادات");
        ar.insert("Machine Profile", "ملف الآلة");
        ar.insert("Material Library", "مكتبة المواد");
        ar.insert("Preview", "معاينة");
        ar.insert("Console", "وحدة التحكم");
        ar.insert("Left Panel", "اللوحة اليسرى");
        ar.insert("Right Panel", "اللوحة اليمنى");
        ar.insert("Toggle drawing tools and shapes panel", "إظهار/إخفاء لوحة أدوات الرسم والأشكال");
        ar.insert("Toggle layers, cuts, and settings panel", "إظهار/إخفاء لوحة الطبقات والقطع والإعدادات");
        ar.insert("Toggle console output panel", "إظهار/إخفاء لوحة إخراج وحدة التحكم");
        ar.insert("Drawing Tools", "أدوات الرسم");
        ar.insert("Job Transformation", "تحويل العمل");
        ar.insert("Z-Probe", "مسبار Z");
        ar.insert("View", "عرض");
        ar.insert("Theme", "السمة");
        ar.insert("Layout", "التخطيط");
        ar.insert("Language", "اللغة");
        ar.insert("Controller", "المتحكم");
        ar.insert("Modern (recommended)", "حديث (موصى به)");
        ar.insert("Pro (new)", "احترافي (جديد)");
        ar.insert("Industrial (advanced)", "صناعي (متقدم)");
        ar.insert("Modern layout (simple)", "تخطيط حديث (بسيط)");
        ar.insert("Classic layout (expert)", "تخطيط كلاسيكي (خبير)");
        ar.insert(
            "Pro layout (aesthetic & practical)",
            "تخطيط احترافي (جمالي وعملي)",
        );
        ar.insert("Beginner Mode", "وضع المبتدئ");
        ar.insert("Connection & Control", "الاتصال والتحكم");
        ar.insert("Job Preparation", "إعداد المهمة");
        ar.insert("Creation & Editing", "الإنشاء والتحرير");
        ar.insert("Advanced Tools", "أدوات متقدمة");
        ar.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "وضع المبتدئ مفعّل: الواجهة مبسطة. عطّله من عرض لإظهار جميع الأدوات.",
        );
        ar.insert("Cuts", "القطع");
        ar.insert("Move", "نقل");
        ar.insert("Laser", "ليزر");
        ar.insert("Layers", "الطبقات");
        ar.insert("Notes", "ملاحظات");
        ar.insert("Project Notes", "ملاحظات المشروع");
        ar.insert("Measure", "قياس");
        ar.insert("Group", "تجميع");
        ar.insert("Ungroup", "إلغاء التجميع");
        ar.insert("Copy", "نسخ");
        ar.insert("Cut", "قص");
        ar.insert("Paste", "لصق");
        ar.insert("Duplicate", "تكرار");
        ar.insert("Select All", "تحديد الكل");
        ar.insert("Air Assist", "مساعد الهواء");
        ar.insert("Exhaust Fan", "مروحة العادم");
        ar.insert("Power Ramping", "تدرج الطاقة");
        ar.insert("Perforation", "تثقيب");
        ar.insert("Construction Layer", "طبقة البناء");
        ar.insert("Maintenance", "الصيانة");
        ar.insert("Cost Estimate", "تقدير التكلفة");
        ar.insert("Export SVG", "تصدير SVG");
        ar.insert("Startup Wizard", "معالج البدء");
        ar.insert("File", "ملف");
        ar.insert("Edit", "تحرير");
        ar.insert("Undo", "تراجع");
        ar.insert("Redo", "إعادة");
        ar.insert("Zoom In", "تكبير");
        ar.insert("Zoom Out", "تصغير");
        ar.insert("Recent Files", "الملفات الأخيرة");
        ar.insert("No recent files", "لا توجد ملفات حديثة");
        ar.insert("Project", "مشروع");
        ar.insert("Open Project", "فتح مشروع");
        ar.insert("Save Project", "حفظ مشروع");
        ar.insert("Export Job Report", "تصدير تقرير العمل");
        ar.insert("Frame", "إطار");
        ar.insert("Dry Run", "تشغيل تجريبي");
        ar.insert("Set Zero", "تعيين الصفر");
        ar.insert("Zero", "صفر");
        ar.insert("Tools", "أدوات");
        ar.insert("Power/Speed Test", "اختبار القوة/السرعة");
        ar.insert("Test Fire", "إطلاق تجريبي");
        ar.insert("GCode Editor", "محرر GCode");
        ar.insert("Tiling", "تبليط");
        ar.insert("Auto Nesting", "تداخل تلقائي");
        ar.insert("Job Queue", "قائمة الانتظار");
        ar.insert("Shortcuts", "اختصارات");
        ar.insert("Dark UI", "واجهة داكنة");
        ar.insert("Light UI", "واجهة فاتحة");
        ar.insert("Save Layer Template", "حفظ قالب الطبقة");
        ar.insert("Load Layer Template", "تحميل قالب الطبقة");
        ar.insert("Help", "مساعدة");
        ar.insert("About", "حول");
        ar.insert("Jog Control", "التحكم بالحركة");
        ar.insert("Step:", "خطوة:");
        ar.insert("Feed:", "تغذية:");
        ar.insert("Rapids", "سريع");
        ar.insert("Fill", "ملء");
        ar.insert("Risk", "خطر");
        ar.insert("Realistic", "واقعي");
        ar.insert("Simulation", "محاكاة");
        ar.insert("Zoom in", "تكبير");
        ar.insert("Zoom out", "تصغير");
        ar.insert("Fit", "ملاءمة");
        ar.insert("Quick Move (Bounds)", "حركة سريعة (حدود)");
        ar.insert("Spindle:", "المغزل:");
        ar.insert("Macros", "ماكرو");
        ar.insert("New Macro", "ماكرو جديد");
        ar.insert("Delete", "حذف");
        ar.insert("Pending Queue", "قائمة الانتظار");
        ar.insert("No queued jobs.", "لا توجد مهام في الانتظار.");
        ar.insert("Execution History", "سجل التنفيذ");
        ar.insert("No history yet.", "لا يوجد سجل بعد.");
        ar.insert("Align:", "محاذاة:");
        ar.insert("Launch Anyway", "تشغيل على أي حال");
        ar.insert("Cannot launch job with critical errors.", "لا يمكن تشغيل المهمة مع أخطاء حرجة.");
        ar.insert("Key", "مفتاح");
        ar.insert("Action", "إجراء");
        ar.insert("Add Shape", "إضافة شكل");
        ar.insert("Clear", "مسح");
        ar.insert("Cancel", "إلغاء");
        ar.insert("Apply", "تطبيق");
        ar.insert("Close", "إغلاق");
        m.insert(Language::Arabic, ar);

        // Chinese
        let mut zh = HashMap::new();
        zh.insert("Connect", "连接");
        zh.insert("Disconnect", "断开");
        zh.insert("Open", "打开");
        zh.insert("Save", "保存");
        zh.insert("Run", "运行");
        zh.insert("Stop", "停止");
        zh.insert("Hold", "暂停");
        zh.insert("Resume", "继续");
        zh.insert("Home", "回零");
        zh.insert("Unlock", "解锁");
        zh.insert("Reset", "重置");
        zh.insert("Settings", "设置");
        zh.insert("Machine Profile", "机器配置");
        zh.insert("Material Library", "材料库");
        zh.insert("Preview", "预览");
        zh.insert("Console", "控制台");
        zh.insert("Left Panel", "左面板");
        zh.insert("Right Panel", "右面板");
        zh.insert("Toggle drawing tools and shapes panel", "显示/隐藏绘图工具和形状面板");
        zh.insert("Toggle layers, cuts, and settings panel", "显示/隐藏图层、切割和设置面板");
        zh.insert("Toggle console output panel", "显示/隐藏控制台输出面板");
        zh.insert("Drawing Tools", "绘图工具");
        zh.insert("Job Transformation", "作业变换");
        zh.insert("Z-Probe", "Z探针");
        zh.insert("View", "视图");
        zh.insert("Theme", "主题");
        zh.insert("Layout", "布局");
        zh.insert("Language", "语言");
        zh.insert("Controller", "控制器");
        zh.insert("Cuts", "切割");
        zh.insert("Move", "移动");
        zh.insert("Laser", "激光");
        zh.insert("Layers", "图层");
        zh.insert("Notes", "备注");
        zh.insert("Project Notes", "项目备注");
        zh.insert("Measure", "测量");
        zh.insert("Group", "编组");
        zh.insert("Ungroup", "取消编组");
        zh.insert("Copy", "复制");
        zh.insert("Cut", "剪切");
        zh.insert("Paste", "粘贴");
        zh.insert("Duplicate", "复制一份");
        zh.insert("Select All", "全选");
        zh.insert("Air Assist", "气辅");
        zh.insert("Exhaust Fan", "排风扇");
        zh.insert("Power Ramping", "功率渐变");
        zh.insert("Perforation", "穿孔");
        zh.insert("Construction Layer", "辅助图层");
        zh.insert("Maintenance", "维护");
        zh.insert("Cost Estimate", "成本估算");
        zh.insert("Export SVG", "导出SVG");
        zh.insert("Startup Wizard", "启动向导");
        zh.insert("File", "文件");
        zh.insert("Edit", "编辑");
        zh.insert("Undo", "撤销");
        zh.insert("Redo", "重做");
        zh.insert("Zoom In", "放大");
        zh.insert("Zoom Out", "缩小");
        zh.insert("Recent Files", "最近文件");
        zh.insert("No recent files", "没有最近文件");
        zh.insert("Project", "项目");
        zh.insert("Open Project", "打开项目");
        zh.insert("Save Project", "保存项目");
        zh.insert("Export Job Report", "导出作业报告");
        zh.insert("Frame", "框架");
        zh.insert("Dry Run", "空运行");
        zh.insert("Set Zero", "设置零点");
        zh.insert("Zero", "归零");
        zh.insert("Tools", "工具");
        zh.insert("Power/Speed Test", "功率/速度测试");
        zh.insert("Test Fire", "测试发射");
        zh.insert("GCode Editor", "GCode编辑器");
        zh.insert("Tiling", "平铺");
        zh.insert("Auto Nesting", "自动排版");
        zh.insert("Job Queue", "作业队列");
        zh.insert("Shortcuts", "快捷键");
        zh.insert("Dark UI", "深色界面");
        zh.insert("Light UI", "浅色界面");
        zh.insert("Save Layer Template", "保存图层模板");
        zh.insert("Load Layer Template", "加载图层模板");
        zh.insert("Help", "帮助");
        zh.insert("About", "关于");
        zh.insert("Jog Control", "点动控制");
        zh.insert("Step:", "步长:");
        zh.insert("Feed:", "进给:");
        zh.insert("Rapids", "快速移动");
        zh.insert("Fill", "填充");
        zh.insert("Risk", "风险");
        zh.insert("Realistic", "逼真");
        zh.insert("Simulation", "仿真");
        zh.insert("Zoom in", "放大");
        zh.insert("Zoom out", "缩小");
        zh.insert("Fit", "适配");
        zh.insert("Quick Move (Bounds)", "快速移动（边界）");
        zh.insert("Spindle:", "主轴:");
        zh.insert("Macros", "宏");
        zh.insert("New Macro", "新建宏");
        zh.insert("Delete", "删除");
        zh.insert("Pending Queue", "等待队列");
        zh.insert("No queued jobs.", "队列中没有作业。");
        zh.insert("Execution History", "执行历史");
        zh.insert("No history yet.", "暂无历史。");
        zh.insert("Align:", "对齐:");
        zh.insert("Launch Anyway", "强制启动");
        zh.insert("Cannot launch job with critical errors.", "存在严重错误，无法启动作业。");
        zh.insert("Key", "键");
        zh.insert("Action", "动作");
        zh.insert("Add Shape", "添加形状");
        zh.insert("Clear", "清除");
        zh.insert("Cancel", "取消");
        zh.insert("Apply", "应用");
        zh.insert("Close", "关闭");
        zh.insert("Modern (recommended)", "现代（推荐）");
        zh.insert("Pro (new)", "专业 (新)");
        zh.insert("Industrial (advanced)", "工业（高级）");
        zh.insert("Modern layout (simple)", "现代布局（简单）");
        zh.insert("Classic layout (expert)", "经典布局（专家）");
        zh.insert(
            "Pro layout (aesthetic & practical)",
            "专业布局 (美观且实用)",
        );
        zh.insert("Beginner Mode", "新手模式");
        zh.insert("Connection & Control", "连接与控制");
        zh.insert("Job Preparation", "作业准备");
        zh.insert("Creation & Editing", "创建与编辑");
        zh.insert("Advanced Tools", "高级工具");
        zh.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "新手模式已启用：界面已简化。在视图中禁用以显示所有工具。",
        );
        m.insert(Language::Chinese, zh);

        // Russian
        let mut ru = HashMap::new();
        ru.insert("Connect", "Подключить");
        ru.insert("Disconnect", "Отключить");
        ru.insert("Open", "Открыть");
        ru.insert("Save", "Сохранить");
        ru.insert("Run", "Запуск");
        ru.insert("Stop", "Стоп");
        ru.insert("Hold", "Пауза");
        ru.insert("Resume", "Продолжить");
        ru.insert("Home", "Домой");
        ru.insert("Unlock", "Разблокировать");
        ru.insert("Reset", "Сброс");
        ru.insert("Settings", "Настройки");
        ru.insert("Machine Profile", "Профиль станка");
        ru.insert("Material Library", "Библиотека материалов");
        ru.insert("Preview", "Предпросмотр");
        ru.insert("Console", "Консоль");
        ru.insert("Left Panel", "Левая панель");
        ru.insert("Right Panel", "Правая панель");
        ru.insert("Toggle drawing tools and shapes panel", "Показать/Скрыть панель инструментов рисования и фигур");
        ru.insert("Toggle layers, cuts, and settings panel", "Показать/Скрыть панель слоев, резов и настроек");
        ru.insert("Toggle console output panel", "Показать/Скрыть панель вывода консоли");
        ru.insert("Drawing Tools", "Инструменты рисования");
        ru.insert("Job Transformation", "Трансформация задания");
        ru.insert("Z-Probe", "Z-датчик");
        ru.insert("View", "Вид");
        ru.insert("Theme", "Тема");
        ru.insert("Layout", "Макет");
        ru.insert("Language", "Язык");
        ru.insert("Controller", "Контроллер");
        ru.insert("Cuts", "Резка");
        ru.insert("Move", "Перемещение");
        ru.insert("Laser", "Лазер");
        ru.insert("Layers", "Слои");
        ru.insert("Notes", "Заметки");
        ru.insert("Project Notes", "Заметки проекта");
        ru.insert("Measure", "Измерение");
        ru.insert("Group", "Группировать");
        ru.insert("Ungroup", "Разгруппировать");
        ru.insert("Copy", "Копировать");
        ru.insert("Cut", "Вырезать");
        ru.insert("Paste", "Вставить");
        ru.insert("Duplicate", "Дублировать");
        ru.insert("Select All", "Выделить всё");
        ru.insert("Air Assist", "Воздушная помощь");
        ru.insert("Exhaust Fan", "Вытяжной вентилятор");
        ru.insert("Power Ramping", "Плавное изменение мощности");
        ru.insert("Perforation", "Перфорация");
        ru.insert("Construction Layer", "Конструкционный слой");
        ru.insert("Maintenance", "Обслуживание");
        ru.insert("Cost Estimate", "Оценка стоимости");
        ru.insert("Export SVG", "Экспорт SVG");
        ru.insert("Startup Wizard", "Мастер настройки");
        ru.insert("Modern (recommended)", "Современный (рекомендуется)");
        ru.insert("Pro (new)", "Про (новый)");
        ru.insert("Industrial (advanced)", "Промышленный (продвинутый)");
        ru.insert("Modern layout (simple)", "Современный макет (простой)");
        ru.insert("Classic layout (expert)", "Классический макет (эксперт)");
        ru.insert(
            "Pro layout (aesthetic & practical)",
            "Про макет (эстетичный и практичный)",
        );
        ru.insert("Beginner Mode", "Режим новичка");
        ru.insert("Connection & Control", "Подключение и управление");
        ru.insert("Job Preparation", "Подготовка задания");
        ru.insert("Creation & Editing", "Создание и редактирование");
        ru.insert("Advanced Tools", "Расширенные инструменты");
        ru.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "Режим новичка активен: интерфейс упрощён. Отключите в меню Вид для отображения всех инструментов.");
        ru.insert("File", "Файл");
        ru.insert("Edit", "Редактировать");
        ru.insert("Undo", "Отменить");
        ru.insert("Redo", "Повторить");
        ru.insert("Zoom In", "Увеличить");
        ru.insert("Zoom Out", "Уменьшить");
        ru.insert("Recent Files", "Недавние файлы");
        ru.insert("No recent files", "Нет недавних файлов");
        ru.insert("Project", "Проект");
        ru.insert("Open Project", "Открыть проект");
        ru.insert("Save Project", "Сохранить проект");
        ru.insert("Export Job Report", "Экспорт отчёта");
        ru.insert("Frame", "Рамка");
        ru.insert("Dry Run", "Пробный запуск");
        ru.insert("Set Zero", "Установить ноль");
        ru.insert("Zero", "Ноль");
        ru.insert("Tools", "Инструменты");
        ru.insert("Power/Speed Test", "Тест мощности/скорости");
        ru.insert("Test Fire", "Пробный выстрел");
        ru.insert("GCode Editor", "Редактор GCode");
        ru.insert("Tiling", "Мозаика");
        ru.insert("Auto Nesting", "Автораскрой");
        ru.insert("Job Queue", "Очередь заданий");
        ru.insert("Shortcuts", "Горячие клавиши");
        ru.insert("Dark UI", "Тёмный интерфейс");
        ru.insert("Light UI", "Светлый интерфейс");
        ru.insert("Save Layer Template", "Сохранить шаблон слоя");
        ru.insert("Load Layer Template", "Загрузить шаблон слоя");
        ru.insert("Help", "Справка");
        ru.insert("About", "О программе");
        ru.insert("Jog Control", "Ручное управление");
        ru.insert("Step:", "Шаг:");
        ru.insert("Feed:", "Подача:");
        ru.insert("Rapids", "Быстрые ходы");
        ru.insert("Fill", "Заливка");
        ru.insert("Risk", "Риск");
        ru.insert("Realistic", "Реалистичный");
        ru.insert("Simulation", "Симуляция");
        ru.insert("Zoom in", "Увеличить");
        ru.insert("Zoom out", "Уменьшить");
        ru.insert("Fit", "Вписать");
        ru.insert("Quick Move (Bounds)", "Быстрое перемещение (Границы)");
        ru.insert("Spindle:", "Шпиндель:");
        ru.insert("Macros", "Макросы");
        ru.insert("New Macro", "Новый макрос");
        ru.insert("Delete", "Удалить");
        ru.insert("Pending Queue", "Очередь ожидания");
        ru.insert("No queued jobs.", "Нет заданий в очереди.");
        ru.insert("Execution History", "История выполнения");
        ru.insert("No history yet.", "Истории пока нет.");
        ru.insert("Align:", "Выровнять:");
        ru.insert("Launch Anyway", "Запустить всё равно");
        ru.insert("Cannot launch job with critical errors.", "Невозможно запустить задание с критическими ошибками.");
        ru.insert("Key", "Клавиша");
        ru.insert("Action", "Действие");
        ru.insert("Add Shape", "Добавить фигуру");
        ru.insert("Clear", "Очистить");
        ru.insert("Cancel", "Отмена");
        ru.insert("Apply", "Применить");
        ru.insert("Close", "Закрыть");
        m.insert(Language::Russian, ru);

        // Turkish
        let mut tr_lang = HashMap::new();
        tr_lang.insert("Connect", "Bağlan");
        tr_lang.insert("Disconnect", "Bağlantıyı Kes");
        tr_lang.insert("Open", "Aç");
        tr_lang.insert("Save", "Kaydet");
        tr_lang.insert("Run", "Çalıştır");
        tr_lang.insert("Stop", "Durdur");
        tr_lang.insert("Hold", "Beklet");
        tr_lang.insert("Resume", "Devam Et");
        tr_lang.insert("Home", "Ana Konum");
        tr_lang.insert("Unlock", "Kilidi Aç");
        tr_lang.insert("Reset", "Sıfırla");
        tr_lang.insert("Settings", "Ayarlar");
        tr_lang.insert("Machine Profile", "Makine Profili");
        tr_lang.insert("Material Library", "Malzeme Kütüphanesi");
        tr_lang.insert("Preview", "Önizleme");
        tr_lang.insert("Console", "Konsol");
        tr_lang.insert("Left Panel", "Sol Panel");
        tr_lang.insert("Right Panel", "Sağ Panel");
        tr_lang.insert("Toggle drawing tools and shapes panel", "Çizim araçları ve şekiller panelini göster/gizle");
        tr_lang.insert("Toggle layers, cuts, and settings panel", "Katmanlar, kesimler ve ayarlar panelini göster/gizle");
        tr_lang.insert("Toggle console output panel", "Konsol çıktı panelini göster/gizle");
        tr_lang.insert("Drawing Tools", "Çizim Araçları");
        tr_lang.insert("Job Transformation", "İş Dönüşümü");
        tr_lang.insert("Z-Probe", "Z-Prob");
        tr_lang.insert("View", "Görünüm");
        tr_lang.insert("Theme", "Tema");
        tr_lang.insert("Layout", "Düzen");
        tr_lang.insert("Language", "Dil");
        tr_lang.insert("Controller", "Denetleyici");
        tr_lang.insert("Cuts", "Kesimler");
        tr_lang.insert("Move", "Taşı");
        tr_lang.insert("Laser", "Lazer");
        tr_lang.insert("Layers", "Katmanlar");
        tr_lang.insert("Notes", "Notlar");
        tr_lang.insert("Project Notes", "Proje Notları");
        tr_lang.insert("Measure", "Ölçüm");
        tr_lang.insert("Group", "Grupla");
        tr_lang.insert("Ungroup", "Grubu Çöz");
        tr_lang.insert("Copy", "Kopyala");
        tr_lang.insert("Cut", "Kes");
        tr_lang.insert("Paste", "Yapıştır");
        tr_lang.insert("Duplicate", "Çoğalt");
        tr_lang.insert("Select All", "Tümünü Seç");
        tr_lang.insert("Air Assist", "Hava Yardımı");
        tr_lang.insert("Exhaust Fan", "Havalandırma Fanı");
        tr_lang.insert("Power Ramping", "Güç Rampalama");
        tr_lang.insert("Perforation", "Delik Açma");
        tr_lang.insert("Construction Layer", "Yapı Katmanı");
        tr_lang.insert("Maintenance", "Bakım");
        tr_lang.insert("Cost Estimate", "Maliyet Tahmini");
        tr_lang.insert("Export SVG", "SVG Dışa Aktar");
        tr_lang.insert("Startup Wizard", "Başlangıç Sihirbazı");
        tr_lang.insert("Modern (recommended)", "Modern (önerilen)");
        tr_lang.insert("Pro (new)", "Pro (yeni)");
        tr_lang.insert("Industrial (advanced)", "Endüstriyel (gelişmiş)");
        tr_lang.insert("Modern layout (simple)", "Modern düzen (basit)");
        tr_lang.insert("Classic layout (expert)", "Klasik düzen (uzman)");
        tr_lang.insert(
            "Pro layout (aesthetic & practical)",
            "Pro düzen (estetik ve pratik)",
        );
        tr_lang.insert("Beginner Mode", "Başlangıç Modu");
        tr_lang.insert("Connection & Control", "Bağlantı ve Kontrol");
        tr_lang.insert("Job Preparation", "İş Hazırlığı");
        tr_lang.insert("Creation & Editing", "Oluşturma ve Düzenleme");
        tr_lang.insert("Advanced Tools", "Gelişmiş Araçlar");
        tr_lang.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "Başlangıç modu etkin: arayüz basitleştirildi. Tüm araçları görmek için Görünüm'de devre dışı bırakın.");
        tr_lang.insert("File", "Dosya");
        tr_lang.insert("Edit", "Düzenle");
        tr_lang.insert("Undo", "Geri Al");
        tr_lang.insert("Redo", "Yinele");
        tr_lang.insert("Zoom In", "Yakınlaştır");
        tr_lang.insert("Zoom Out", "Uzaklaştır");
        tr_lang.insert("Recent Files", "Son Dosyalar");
        tr_lang.insert("No recent files", "Son dosya yok");
        tr_lang.insert("Project", "Proje");
        tr_lang.insert("Open Project", "Proje Aç");
        tr_lang.insert("Save Project", "Projeyi Kaydet");
        tr_lang.insert("Export Job Report", "İş Raporu Dışa Aktar");
        tr_lang.insert("Frame", "Çerçeve");
        tr_lang.insert("Dry Run", "Kuru Çalışma");
        tr_lang.insert("Set Zero", "Sıfır Ayarla");
        tr_lang.insert("Zero", "Sıfır");
        tr_lang.insert("Tools", "Araçlar");
        tr_lang.insert("Power/Speed Test", "Güç/Hız Testi");
        tr_lang.insert("Test Fire", "Test Atışı");
        tr_lang.insert("GCode Editor", "GCode Editörü");
        tr_lang.insert("Tiling", "Döşeme");
        tr_lang.insert("Auto Nesting", "Otomatik Yerleşim");
        tr_lang.insert("Job Queue", "İş Kuyruğu");
        tr_lang.insert("Shortcuts", "Kısayollar");
        tr_lang.insert("Dark UI", "Koyu Arayüz");
        tr_lang.insert("Light UI", "Açık Arayüz");
        tr_lang.insert("Save Layer Template", "Katman Şablonu Kaydet");
        tr_lang.insert("Load Layer Template", "Katman Şablonu Yükle");
        tr_lang.insert("Help", "Yardım");
        tr_lang.insert("About", "Hakkında");
        tr_lang.insert("Jog Control", "Jog Kontrolü");
        tr_lang.insert("Step:", "Adım:");
        tr_lang.insert("Feed:", "Besleme:");
        tr_lang.insert("Rapids", "Hızlı");
        tr_lang.insert("Fill", "Dolgu");
        tr_lang.insert("Risk", "Risk");
        tr_lang.insert("Realistic", "Gerçekçi");
        tr_lang.insert("Simulation", "Simülasyon");
        tr_lang.insert("Zoom in", "Yakınlaştır");
        tr_lang.insert("Zoom out", "Uzaklaştır");
        tr_lang.insert("Fit", "Sığdır");
        tr_lang.insert("Quick Move (Bounds)", "Hızlı Hareket (Sınırlar)");
        tr_lang.insert("Spindle:", "İş Mili:");
        tr_lang.insert("Macros", "Makrolar");
        tr_lang.insert("New Macro", "Yeni Makro");
        tr_lang.insert("Delete", "Sil");
        tr_lang.insert("Pending Queue", "Bekleyen Kuyruk");
        tr_lang.insert("No queued jobs.", "Kuyrukta iş yok.");
        tr_lang.insert("Execution History", "Yürütme Geçmişi");
        tr_lang.insert("No history yet.", "Henüz geçmiş yok.");
        tr_lang.insert("Align:", "Hizala:");
        tr_lang.insert("Launch Anyway", "Yine de Başlat");
        tr_lang.insert("Cannot launch job with critical errors.", "Kritik hatalarla iş başlatılamaz.");
        tr_lang.insert("Key", "Tuş");
        tr_lang.insert("Action", "Eylem");
        tr_lang.insert("Add Shape", "Şekil Ekle");
        tr_lang.insert("Clear", "Temizle");
        tr_lang.insert("Cancel", "İptal");
        tr_lang.insert("Apply", "Uygula");
        tr_lang.insert("Close", "Kapat");
        m.insert(Language::Turkish, tr_lang);

        // Korean
        let mut ko = HashMap::new();
        ko.insert("Connect", "연결");
        ko.insert("Disconnect", "연결 해제");
        ko.insert("Open", "열기");
        ko.insert("Save", "저장");
        ko.insert("Run", "실행");
        ko.insert("Stop", "정지");
        ko.insert("Hold", "일시정지");
        ko.insert("Resume", "재개");
        ko.insert("Home", "원점");
        ko.insert("Unlock", "잠금해제");
        ko.insert("Reset", "초기화");
        ko.insert("Settings", "설정");
        ko.insert("Machine Profile", "기계 프로필");
        ko.insert("Material Library", "재료 라이브러리");
        ko.insert("Preview", "미리보기");
        ko.insert("Console", "콘솔");
        ko.insert("Left Panel", "왼쪽 패널");
        ko.insert("Right Panel", "오른쪽 패널");
        ko.insert("Toggle drawing tools and shapes panel", "그리기 도구 및 도형 패널 표시/숨기기");
        ko.insert("Toggle layers, cuts, and settings panel", "레이어, 컷 및 설정 패널 표시/숨기기");
        ko.insert("Toggle console output panel", "콘솔 출력 패널 표시/숨기기");
        ko.insert("Drawing Tools", "그리기 도구");
        ko.insert("Job Transformation", "작업 변환");
        ko.insert("Z-Probe", "Z-프로브");
        ko.insert("View", "보기");
        ko.insert("Theme", "테마");
        ko.insert("Layout", "레이아웃");
        ko.insert("Language", "언어");
        ko.insert("Controller", "컨트롤러");
        ko.insert("Cuts", "절단");
        ko.insert("Move", "이동");
        ko.insert("Laser", "레이저");
        ko.insert("Layers", "레이어");
        ko.insert("Notes", "메모");
        ko.insert("Project Notes", "프로젝트 메모");
        ko.insert("Measure", "측정");
        ko.insert("Group", "그룹화");
        ko.insert("Ungroup", "그룹해제");
        ko.insert("Copy", "복사");
        ko.insert("Cut", "잘라내기");
        ko.insert("Paste", "붙여넣기");
        ko.insert("Duplicate", "복제");
        ko.insert("Select All", "모두 선택");
        ko.insert("Air Assist", "에어 어시스트");
        ko.insert("Exhaust Fan", "배기 팬");
        ko.insert("Power Ramping", "출력 램핑");
        ko.insert("Perforation", "천공");
        ko.insert("Construction Layer", "보조 레이어");
        ko.insert("Maintenance", "유지보수");
        ko.insert("Cost Estimate", "비용 추정");
        ko.insert("Export SVG", "SVG 내보내기");
        ko.insert("Startup Wizard", "시작 마법사");
        ko.insert("Modern (recommended)", "현대적 (권장)");
        ko.insert("Pro (new)", "프로 (신규)");
        ko.insert("Industrial (advanced)", "산업용 (고급)");
        ko.insert("Modern layout (simple)", "현대적 레이아웃 (간단)");
        ko.insert("Classic layout (expert)", "클래식 레이아웃 (전문가)");
        ko.insert(
            "Pro layout (aesthetic & practical)",
            "프로 레이아웃 (미적이고 실용적)",
        );
        ko.insert("Beginner Mode", "초보자 모드");
        ko.insert("Connection & Control", "연결 및 제어");
        ko.insert("Job Preparation", "작업 준비");
        ko.insert("Creation & Editing", "생성 및 편집");
        ko.insert("Advanced Tools", "고급 도구");
        ko.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "초보자 모드 활성화: 인터페이스가 간소화되었습니다. 모든 도구를 보려면 보기에서 비활성화하세요.");
        ko.insert("File", "파일");
        ko.insert("Edit", "편집");
        ko.insert("Undo", "실행 취소");
        ko.insert("Redo", "다시 실행");
        ko.insert("Zoom In", "확대");
        ko.insert("Zoom Out", "축소");
        ko.insert("Recent Files", "최근 파일");
        ko.insert("No recent files", "최근 파일 없음");
        ko.insert("Project", "프로젝트");
        ko.insert("Open Project", "프로젝트 열기");
        ko.insert("Save Project", "프로젝트 저장");
        ko.insert("Export Job Report", "작업 보고서 내보내기");
        ko.insert("Frame", "프레임");
        ko.insert("Dry Run", "테스트 실행");
        ko.insert("Set Zero", "영점 설정");
        ko.insert("Zero", "영점");
        ko.insert("Tools", "도구");
        ko.insert("Power/Speed Test", "출력/속도 테스트");
        ko.insert("Test Fire", "시험 발사");
        ko.insert("GCode Editor", "GCode 편집기");
        ko.insert("Tiling", "타일링");
        ko.insert("Auto Nesting", "자동 배치");
        ko.insert("Job Queue", "작업 대기열");
        ko.insert("Shortcuts", "단축키");
        ko.insert("Dark UI", "다크 UI");
        ko.insert("Light UI", "라이트 UI");
        ko.insert("Save Layer Template", "레이어 템플릿 저장");
        ko.insert("Load Layer Template", "레이어 템플릿 불러오기");
        ko.insert("Help", "도움말");
        ko.insert("About", "정보");
        ko.insert("Jog Control", "조그 제어");
        ko.insert("Step:", "스텝:");
        ko.insert("Feed:", "이송:");
        ko.insert("Rapids", "급속이동");
        ko.insert("Fill", "채우기");
        ko.insert("Risk", "위험");
        ko.insert("Realistic", "사실적");
        ko.insert("Simulation", "시뮬레이션");
        ko.insert("Zoom in", "확대");
        ko.insert("Zoom out", "축소");
        ko.insert("Fit", "맞춤");
        ko.insert("Quick Move (Bounds)", "빠른 이동 (범위)");
        ko.insert("Spindle:", "스핀들:");
        ko.insert("Macros", "매크로");
        ko.insert("New Macro", "새 매크로");
        ko.insert("Delete", "삭제");
        ko.insert("Pending Queue", "대기 중");
        ko.insert("No queued jobs.", "대기 중인 작업 없음.");
        ko.insert("Execution History", "실행 기록");
        ko.insert("No history yet.", "기록 없음.");
        ko.insert("Align:", "정렬:");
        ko.insert("Launch Anyway", "강제 실행");
        ko.insert("Cannot launch job with critical errors.", "심각한 오류로 작업을 시작할 수 없습니다.");
        ko.insert("Key", "키");
        ko.insert("Action", "동작");
        ko.insert("Add Shape", "도형 추가");
        ko.insert("Clear", "지우기");
        ko.insert("Cancel", "취소");
        ko.insert("Apply", "적용");
        ko.insert("Close", "닫기");
        m.insert(Language::Korean, ko);

        // Polish
        let mut pl = HashMap::new();
        pl.insert("Connect", "Połącz");
        pl.insert("Disconnect", "Rozłącz");
        pl.insert("Open", "Otwórz");
        pl.insert("Save", "Zapisz");
        pl.insert("Run", "Uruchom");
        pl.insert("Stop", "Zatrzymaj");
        pl.insert("Hold", "Wstrzymaj");
        pl.insert("Resume", "Wznów");
        pl.insert("Home", "Pozycja zerowa");
        pl.insert("Unlock", "Odblokuj");
        pl.insert("Reset", "Resetuj");
        pl.insert("Settings", "Ustawienia");
        pl.insert("Machine Profile", "Profil maszyny");
        pl.insert("Material Library", "Biblioteka materiałów");
        pl.insert("Preview", "Podgląd");
        pl.insert("Console", "Konsola");
        pl.insert("Left Panel", "Lewy panel");
        pl.insert("Right Panel", "Prawy panel");
        pl.insert("Toggle drawing tools and shapes panel", "Pokaż/Ukryj panel narzędzi rysowania i kształtów");
        pl.insert("Toggle layers, cuts, and settings panel", "Pokaż/Ukryj panel warstw, cięć i ustawień");
        pl.insert("Toggle console output panel", "Pokaż/Ukryj panel wyjściowy konsoli");
        pl.insert("Drawing Tools", "Narzędzia rysowania");
        pl.insert("Job Transformation", "Transformacja zadania");
        pl.insert("Z-Probe", "Sonda Z");
        pl.insert("View", "Widok");
        pl.insert("Theme", "Motyw");
        pl.insert("Layout", "Układ");
        pl.insert("Language", "Język");
        pl.insert("Controller", "Kontroler");
        pl.insert("Cuts", "Cięcia");
        pl.insert("Move", "Przesuń");
        pl.insert("Laser", "Laser");
        pl.insert("Layers", "Warstwy");
        pl.insert("Notes", "Notatki");
        pl.insert("Project Notes", "Notatki projektu");
        pl.insert("Measure", "Pomiar");
        pl.insert("Group", "Grupuj");
        pl.insert("Ungroup", "Rozgrupuj");
        pl.insert("Copy", "Kopiuj");
        pl.insert("Cut", "Wytnij");
        pl.insert("Paste", "Wklej");
        pl.insert("Duplicate", "Duplikuj");
        pl.insert("Select All", "Zaznacz wszystko");
        pl.insert("Air Assist", "Wspomaganie powietrzem");
        pl.insert("Exhaust Fan", "Wentylator wyciągowy");
        pl.insert("Power Ramping", "Rampa mocy");
        pl.insert("Perforation", "Perforacja");
        pl.insert("Construction Layer", "Warstwa konstrukcyjna");
        pl.insert("Maintenance", "Konserwacja");
        pl.insert("Cost Estimate", "Szacunek kosztów");
        pl.insert("Export SVG", "Eksport SVG");
        pl.insert("Startup Wizard", "Kreator uruchamiania");
        pl.insert("Modern (recommended)", "Nowoczesny (zalecany)");
        pl.insert("Pro (new)", "Pro (nowy)");
        pl.insert("Industrial (advanced)", "Przemysłowy (zaawansowany)");
        pl.insert("Modern layout (simple)", "Nowoczesny układ (prosty)");
        pl.insert("Classic layout (expert)", "Klasyczny układ (ekspert)");
        pl.insert(
            "Pro layout (aesthetic & practical)",
            "Pro układ (estetyczny i praktyczny)",
        );
        pl.insert("Beginner Mode", "Tryb początkującego");
        pl.insert("Connection & Control", "Połączenie i sterowanie");
        pl.insert("Job Preparation", "Przygotowanie zadania");
        pl.insert("Creation & Editing", "Tworzenie i edycja");
        pl.insert("Advanced Tools", "Zaawansowane narzędzia");
        pl.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "Tryb początkującego aktywny: interfejs uproszczony. Wyłącz w Widok, aby zobaczyć wszystkie narzędzia.");
        pl.insert("File", "Plik");
        pl.insert("Edit", "Edycja");
        pl.insert("Undo", "Cofnij");
        pl.insert("Redo", "Ponów");
        pl.insert("Zoom In", "Powiększ");
        pl.insert("Zoom Out", "Pomniejsz");
        pl.insert("Recent Files", "Ostatnie pliki");
        pl.insert("No recent files", "Brak ostatnich plików");
        pl.insert("Project", "Projekt");
        pl.insert("Open Project", "Otwórz projekt");
        pl.insert("Save Project", "Zapisz projekt");
        pl.insert("Export Job Report", "Eksportuj raport");
        pl.insert("Frame", "Ramka");
        pl.insert("Dry Run", "Próba na sucho");
        pl.insert("Set Zero", "Ustaw zero");
        pl.insert("Zero", "Zero");
        pl.insert("Tools", "Narzędzia");
        pl.insert("Power/Speed Test", "Test mocy/prędkości");
        pl.insert("Test Fire", "Strzał testowy");
        pl.insert("GCode Editor", "Edytor GCode");
        pl.insert("Tiling", "Kafelkowanie");
        pl.insert("Auto Nesting", "Automatyczne rozmieszczenie");
        pl.insert("Job Queue", "Kolejka zadań");
        pl.insert("Shortcuts", "Skróty klawiszowe");
        pl.insert("Dark UI", "Ciemny interfejs");
        pl.insert("Light UI", "Jasny interfejs");
        pl.insert("Save Layer Template", "Zapisz szablon warstwy");
        pl.insert("Load Layer Template", "Wczytaj szablon warstwy");
        pl.insert("Help", "Pomoc");
        pl.insert("About", "O programie");
        pl.insert("Jog Control", "Sterowanie ręczne");
        pl.insert("Step:", "Krok:");
        pl.insert("Feed:", "Posuw:");
        pl.insert("Rapids", "Szybkie ruchy");
        pl.insert("Fill", "Wypełnienie");
        pl.insert("Risk", "Ryzyko");
        pl.insert("Realistic", "Realistyczny");
        pl.insert("Simulation", "Symulacja");
        pl.insert("Zoom in", "Powiększ");
        pl.insert("Zoom out", "Pomniejsz");
        pl.insert("Fit", "Dopasuj");
        pl.insert("Quick Move (Bounds)", "Szybki ruch (Granice)");
        pl.insert("Spindle:", "Wrzeciono:");
        pl.insert("Macros", "Makra");
        pl.insert("New Macro", "Nowe makro");
        pl.insert("Delete", "Usuń");
        pl.insert("Pending Queue", "Kolejka oczekujących");
        pl.insert("No queued jobs.", "Brak zadań w kolejce.");
        pl.insert("Execution History", "Historia wykonania");
        pl.insert("No history yet.", "Brak historii.");
        pl.insert("Align:", "Wyrównaj:");
        pl.insert("Launch Anyway", "Uruchom mimo to");
        pl.insert("Cannot launch job with critical errors.", "Nie można uruchomić zadania z krytycznymi błędami.");
        pl.insert("Key", "Klawisz");
        pl.insert("Action", "Akcja");
        pl.insert("Add Shape", "Dodaj kształt");
        pl.insert("Clear", "Wyczyść");
        pl.insert("Cancel", "Anuluj");
        pl.insert("Apply", "Zastosuj");
        pl.insert("Close", "Zamknij");
        m.insert(Language::Polish, pl);

        m
    },
);

static CURRENT_LANG: LazyLock<RwLock<Language>> = LazyLock::new(|| RwLock::new(Language::English));

pub fn set_language(lang: Language) {
    if let Ok(mut l) = CURRENT_LANG.write() {
        *l = lang;
    }
}

pub fn get_language() -> Language {
    if let Ok(l) = CURRENT_LANG.read() {
        *l
    } else {
        Language::English
    }
}

pub fn tr(key: &str) -> String {
    let lang = get_language();
    if lang == Language::English {
        return key.to_string();
    }

    if let Some(map) = DICTIONARY.get(&lang) {
        let map: &HashMap<&'static str, &'static str> = map;
        if let Some(val) = map.get(key) {
            return val.to_string();
        }
    }
    key.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_returns_key_as_is() {
        set_language(Language::English);
        assert_eq!(tr("Connect"), "Connect");
        assert_eq!(tr("NonExistentKey"), "NonExistentKey");
    }

    #[test]
    fn french_translates_known_key() {
        // Need to lock test execution if they rely on global state.
        // A better approach is directly testing the dictionary.
        let map = DICTIONARY.get(&Language::French).unwrap();
        assert_eq!(*map.get("Connect").unwrap(), "Connecter");
        assert_eq!(*map.get("Open").unwrap(), "Ouvrir");
    }

    #[test]
    fn unknown_key_falls_back_to_english() {
        set_language(Language::German);
        assert_eq!(tr("SomeUnknownKey"), "SomeUnknownKey");
        set_language(Language::English);
    }

    #[test]
    fn all_languages_have_dictionary_entries() {
        let languages = [
            Language::French,
            Language::Japanese,
            Language::German,
            Language::Italian,
            Language::Spanish,
            Language::Portuguese,
            Language::Arabic,
            Language::Chinese,
            Language::Russian,
            Language::Turkish,
            Language::Korean,
            Language::Polish,
        ];
        for lang in languages {
            assert!(
                DICTIONARY.contains_key(&lang),
                "Missing dictionary for {:?}",
                lang
            );
            let map = DICTIONARY.get(&lang).unwrap();
            assert!(
                map.contains_key("Connect"),
                "Missing 'Connect' key for {:?}",
                lang
            );
        }
    }

    #[test]
    fn language_name_returns_native_name() {
        assert_eq!(Language::French.name(), "Français");
        assert_eq!(Language::Japanese.name(), "日本語");
        assert_eq!(Language::English.name(), "English");
    }

    #[test]
    fn set_and_get_language_roundtrips() {
        set_language(Language::Spanish);
        assert_eq!(get_language(), Language::Spanish);
        set_language(Language::English);
        assert_eq!(get_language(), Language::English);
    }
}
