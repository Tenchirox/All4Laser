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
