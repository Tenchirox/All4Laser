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
        // Node editing & selection
        fr.insert("Node Editing", "Édition de nœuds");
        fr.insert("Selection", "Sélection");
        fr.insert("Create", "Créer");
        fr.insert("Modify", "Modifier");
        // Drawing tools
        fr.insert("Rect", "Rect");
        fr.insert("Circle", "Cercle");
        fr.insert("Origin X:", "Origine X :");
        fr.insert("Radius:", "Rayon :");
        fr.insert("Layer:", "Couche :");
        fr.insert("Set to Active Layer", "Appliquer au calque actif");
        fr.insert("Use the Text Tool panel below to create text paths.", "Utilisez le panneau Outil Texte ci-dessous pour créer des tracés texte.");
        // Text tool
        fr.insert("Text Tool", "Outil Texte");
        fr.insert("Variable Text (Serial Numbers)", "Texte variable (numéros de série)");
        fr.insert("Text:", "Texte :");
        fr.insert("Size:", "Taille :");
        fr.insert("Source:", "Source :");
        fr.insert("Bundled", "Intégrées");
        fr.insert("System", "Système");
        fr.insert("Font:", "Police :");
        fr.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Polices intégrées au projet (SIL OFL 1.1, utilisation compatible GPLv3).");
        fr.insert("Loading font previews...", "Chargement des aperçus de polices…");
        fr.insert("No system fonts detected. Use Bundled or File source.", "Aucune police système détectée. Utilisez les polices intégrées ou un fichier.");
        fr.insert("Load Font File", "Charger un fichier de police");
        fr.insert("Add Text to Drawing", "Ajouter le texte au dessin");
        // Variable text
        fr.insert("Serial", "Série");
        fr.insert("CSV Column", "Colonne CSV");
        fr.insert("Prefix:", "Préfixe :");
        fr.insert("Suffix:", "Suffixe :");
        fr.insert("Start:", "Début :");
        fr.insert("Inc:", "Inc :");
        fr.insert("Pad:", "Rembourrage :");
        fr.insert("Batch Count:", "Nombre de lots :");
        fr.insert("Column:", "Colonne :");
        fr.insert("Header row", "Ligne d'en-tête");
        fr.insert("Delimiter:", "Délimiteur :");
        fr.insert("Load CSV", "Charger CSV");
        // Align / Distribute
        fr.insert("Align / Distribute", "Aligner / Distribuer");
        fr.insert("Align Left", "Aligner à gauche");
        fr.insert("Align Right", "Aligner à droite");
        fr.insert("Align Top", "Aligner en haut");
        fr.insert("Align Bottom", "Aligner en bas");
        fr.insert("Center Horizontal", "Centrer horizontalement");
        fr.insert("Center Vertical", "Centrer verticalement");
        fr.insert("Distribute H", "Distribuer H");
        fr.insert("Distribute V", "Distribuer V");
        // Shape properties
        fr.insert("Shape Properties", "Propriétés de la forme");
        fr.insert("Select a shape to edit properties.", "Sélectionnez une forme pour modifier ses propriétés.");
        // Session recovery
        fr.insert("Session Recovery", "Récupération de session");
        fr.insert("A previous session was interrupted. Restore it?", "Une session précédente a été interrompue. La restaurer ?");
        fr.insert("Restore", "Restaurer");
        fr.insert("Discard", "Ignorer");
        // Preview placeholder
        fr.insert("Load a GCode file or draw shapes to preview", "Chargez un fichier GCode ou dessinez des formes pour prévisualiser");
        // Materials
        fr.insert("Apply Recommended", "Appliquer les recommandés");
        fr.insert("Apply to Active Layer", "Appliquer au calque actif");
        fr.insert("Material Presets", "Préréglages matériaux");
        // Cut list headers
        fr.insert("Mode", "Mode");
        fr.insert("Spd/Pwr", "Vit/Puis");
        fr.insert("Out", "Sort");
        // Misc modify buttons
        fr.insert("Array", "Réseau");
        fr.insert("Grid", "Grille");
        fr.insert("Offset", "Décalage");
        fr.insert("Boolean", "Booléen");
        fr.insert("Circular Array", "Réseau circulaire");
        fr.insert("Grid Array", "Réseau en grille");
        fr.insert("Offset Path", "Décalage du tracé");
        fr.insert("Boolean Operations", "Opérations booléennes");
        // Cut list extra
        fr.insert("View", "Affichage");
        // Font source "File" tab (distinct from menu "File"→"Fichier")
        fr.insert("File", "Fichier");
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
        // New UI keys
        ja.insert("Node Editing", "ノード編集");
        ja.insert("Selection", "選択");
        ja.insert("Create", "作成");
        ja.insert("Modify", "変更");
        ja.insert("Rect", "矩形");
        ja.insert("Circle", "円");
        ja.insert("Origin X:", "原点 X:");
        ja.insert("Radius:", "半径:");
        ja.insert("Layer:", "レイヤー:");
        ja.insert("Set to Active Layer", "アクティブレイヤーに設定");
        ja.insert("Use the Text Tool panel below to create text paths.", "テキストパスを作成するには、下のテキストツールパネルを使用してください。");
        ja.insert("Text Tool", "テキストツール");
        ja.insert("Variable Text (Serial Numbers)", "変数テキスト（シリアル番号）");
        ja.insert("Text:", "テキスト:");
        ja.insert("Size:", "サイズ:");
        ja.insert("Source:", "ソース:");
        ja.insert("Bundled", "内蔵");
        ja.insert("System", "システム");
        ja.insert("Font:", "フォント:");
        ja.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "プロジェクトに含まれる内蔵フォント（SIL OFL 1.1、GPLv3互換使用）。");
        ja.insert("Loading font previews...", "フォントプレビューを読み込み中…");
        ja.insert("No system fonts detected. Use Bundled or File source.", "システムフォントが検出されません。内蔵またはファイルソースを使用してください。");
        ja.insert("Load Font File", "フォントファイルを読み込む");
        ja.insert("Add Text to Drawing", "図面にテキストを追加");
        ja.insert("Serial", "シリアル");
        ja.insert("CSV Column", "CSV列");
        ja.insert("Prefix:", "接頭辞:");
        ja.insert("Suffix:", "接尾辞:");
        ja.insert("Start:", "開始:");
        ja.insert("Inc:", "増分:");
        ja.insert("Pad:", "桁数:");
        ja.insert("Batch Count:", "バッチ数:");
        ja.insert("Column:", "列:");
        ja.insert("Header row", "ヘッダー行");
        ja.insert("Delimiter:", "区切り文字:");
        ja.insert("Load CSV", "CSV読み込み");
        ja.insert("Align / Distribute", "整列 / 配分");
        ja.insert("Align Left", "左揃え");
        ja.insert("Align Right", "右揃え");
        ja.insert("Align Top", "上揃え");
        ja.insert("Align Bottom", "下揃え");
        ja.insert("Center Horizontal", "水平中央");
        ja.insert("Center Vertical", "垂直中央");
        ja.insert("Distribute H", "水平配分");
        ja.insert("Distribute V", "垂直配分");
        ja.insert("Shape Properties", "形状プロパティ");
        ja.insert("Select a shape to edit properties.", "プロパティを編集するには形状を選択してください。");
        ja.insert("Session Recovery", "セッション復旧");
        ja.insert("A previous session was interrupted. Restore it?", "前回のセッションが中断されました。復元しますか？");
        ja.insert("Restore", "復元");
        ja.insert("Discard", "破棄");
        ja.insert("Load a GCode file or draw shapes to preview", "GCodeファイルを読み込むか、形状を描いてプレビュー");
        ja.insert("Apply Recommended", "推奨を適用");
        ja.insert("Apply to Active Layer", "アクティブレイヤーに適用");
        ja.insert("Material Presets", "素材プリセット");
        ja.insert("Mode", "モード");
        ja.insert("Spd/Pwr", "速度/出力");
        ja.insert("Out", "出力");
        ja.insert("Array", "配列");
        ja.insert("Grid", "グリッド");
        ja.insert("Offset", "オフセット");
        ja.insert("Boolean", "ブーリアン");
        ja.insert("Circular Array", "円形配列");
        ja.insert("Grid Array", "グリッド配列");
        ja.insert("Offset Path", "パスオフセット");
        ja.insert("Boolean Operations", "ブーリアン演算");
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
        // New UI keys
        de.insert("Node Editing", "Knotenbearbeitung");
        de.insert("Selection", "Auswahl");
        de.insert("Create", "Erstellen");
        de.insert("Modify", "Ändern");
        de.insert("Rect", "Rechteck");
        de.insert("Circle", "Kreis");
        de.insert("Origin X:", "Ursprung X:");
        de.insert("Radius:", "Radius:");
        de.insert("Layer:", "Ebene:");
        de.insert("Set to Active Layer", "Aktive Ebene zuweisen");
        de.insert("Use the Text Tool panel below to create text paths.", "Verwenden Sie das Textwerkzeug-Panel unten, um Textpfade zu erstellen.");
        de.insert("Text Tool", "Textwerkzeug");
        de.insert("Variable Text (Serial Numbers)", "Variabler Text (Seriennummern)");
        de.insert("Text:", "Text:");
        de.insert("Size:", "Größe:");
        de.insert("Source:", "Quelle:");
        de.insert("Bundled", "Mitgeliefert");
        de.insert("System", "System");
        de.insert("Font:", "Schriftart:");
        de.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Im Projekt enthaltene Schriftarten (SIL OFL 1.1, GPLv3-kompatibel).");
        de.insert("Loading font previews...", "Schriftvorschau wird geladen…");
        de.insert("No system fonts detected. Use Bundled or File source.", "Keine Systemschriften erkannt. Verwenden Sie mitgelieferte oder Dateiquelle.");
        de.insert("Load Font File", "Schriftdatei laden");
        de.insert("Add Text to Drawing", "Text zur Zeichnung hinzufügen");
        de.insert("Serial", "Seriell");
        de.insert("CSV Column", "CSV-Spalte");
        de.insert("Prefix:", "Präfix:");
        de.insert("Suffix:", "Suffix:");
        de.insert("Start:", "Start:");
        de.insert("Inc:", "Inkr.:");
        de.insert("Pad:", "Auffüllen:");
        de.insert("Batch Count:", "Stückzahl:");
        de.insert("Column:", "Spalte:");
        de.insert("Header row", "Kopfzeile");
        de.insert("Delimiter:", "Trennzeichen:");
        de.insert("Load CSV", "CSV laden");
        de.insert("Align / Distribute", "Ausrichten / Verteilen");
        de.insert("Align Left", "Links ausrichten");
        de.insert("Align Right", "Rechts ausrichten");
        de.insert("Align Top", "Oben ausrichten");
        de.insert("Align Bottom", "Unten ausrichten");
        de.insert("Center Horizontal", "Horizontal zentrieren");
        de.insert("Center Vertical", "Vertikal zentrieren");
        de.insert("Distribute H", "Horizontal verteilen");
        de.insert("Distribute V", "Vertikal verteilen");
        de.insert("Shape Properties", "Formeigenschaften");
        de.insert("Select a shape to edit properties.", "Wählen Sie eine Form, um Eigenschaften zu bearbeiten.");
        de.insert("Session Recovery", "Sitzungswiederherstellung");
        de.insert("A previous session was interrupted. Restore it?", "Eine vorherige Sitzung wurde unterbrochen. Wiederherstellen?");
        de.insert("Restore", "Wiederherstellen");
        de.insert("Discard", "Verwerfen");
        de.insert("Load a GCode file or draw shapes to preview", "Laden Sie eine GCode-Datei oder zeichnen Sie Formen zur Vorschau");
        de.insert("Apply Recommended", "Empfohlene anwenden");
        de.insert("Apply to Active Layer", "Auf aktive Ebene anwenden");
        de.insert("Material Presets", "Materialvorlagen");
        de.insert("Mode", "Modus");
        de.insert("Spd/Pwr", "Geschw./Leist.");
        de.insert("Out", "Ausg.");
        de.insert("Array", "Anordnung");
        de.insert("Grid", "Raster");
        de.insert("Offset", "Versatz");
        de.insert("Boolean", "Boolesch");
        de.insert("Circular Array", "Kreisanordnung");
        de.insert("Grid Array", "Rasteranordnung");
        de.insert("Offset Path", "Pfadversatz");
        de.insert("Boolean Operations", "Boolesche Operationen");
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
        // New UI keys
        it.insert("Node Editing", "Modifica nodi");
        it.insert("Selection", "Selezione");
        it.insert("Create", "Crea");
        it.insert("Modify", "Modifica");
        it.insert("Rect", "Rettangolo");
        it.insert("Circle", "Cerchio");
        it.insert("Origin X:", "Origine X:");
        it.insert("Radius:", "Raggio:");
        it.insert("Layer:", "Livello:");
        it.insert("Set to Active Layer", "Imposta livello attivo");
        it.insert("Use the Text Tool panel below to create text paths.", "Usa il pannello Strumento Testo qui sotto per creare tracciati di testo.");
        it.insert("Text Tool", "Strumento Testo");
        it.insert("Variable Text (Serial Numbers)", "Testo variabile (numeri di serie)");
        it.insert("Text:", "Testo:");
        it.insert("Size:", "Dimensione:");
        it.insert("Source:", "Sorgente:");
        it.insert("Bundled", "Inclusi");
        it.insert("System", "Sistema");
        it.insert("Font:", "Font:");
        it.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Font inclusi nel progetto (SIL OFL 1.1, uso compatibile GPLv3).");
        it.insert("Loading font previews...", "Caricamento anteprima font…");
        it.insert("No system fonts detected. Use Bundled or File source.", "Nessun font di sistema rilevato. Usa i font inclusi o un file.");
        it.insert("Load Font File", "Carica file font");
        it.insert("Add Text to Drawing", "Aggiungi testo al disegno");
        it.insert("Serial", "Seriale");
        it.insert("CSV Column", "Colonna CSV");
        it.insert("Prefix:", "Prefisso:");
        it.insert("Suffix:", "Suffisso:");
        it.insert("Start:", "Inizio:");
        it.insert("Inc:", "Incr.:");
        it.insert("Pad:", "Riempimento:");
        it.insert("Batch Count:", "Conteggio lotto:");
        it.insert("Column:", "Colonna:");
        it.insert("Header row", "Riga intestazione");
        it.insert("Delimiter:", "Delimitatore:");
        it.insert("Load CSV", "Carica CSV");
        it.insert("Align / Distribute", "Allinea / Distribuisci");
        it.insert("Align Left", "Allinea a sinistra");
        it.insert("Align Right", "Allinea a destra");
        it.insert("Align Top", "Allinea in alto");
        it.insert("Align Bottom", "Allinea in basso");
        it.insert("Center Horizontal", "Centra orizzontalmente");
        it.insert("Center Vertical", "Centra verticalmente");
        it.insert("Distribute H", "Distribuisci O");
        it.insert("Distribute V", "Distribuisci V");
        it.insert("Shape Properties", "Proprietà forma");
        it.insert("Select a shape to edit properties.", "Seleziona una forma per modificarne le proprietà.");
        it.insert("Session Recovery", "Recupero sessione");
        it.insert("A previous session was interrupted. Restore it?", "Una sessione precedente è stata interrotta. Ripristinarla?");
        it.insert("Restore", "Ripristina");
        it.insert("Discard", "Scarta");
        it.insert("Load a GCode file or draw shapes to preview", "Carica un file GCode o disegna forme per l'anteprima");
        it.insert("Apply Recommended", "Applica consigliati");
        it.insert("Apply to Active Layer", "Applica al livello attivo");
        it.insert("Material Presets", "Preset materiali");
        it.insert("Mode", "Modalità");
        it.insert("Spd/Pwr", "Vel./Pot.");
        it.insert("Out", "Usc.");
        it.insert("Array", "Matrice");
        it.insert("Grid", "Griglia");
        it.insert("Offset", "Offset");
        it.insert("Boolean", "Booleano");
        it.insert("Circular Array", "Matrice circolare");
        it.insert("Grid Array", "Matrice a griglia");
        it.insert("Offset Path", "Offset tracciato");
        it.insert("Boolean Operations", "Operazioni booleane");
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
        // New UI keys
        es.insert("Node Editing", "Edición de nodos");
        es.insert("Selection", "Selección");
        es.insert("Create", "Crear");
        es.insert("Modify", "Modificar");
        es.insert("Rect", "Rectángulo");
        es.insert("Circle", "Círculo");
        es.insert("Origin X:", "Origen X:");
        es.insert("Radius:", "Radio:");
        es.insert("Layer:", "Capa:");
        es.insert("Set to Active Layer", "Asignar a capa activa");
        es.insert("Use the Text Tool panel below to create text paths.", "Use el panel Herramienta de Texto para crear trazados de texto.");
        es.insert("Text Tool", "Herramienta de Texto");
        es.insert("Variable Text (Serial Numbers)", "Texto variable (números de serie)");
        es.insert("Text:", "Texto:");
        es.insert("Size:", "Tamaño:");
        es.insert("Source:", "Fuente:");
        es.insert("Bundled", "Incluidas");
        es.insert("System", "Sistema");
        es.insert("Font:", "Tipografía:");
        es.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Tipografías incluidas en el proyecto (SIL OFL 1.1, uso compatible con GPLv3).");
        es.insert("Loading font previews...", "Cargando vista previa de tipografías…");
        es.insert("No system fonts detected. Use Bundled or File source.", "No se detectaron tipografías del sistema. Use las incluidas o un archivo.");
        es.insert("Load Font File", "Cargar archivo de tipografía");
        es.insert("Add Text to Drawing", "Añadir texto al dibujo");
        es.insert("Serial", "Serie");
        es.insert("CSV Column", "Columna CSV");
        es.insert("Prefix:", "Prefijo:");
        es.insert("Suffix:", "Sufijo:");
        es.insert("Start:", "Inicio:");
        es.insert("Inc:", "Incr.:");
        es.insert("Pad:", "Relleno:");
        es.insert("Batch Count:", "Cantidad de lotes:");
        es.insert("Column:", "Columna:");
        es.insert("Header row", "Fila de encabezado");
        es.insert("Delimiter:", "Delimitador:");
        es.insert("Load CSV", "Cargar CSV");
        es.insert("Align / Distribute", "Alinear / Distribuir");
        es.insert("Align Left", "Alinear a la izquierda");
        es.insert("Align Right", "Alinear a la derecha");
        es.insert("Align Top", "Alinear arriba");
        es.insert("Align Bottom", "Alinear abajo");
        es.insert("Center Horizontal", "Centrar horizontalmente");
        es.insert("Center Vertical", "Centrar verticalmente");
        es.insert("Distribute H", "Distribuir H");
        es.insert("Distribute V", "Distribuir V");
        es.insert("Shape Properties", "Propiedades de forma");
        es.insert("Select a shape to edit properties.", "Seleccione una forma para editar propiedades.");
        es.insert("Session Recovery", "Recuperación de sesión");
        es.insert("A previous session was interrupted. Restore it?", "Una sesión anterior fue interrumpida. ¿Restaurarla?");
        es.insert("Restore", "Restaurar");
        es.insert("Discard", "Descartar");
        es.insert("Load a GCode file or draw shapes to preview", "Cargue un archivo GCode o dibuje formas para previsualizar");
        es.insert("Apply Recommended", "Aplicar recomendados");
        es.insert("Apply to Active Layer", "Aplicar a capa activa");
        es.insert("Material Presets", "Preajustes de material");
        es.insert("Mode", "Modo");
        es.insert("Spd/Pwr", "Vel./Pot.");
        es.insert("Out", "Sal.");
        es.insert("Array", "Matriz");
        es.insert("Grid", "Cuadrícula");
        es.insert("Offset", "Desplazamiento");
        es.insert("Boolean", "Booleano");
        es.insert("Circular Array", "Matriz circular");
        es.insert("Grid Array", "Matriz en cuadrícula");
        es.insert("Offset Path", "Desplazar trazado");
        es.insert("Boolean Operations", "Operaciones booleanas");
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
        // New UI keys
        pt.insert("Node Editing", "Edição de nós");
        pt.insert("Selection", "Seleção");
        pt.insert("Create", "Criar");
        pt.insert("Modify", "Modificar");
        pt.insert("Rect", "Retângulo");
        pt.insert("Circle", "Círculo");
        pt.insert("Origin X:", "Origem X:");
        pt.insert("Radius:", "Raio:");
        pt.insert("Layer:", "Camada:");
        pt.insert("Set to Active Layer", "Definir como camada ativa");
        pt.insert("Use the Text Tool panel below to create text paths.", "Use o painel Ferramenta de Texto abaixo para criar caminhos de texto.");
        pt.insert("Text Tool", "Ferramenta de Texto");
        pt.insert("Variable Text (Serial Numbers)", "Texto variável (números de série)");
        pt.insert("Text:", "Texto:");
        pt.insert("Size:", "Tamanho:");
        pt.insert("Source:", "Fonte:");
        pt.insert("Bundled", "Incluídas");
        pt.insert("System", "Sistema");
        pt.insert("Font:", "Tipografia:");
        pt.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Fontes incluídas no projeto (SIL OFL 1.1, uso compatível com GPLv3).");
        pt.insert("Loading font previews...", "Carregando pré-visualização de fontes…");
        pt.insert("No system fonts detected. Use Bundled or File source.", "Nenhuma fonte do sistema detectada. Use as incluídas ou um arquivo.");
        pt.insert("Load Font File", "Carregar arquivo de fonte");
        pt.insert("Add Text to Drawing", "Adicionar texto ao desenho");
        pt.insert("Serial", "Série");
        pt.insert("CSV Column", "Coluna CSV");
        pt.insert("Prefix:", "Prefixo:");
        pt.insert("Suffix:", "Sufixo:");
        pt.insert("Start:", "Início:");
        pt.insert("Inc:", "Incr.:");
        pt.insert("Pad:", "Preenchimento:");
        pt.insert("Batch Count:", "Quantidade de lotes:");
        pt.insert("Column:", "Coluna:");
        pt.insert("Header row", "Linha de cabeçalho");
        pt.insert("Delimiter:", "Delimitador:");
        pt.insert("Load CSV", "Carregar CSV");
        pt.insert("Align / Distribute", "Alinhar / Distribuir");
        pt.insert("Align Left", "Alinhar à esquerda");
        pt.insert("Align Right", "Alinhar à direita");
        pt.insert("Align Top", "Alinhar ao topo");
        pt.insert("Align Bottom", "Alinhar à base");
        pt.insert("Center Horizontal", "Centralizar horizontalmente");
        pt.insert("Center Vertical", "Centralizar verticalmente");
        pt.insert("Distribute H", "Distribuir H");
        pt.insert("Distribute V", "Distribuir V");
        pt.insert("Shape Properties", "Propriedades da forma");
        pt.insert("Select a shape to edit properties.", "Selecione uma forma para editar propriedades.");
        pt.insert("Session Recovery", "Recuperação de sessão");
        pt.insert("A previous session was interrupted. Restore it?", "Uma sessão anterior foi interrompida. Restaurá-la?");
        pt.insert("Restore", "Restaurar");
        pt.insert("Discard", "Descartar");
        pt.insert("Load a GCode file or draw shapes to preview", "Carregue um arquivo GCode ou desenhe formas para pré-visualizar");
        pt.insert("Apply Recommended", "Aplicar recomendados");
        pt.insert("Apply to Active Layer", "Aplicar à camada ativa");
        pt.insert("Material Presets", "Predefinições de material");
        pt.insert("Mode", "Modo");
        pt.insert("Spd/Pwr", "Vel./Pot.");
        pt.insert("Out", "Saída");
        pt.insert("Array", "Matriz");
        pt.insert("Grid", "Grade");
        pt.insert("Offset", "Deslocamento");
        pt.insert("Boolean", "Booleano");
        pt.insert("Circular Array", "Matriz circular");
        pt.insert("Grid Array", "Matriz em grade");
        pt.insert("Offset Path", "Deslocar caminho");
        pt.insert("Boolean Operations", "Operações booleanas");
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
        // New UI keys
        ar.insert("Node Editing", "تحرير العقد");
        ar.insert("Selection", "تحديد");
        ar.insert("Create", "إنشاء");
        ar.insert("Modify", "تعديل");
        ar.insert("Rect", "مستطيل");
        ar.insert("Circle", "دائرة");
        ar.insert("Origin X:", "الأصل X:");
        ar.insert("Radius:", "نصف القطر:");
        ar.insert("Layer:", "الطبقة:");
        ar.insert("Set to Active Layer", "تعيين الطبقة النشطة");
        ar.insert("Use the Text Tool panel below to create text paths.", "استخدم لوحة أداة النص أدناه لإنشاء مسارات نصية.");
        ar.insert("Text Tool", "أداة النص");
        ar.insert("Variable Text (Serial Numbers)", "نص متغير (أرقام تسلسلية)");
        ar.insert("Text:", "النص:");
        ar.insert("Size:", "الحجم:");
        ar.insert("Source:", "المصدر:");
        ar.insert("Bundled", "مدمجة");
        ar.insert("System", "النظام");
        ar.insert("Font:", "الخط:");
        ar.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "خطوط مدمجة في المشروع (SIL OFL 1.1، استخدام متوافق مع GPLv3).");
        ar.insert("Loading font previews...", "جارٍ تحميل معاينة الخطوط…");
        ar.insert("No system fonts detected. Use Bundled or File source.", "لم يتم اكتشاف خطوط النظام. استخدم الخطوط المدمجة أو ملف.");
        ar.insert("Load Font File", "تحميل ملف خط");
        ar.insert("Add Text to Drawing", "إضافة نص إلى الرسم");
        ar.insert("Serial", "تسلسلي");
        ar.insert("CSV Column", "عمود CSV");
        ar.insert("Prefix:", "بادئة:");
        ar.insert("Suffix:", "لاحقة:");
        ar.insert("Start:", "البداية:");
        ar.insert("Inc:", "الزيادة:");
        ar.insert("Pad:", "الحشو:");
        ar.insert("Batch Count:", "عدد الدُفعات:");
        ar.insert("Column:", "العمود:");
        ar.insert("Header row", "صف العنوان");
        ar.insert("Delimiter:", "الفاصل:");
        ar.insert("Load CSV", "تحميل CSV");
        ar.insert("Align / Distribute", "محاذاة / توزيع");
        ar.insert("Align Left", "محاذاة لليسار");
        ar.insert("Align Right", "محاذاة لليمين");
        ar.insert("Align Top", "محاذاة للأعلى");
        ar.insert("Align Bottom", "محاذاة للأسفل");
        ar.insert("Center Horizontal", "توسيط أفقي");
        ar.insert("Center Vertical", "توسيط عمودي");
        ar.insert("Distribute H", "توزيع أفقي");
        ar.insert("Distribute V", "توزيع عمودي");
        ar.insert("Shape Properties", "خصائص الشكل");
        ar.insert("Select a shape to edit properties.", "حدد شكلاً لتعديل الخصائص.");
        ar.insert("Session Recovery", "استعادة الجلسة");
        ar.insert("A previous session was interrupted. Restore it?", "تمت مقاطعة جلسة سابقة. هل تريد استعادتها؟");
        ar.insert("Restore", "استعادة");
        ar.insert("Discard", "تجاهل");
        ar.insert("Load a GCode file or draw shapes to preview", "قم بتحميل ملف GCode أو ارسم أشكالاً للمعاينة");
        ar.insert("Apply Recommended", "تطبيق الموصى به");
        ar.insert("Apply to Active Layer", "تطبيق على الطبقة النشطة");
        ar.insert("Material Presets", "إعدادات المواد المسبقة");
        ar.insert("Mode", "الوضع");
        ar.insert("Spd/Pwr", "سرعة/طاقة");
        ar.insert("Out", "خرج");
        ar.insert("Array", "مصفوفة");
        ar.insert("Grid", "شبكة");
        ar.insert("Offset", "إزاحة");
        ar.insert("Boolean", "منطقي");
        ar.insert("Circular Array", "مصفوفة دائرية");
        ar.insert("Grid Array", "مصفوفة شبكية");
        ar.insert("Offset Path", "إزاحة المسار");
        ar.insert("Boolean Operations", "العمليات المنطقية");
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
        // New UI keys
        zh.insert("Node Editing", "节点编辑");
        zh.insert("Selection", "选择");
        zh.insert("Create", "创建");
        zh.insert("Modify", "修改");
        zh.insert("Rect", "矩形");
        zh.insert("Circle", "圆");
        zh.insert("Origin X:", "原点 X:");
        zh.insert("Radius:", "半径:");
        zh.insert("Layer:", "图层:");
        zh.insert("Set to Active Layer", "设置为活动图层");
        zh.insert("Use the Text Tool panel below to create text paths.", "使用下方的文字工具面板创建文字路径。");
        zh.insert("Text Tool", "文字工具");
        zh.insert("Variable Text (Serial Numbers)", "变量文本（序列号）");
        zh.insert("Text:", "文本:");
        zh.insert("Size:", "大小:");
        zh.insert("Source:", "来源:");
        zh.insert("Bundled", "内置");
        zh.insert("System", "系统");
        zh.insert("Font:", "字体:");
        zh.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "项目内置字体（SIL OFL 1.1，GPLv3兼容使用）。");
        zh.insert("Loading font previews...", "正在加载字体预览…");
        zh.insert("No system fonts detected. Use Bundled or File source.", "未检测到系统字体。请使用内置或文件来源。");
        zh.insert("Load Font File", "加载字体文件");
        zh.insert("Add Text to Drawing", "添加文本到图纸");
        zh.insert("Serial", "序列");
        zh.insert("CSV Column", "CSV列");
        zh.insert("Prefix:", "前缀:");
        zh.insert("Suffix:", "后缀:");
        zh.insert("Start:", "起始:");
        zh.insert("Inc:", "增量:");
        zh.insert("Pad:", "填充:");
        zh.insert("Batch Count:", "批次数:");
        zh.insert("Column:", "列:");
        zh.insert("Header row", "标题行");
        zh.insert("Delimiter:", "分隔符:");
        zh.insert("Load CSV", "加载CSV");
        zh.insert("Align / Distribute", "对齐 / 分布");
        zh.insert("Align Left", "左对齐");
        zh.insert("Align Right", "右对齐");
        zh.insert("Align Top", "顶部对齐");
        zh.insert("Align Bottom", "底部对齐");
        zh.insert("Center Horizontal", "水平居中");
        zh.insert("Center Vertical", "垂直居中");
        zh.insert("Distribute H", "水平分布");
        zh.insert("Distribute V", "垂直分布");
        zh.insert("Shape Properties", "形状属性");
        zh.insert("Select a shape to edit properties.", "选择一个形状以编辑属性。");
        zh.insert("Session Recovery", "会话恢复");
        zh.insert("A previous session was interrupted. Restore it?", "上次会话被中断。是否恢复？");
        zh.insert("Restore", "恢复");
        zh.insert("Discard", "放弃");
        zh.insert("Load a GCode file or draw shapes to preview", "加载GCode文件或绘制形状以预览");
        zh.insert("Apply Recommended", "应用推荐");
        zh.insert("Apply to Active Layer", "应用到活动图层");
        zh.insert("Material Presets", "材料预设");
        zh.insert("Mode", "模式");
        zh.insert("Spd/Pwr", "速度/功率");
        zh.insert("Out", "输出");
        zh.insert("Array", "阵列");
        zh.insert("Grid", "网格");
        zh.insert("Offset", "偏移");
        zh.insert("Boolean", "布尔");
        zh.insert("Circular Array", "圆形阵列");
        zh.insert("Grid Array", "网格阵列");
        zh.insert("Offset Path", "路径偏移");
        zh.insert("Boolean Operations", "布尔运算");
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
        // New UI keys
        ru.insert("Node Editing", "Редактирование узлов");
        ru.insert("Selection", "Выделение");
        ru.insert("Create", "Создать");
        ru.insert("Modify", "Изменить");
        ru.insert("Rect", "Прямоугольник");
        ru.insert("Circle", "Круг");
        ru.insert("Origin X:", "Начало X:");
        ru.insert("Radius:", "Радиус:");
        ru.insert("Layer:", "Слой:");
        ru.insert("Set to Active Layer", "Назначить активный слой");
        ru.insert("Use the Text Tool panel below to create text paths.", "Используйте панель текстового инструмента ниже для создания текстовых контуров.");
        ru.insert("Text Tool", "Текстовый инструмент");
        ru.insert("Variable Text (Serial Numbers)", "Переменный текст (серийные номера)");
        ru.insert("Text:", "Текст:");
        ru.insert("Size:", "Размер:");
        ru.insert("Source:", "Источник:");
        ru.insert("Bundled", "Встроенные");
        ru.insert("System", "Системные");
        ru.insert("Font:", "Шрифт:");
        ru.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Встроенные шрифты проекта (SIL OFL 1.1, совместимые с GPLv3).");
        ru.insert("Loading font previews...", "Загрузка превью шрифтов…");
        ru.insert("No system fonts detected. Use Bundled or File source.", "Системные шрифты не обнаружены. Используйте встроенные или файл.");
        ru.insert("Load Font File", "Загрузить файл шрифта");
        ru.insert("Add Text to Drawing", "Добавить текст на чертёж");
        ru.insert("Serial", "Серийный");
        ru.insert("CSV Column", "Столбец CSV");
        ru.insert("Prefix:", "Префикс:");
        ru.insert("Suffix:", "Суффикс:");
        ru.insert("Start:", "Начало:");
        ru.insert("Inc:", "Шаг:");
        ru.insert("Pad:", "Заполнение:");
        ru.insert("Batch Count:", "Кол-во партий:");
        ru.insert("Column:", "Столбец:");
        ru.insert("Header row", "Строка заголовка");
        ru.insert("Delimiter:", "Разделитель:");
        ru.insert("Load CSV", "Загрузить CSV");
        ru.insert("Align / Distribute", "Выравнивание / Распределение");
        ru.insert("Align Left", "По левому краю");
        ru.insert("Align Right", "По правому краю");
        ru.insert("Align Top", "По верхнему краю");
        ru.insert("Align Bottom", "По нижнему краю");
        ru.insert("Center Horizontal", "Центрировать по горизонтали");
        ru.insert("Center Vertical", "Центрировать по вертикали");
        ru.insert("Distribute H", "Распределить по горизонтали");
        ru.insert("Distribute V", "Распределить по вертикали");
        ru.insert("Shape Properties", "Свойства фигуры");
        ru.insert("Select a shape to edit properties.", "Выберите фигуру для редактирования свойств.");
        ru.insert("Session Recovery", "Восстановление сессии");
        ru.insert("A previous session was interrupted. Restore it?", "Предыдущая сессия была прервана. Восстановить?");
        ru.insert("Restore", "Восстановить");
        ru.insert("Discard", "Отклонить");
        ru.insert("Load a GCode file or draw shapes to preview", "Загрузите файл GCode или нарисуйте фигуры для предпросмотра");
        ru.insert("Apply Recommended", "Применить рекомендуемые");
        ru.insert("Apply to Active Layer", "Применить к активному слою");
        ru.insert("Material Presets", "Шаблоны материалов");
        ru.insert("Mode", "Режим");
        ru.insert("Spd/Pwr", "Скор./Мощн.");
        ru.insert("Out", "Выход");
        ru.insert("Array", "Массив");
        ru.insert("Grid", "Сетка");
        ru.insert("Offset", "Смещение");
        ru.insert("Boolean", "Булевы");
        ru.insert("Circular Array", "Круговой массив");
        ru.insert("Grid Array", "Сеточный массив");
        ru.insert("Offset Path", "Смещение контура");
        ru.insert("Boolean Operations", "Булевы операции");
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
        // New UI keys
        tr_lang.insert("Node Editing", "Düğüm Düzenleme");
        tr_lang.insert("Selection", "Seçim");
        tr_lang.insert("Create", "Oluştur");
        tr_lang.insert("Modify", "Değiştir");
        tr_lang.insert("Rect", "Dikdörtgen");
        tr_lang.insert("Circle", "Daire");
        tr_lang.insert("Origin X:", "Başlangıç X:");
        tr_lang.insert("Radius:", "Yarıçap:");
        tr_lang.insert("Layer:", "Katman:");
        tr_lang.insert("Set to Active Layer", "Aktif Katmana Ata");
        tr_lang.insert("Use the Text Tool panel below to create text paths.", "Metin yolları oluşturmak için aşağıdaki Metin Aracı panelini kullanın.");
        tr_lang.insert("Text Tool", "Metin Aracı");
        tr_lang.insert("Variable Text (Serial Numbers)", "Değişken Metin (Seri Numaraları)");
        tr_lang.insert("Text:", "Metin:");
        tr_lang.insert("Size:", "Boyut:");
        tr_lang.insert("Source:", "Kaynak:");
        tr_lang.insert("Bundled", "Dahili");
        tr_lang.insert("System", "Sistem");
        tr_lang.insert("Font:", "Yazı Tipi:");
        tr_lang.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Projeye dahil yazı tipleri (SIL OFL 1.1, GPLv3 uyumlu kullanım).");
        tr_lang.insert("Loading font previews...", "Yazı tipi önizlemeleri yükleniyor…");
        tr_lang.insert("No system fonts detected. Use Bundled or File source.", "Sistem yazı tipi bulunamadı. Dahili veya dosya kaynağı kullanın.");
        tr_lang.insert("Load Font File", "Yazı Tipi Dosyası Yükle");
        tr_lang.insert("Add Text to Drawing", "Çizime Metin Ekle");
        tr_lang.insert("Serial", "Seri");
        tr_lang.insert("CSV Column", "CSV Sütunu");
        tr_lang.insert("Prefix:", "Önek:");
        tr_lang.insert("Suffix:", "Sonek:");
        tr_lang.insert("Start:", "Başlangıç:");
        tr_lang.insert("Inc:", "Artış:");
        tr_lang.insert("Pad:", "Doldurma:");
        tr_lang.insert("Batch Count:", "Parti Sayısı:");
        tr_lang.insert("Column:", "Sütun:");
        tr_lang.insert("Header row", "Başlık satırı");
        tr_lang.insert("Delimiter:", "Ayırıcı:");
        tr_lang.insert("Load CSV", "CSV Yükle");
        tr_lang.insert("Align / Distribute", "Hizala / Dağıt");
        tr_lang.insert("Align Left", "Sola Hizala");
        tr_lang.insert("Align Right", "Sağa Hizala");
        tr_lang.insert("Align Top", "Üste Hizala");
        tr_lang.insert("Align Bottom", "Alta Hizala");
        tr_lang.insert("Center Horizontal", "Yatay Ortala");
        tr_lang.insert("Center Vertical", "Dikey Ortala");
        tr_lang.insert("Distribute H", "Yatay Dağıt");
        tr_lang.insert("Distribute V", "Dikey Dağıt");
        tr_lang.insert("Shape Properties", "Şekil Özellikleri");
        tr_lang.insert("Select a shape to edit properties.", "Özellikleri düzenlemek için bir şekil seçin.");
        tr_lang.insert("Session Recovery", "Oturum Kurtarma");
        tr_lang.insert("A previous session was interrupted. Restore it?", "Önceki oturum kesildi. Geri yüklensin mi?");
        tr_lang.insert("Restore", "Geri Yükle");
        tr_lang.insert("Discard", "At");
        tr_lang.insert("Load a GCode file or draw shapes to preview", "Önizleme için bir GCode dosyası yükleyin veya şekil çizin");
        tr_lang.insert("Apply Recommended", "Önerileni Uygula");
        tr_lang.insert("Apply to Active Layer", "Aktif Katmana Uygula");
        tr_lang.insert("Material Presets", "Malzeme Ön Ayarları");
        tr_lang.insert("Mode", "Mod");
        tr_lang.insert("Spd/Pwr", "Hız/Güç");
        tr_lang.insert("Out", "Çıkış");
        tr_lang.insert("Array", "Dizi");
        tr_lang.insert("Grid", "Izgara");
        tr_lang.insert("Offset", "Kaydırma");
        tr_lang.insert("Boolean", "Mantıksal");
        tr_lang.insert("Circular Array", "Dairesel Dizi");
        tr_lang.insert("Grid Array", "Izgara Dizisi");
        tr_lang.insert("Offset Path", "Yol Kaydırma");
        tr_lang.insert("Boolean Operations", "Mantıksal İşlemler");
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
        // New UI keys
        ko.insert("Node Editing", "노드 편집");
        ko.insert("Selection", "선택");
        ko.insert("Create", "생성");
        ko.insert("Modify", "수정");
        ko.insert("Rect", "사각형");
        ko.insert("Circle", "원");
        ko.insert("Origin X:", "원점 X:");
        ko.insert("Radius:", "반지름:");
        ko.insert("Layer:", "레이어:");
        ko.insert("Set to Active Layer", "활성 레이어로 설정");
        ko.insert("Use the Text Tool panel below to create text paths.", "아래의 텍스트 도구 패널을 사용하여 텍스트 경로를 만드세요.");
        ko.insert("Text Tool", "텍스트 도구");
        ko.insert("Variable Text (Serial Numbers)", "변수 텍스트 (일련번호)");
        ko.insert("Text:", "텍스트:");
        ko.insert("Size:", "크기:");
        ko.insert("Source:", "소스:");
        ko.insert("Bundled", "내장");
        ko.insert("System", "시스템");
        ko.insert("Font:", "글꼴:");
        ko.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "프로젝트에 포함된 내장 글꼴 (SIL OFL 1.1, GPLv3 호환 사용).");
        ko.insert("Loading font previews...", "글꼴 미리보기 로딩 중…");
        ko.insert("No system fonts detected. Use Bundled or File source.", "시스템 글꼴을 찾을 수 없습니다. 내장 또는 파일 소스를 사용하세요.");
        ko.insert("Load Font File", "글꼴 파일 불러오기");
        ko.insert("Add Text to Drawing", "도면에 텍스트 추가");
        ko.insert("Serial", "시리얼");
        ko.insert("CSV Column", "CSV 열");
        ko.insert("Prefix:", "접두사:");
        ko.insert("Suffix:", "접미사:");
        ko.insert("Start:", "시작:");
        ko.insert("Inc:", "증가:");
        ko.insert("Pad:", "자릿수:");
        ko.insert("Batch Count:", "배치 수:");
        ko.insert("Column:", "열:");
        ko.insert("Header row", "헤더 행");
        ko.insert("Delimiter:", "구분자:");
        ko.insert("Load CSV", "CSV 불러오기");
        ko.insert("Align / Distribute", "정렬 / 배분");
        ko.insert("Align Left", "왼쪽 정렬");
        ko.insert("Align Right", "오른쪽 정렬");
        ko.insert("Align Top", "위쪽 정렬");
        ko.insert("Align Bottom", "아래쪽 정렬");
        ko.insert("Center Horizontal", "수평 중앙");
        ko.insert("Center Vertical", "수직 중앙");
        ko.insert("Distribute H", "수평 배분");
        ko.insert("Distribute V", "수직 배분");
        ko.insert("Shape Properties", "도형 속성");
        ko.insert("Select a shape to edit properties.", "속성을 편집할 도형을 선택하세요.");
        ko.insert("Session Recovery", "세션 복구");
        ko.insert("A previous session was interrupted. Restore it?", "이전 세션이 중단되었습니다. 복원하시겠습니까?");
        ko.insert("Restore", "복원");
        ko.insert("Discard", "삭제");
        ko.insert("Load a GCode file or draw shapes to preview", "GCode 파일을 불러오거나 도형을 그려서 미리보기");
        ko.insert("Apply Recommended", "추천 적용");
        ko.insert("Apply to Active Layer", "활성 레이어에 적용");
        ko.insert("Material Presets", "재료 프리셋");
        ko.insert("Mode", "모드");
        ko.insert("Spd/Pwr", "속도/출력");
        ko.insert("Out", "출력");
        ko.insert("Array", "배열");
        ko.insert("Grid", "그리드");
        ko.insert("Offset", "오프셋");
        ko.insert("Boolean", "불리언");
        ko.insert("Circular Array", "원형 배열");
        ko.insert("Grid Array", "그리드 배열");
        ko.insert("Offset Path", "경로 오프셋");
        ko.insert("Boolean Operations", "불리언 연산");
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
        // New UI keys
        pl.insert("Node Editing", "Edycja węzłów");
        pl.insert("Selection", "Zaznaczenie");
        pl.insert("Create", "Utwórz");
        pl.insert("Modify", "Modyfikuj");
        pl.insert("Rect", "Prostokąt");
        pl.insert("Circle", "Okrąg");
        pl.insert("Origin X:", "Początek X:");
        pl.insert("Radius:", "Promień:");
        pl.insert("Layer:", "Warstwa:");
        pl.insert("Set to Active Layer", "Ustaw aktywną warstwę");
        pl.insert("Use the Text Tool panel below to create text paths.", "Użyj panelu Narzędzia tekstu poniżej, aby tworzyć ścieżki tekstowe.");
        pl.insert("Text Tool", "Narzędzie tekstu");
        pl.insert("Variable Text (Serial Numbers)", "Tekst zmienny (numery seryjne)");
        pl.insert("Text:", "Tekst:");
        pl.insert("Size:", "Rozmiar:");
        pl.insert("Source:", "Źródło:");
        pl.insert("Bundled", "Wbudowane");
        pl.insert("System", "Systemowe");
        pl.insert("Font:", "Czcionka:");
        pl.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Czcionki wbudowane w projekt (SIL OFL 1.1, użycie zgodne z GPLv3).");
        pl.insert("Loading font previews...", "Ładowanie podglądu czcionek…");
        pl.insert("No system fonts detected. Use Bundled or File source.", "Nie wykryto czcionek systemowych. Użyj wbudowanych lub pliku.");
        pl.insert("Load Font File", "Załaduj plik czcionki");
        pl.insert("Add Text to Drawing", "Dodaj tekst do rysunku");
        pl.insert("Serial", "Seryjny");
        pl.insert("CSV Column", "Kolumna CSV");
        pl.insert("Prefix:", "Prefiks:");
        pl.insert("Suffix:", "Sufiks:");
        pl.insert("Start:", "Start:");
        pl.insert("Inc:", "Przyrost:");
        pl.insert("Pad:", "Wypełnienie:");
        pl.insert("Batch Count:", "Liczba partii:");
        pl.insert("Column:", "Kolumna:");
        pl.insert("Header row", "Wiersz nagłówka");
        pl.insert("Delimiter:", "Separator:");
        pl.insert("Load CSV", "Załaduj CSV");
        pl.insert("Align / Distribute", "Wyrównaj / Rozłóż");
        pl.insert("Align Left", "Wyrównaj do lewej");
        pl.insert("Align Right", "Wyrównaj do prawej");
        pl.insert("Align Top", "Wyrównaj do góry");
        pl.insert("Align Bottom", "Wyrównaj do dołu");
        pl.insert("Center Horizontal", "Wyśrodkuj poziomo");
        pl.insert("Center Vertical", "Wyśrodkuj pionowo");
        pl.insert("Distribute H", "Rozłóż poziomo");
        pl.insert("Distribute V", "Rozłóż pionowo");
        pl.insert("Shape Properties", "Właściwości kształtu");
        pl.insert("Select a shape to edit properties.", "Wybierz kształt, aby edytować właściwości.");
        pl.insert("Session Recovery", "Odzyskiwanie sesji");
        pl.insert("A previous session was interrupted. Restore it?", "Poprzednia sesja została przerwana. Przywrócić?");
        pl.insert("Restore", "Przywróć");
        pl.insert("Discard", "Odrzuć");
        pl.insert("Load a GCode file or draw shapes to preview", "Załaduj plik GCode lub narysuj kształty, aby wyświetlić podgląd");
        pl.insert("Apply Recommended", "Zastosuj zalecane");
        pl.insert("Apply to Active Layer", "Zastosuj do aktywnej warstwy");
        pl.insert("Material Presets", "Predefiniowane materiały");
        pl.insert("Mode", "Tryb");
        pl.insert("Spd/Pwr", "Prędk./Moc");
        pl.insert("Out", "Wyj.");
        pl.insert("Array", "Szyk");
        pl.insert("Grid", "Siatka");
        pl.insert("Offset", "Przesunięcie");
        pl.insert("Boolean", "Logiczne");
        pl.insert("Circular Array", "Szyk kołowy");
        pl.insert("Grid Array", "Szyk siatkowy");
        pl.insert("Offset Path", "Przesunięcie ścieżki");
        pl.insert("Boolean Operations", "Operacje logiczne");
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
    fn all_languages_have_new_ui_keys() {
        let required_keys = [
            "Node Editing", "Selection", "Create", "Modify",
            "Rect", "Circle", "Origin X:", "Radius:", "Layer:",
            "Set to Active Layer", "Text Tool", "Variable Text (Serial Numbers)",
            "Text:", "Size:", "Source:", "Bundled", "System", "Font:",
            "Load Font File", "Add Text to Drawing",
            "Serial", "CSV Column", "Prefix:", "Suffix:", "Start:", "Inc:", "Pad:",
            "Batch Count:", "Column:", "Header row", "Delimiter:", "Load CSV",
            "Align / Distribute", "Align Left", "Align Right", "Align Top", "Align Bottom",
            "Center Horizontal", "Center Vertical", "Distribute H", "Distribute V",
            "Shape Properties", "Select a shape to edit properties.",
            "Session Recovery", "Restore", "Discard",
            "Load a GCode file or draw shapes to preview",
            "Apply Recommended", "Apply to Active Layer", "Material Presets",
            "Mode", "Spd/Pwr", "Out",
            "Array", "Grid", "Offset", "Boolean",
            "Circular Array", "Grid Array", "Offset Path", "Boolean Operations",
        ];
        let languages = [
            Language::French, Language::Japanese, Language::German,
            Language::Italian, Language::Spanish, Language::Portuguese,
            Language::Arabic, Language::Chinese, Language::Russian,
            Language::Turkish, Language::Korean, Language::Polish,
        ];
        for lang in languages {
            let map = DICTIONARY.get(&lang).expect(&format!("Missing dictionary for {:?}", lang));
            for key in &required_keys {
                assert!(
                    map.contains_key(key),
                    "Missing key '{}' for {:?}",
                    key, lang
                );
            }
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
