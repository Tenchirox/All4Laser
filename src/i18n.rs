use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;

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
        }
    }
}

// Global localization store
lazy_static! {
    static ref DICTIONARY: HashMap<Language, HashMap<&'static str, &'static str>> = {
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
        fr.insert("Industrial (advanced)", "Industriel (avancé)");
        fr.insert("Modern layout (simple)", "Disposition moderne (simple)");
        fr.insert("Classic layout (expert)", "Disposition classique (expert)");
        fr.insert("Beginner Mode", "Mode débutant");
        fr.insert("Connection & Control", "Connexion & Contrôle");
        fr.insert("Job Preparation", "Préparation du job");
        fr.insert("Creation & Editing", "Création & édition");
        fr.insert("Advanced Tools", "Outils avancés");
        fr.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "Mode débutant actif : interface simplifiée. Désactivez-le dans Affichage pour voir tous les outils.",
        );
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
        ja.insert("Industrial (advanced)", "インダストリアル（上級者向け）");
        ja.insert("Modern layout (simple)", "モダンレイアウト（シンプル）");
        ja.insert("Classic layout (expert)", "クラシックレイアウト（上級者向け）");
        ja.insert("Beginner Mode", "初心者モード");
        ja.insert("Connection & Control", "接続と操作");
        ja.insert("Job Preparation", "ジョブ準備");
        ja.insert("Creation & Editing", "作成と編集");
        ja.insert("Advanced Tools", "上級ツール");
        ja.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "初心者モードが有効です：UIは簡略化されています。すべてのツールを表示するには表示メニューで無効化してください。",
        );
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
        de.insert("Industrial (advanced)", "Industriell (erweitert)");
        de.insert("Modern layout (simple)", "Modernes Layout (einfach)");
        de.insert("Classic layout (expert)", "Klassisches Layout (Experte)");
        de.insert("Beginner Mode", "Anfängermodus");
        de.insert("Connection & Control", "Verbindung & Steuerung");
        de.insert("Job Preparation", "Jobvorbereitung");
        de.insert("Creation & Editing", "Erstellung & Bearbeitung");
        de.insert("Advanced Tools", "Erweiterte Werkzeuge");
        de.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "Anfängermodus aktiv: Die Oberfläche ist vereinfacht. Deaktivieren Sie ihn in Ansicht, um alle Werkzeuge zu sehen.",
        );
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
        it.insert("Industrial (advanced)", "Industriale (avanzato)");
        it.insert("Modern layout (simple)", "Layout moderno (semplice)");
        it.insert("Classic layout (expert)", "Layout classico (esperto)");
        it.insert("Beginner Mode", "Modalità principiante");
        it.insert("Connection & Control", "Connessione e Controllo");
        it.insert("Job Preparation", "Preparazione lavoro");
        it.insert("Creation & Editing", "Creazione e modifica");
        it.insert("Advanced Tools", "Strumenti avanzati");
        it.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "Modalità principiante attiva: interfaccia semplificata. Disattivala in Vista per mostrare tutti gli strumenti.",
        );
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
        es.insert("Industrial (advanced)", "Industrial (avanzado)");
        es.insert("Modern layout (simple)", "Diseño moderno (simple)");
        es.insert("Classic layout (expert)", "Diseño clásico (experto)");
        es.insert("Beginner Mode", "Modo principiante");
        es.insert("Connection & Control", "Conexión y control");
        es.insert("Job Preparation", "Preparación del trabajo");
        es.insert("Creation & Editing", "Creación y edición");
        es.insert("Advanced Tools", "Herramientas avanzadas");
        es.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "Modo principiante activo: interfaz simplificada. Desactívalo en Ver para mostrar todas las herramientas.",
        );
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
        pt.insert("Industrial (advanced)", "Industrial (avançado)");
        pt.insert("Modern layout (simple)", "Layout moderno (simples)");
        pt.insert("Classic layout (expert)", "Layout clássico (especialista)");
        pt.insert("Beginner Mode", "Modo iniciante");
        pt.insert("Connection & Control", "Conexão e Controle");
        pt.insert("Job Preparation", "Preparação do trabalho");
        pt.insert("Creation & Editing", "Criação e edição");
        pt.insert("Advanced Tools", "Ferramentas avançadas");
        pt.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "Modo iniciante ativo: interface simplificada. Desative em Visualizar para mostrar todas as ferramentas.",
        );
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
        ar.insert("Industrial (advanced)", "صناعي (متقدم)");
        ar.insert("Modern layout (simple)", "تخطيط حديث (بسيط)");
        ar.insert("Classic layout (expert)", "تخطيط كلاسيكي (خبير)");
        ar.insert("Beginner Mode", "وضع المبتدئ");
        ar.insert("Connection & Control", "الاتصال والتحكم");
        ar.insert("Job Preparation", "إعداد المهمة");
        ar.insert("Creation & Editing", "الإنشاء والتحرير");
        ar.insert("Advanced Tools", "أدوات متقدمة");
        ar.insert(
            "Beginner mode active: interface simplified. Disable it in View to show all tools.",
            "وضع المبتدئ مفعّل: الواجهة مبسطة. عطّله من عرض لإظهار جميع الأدوات.",
        );
        m.insert(Language::Arabic, ar);

        m
    };

    static ref CURRENT_LANG: RwLock<Language> = RwLock::new(Language::English);
}

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
