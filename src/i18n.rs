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
static DICTIONARY: LazyLock<HashMap<Language, HashMap<&'static str, &'static str>>> = LazyLock::new(|| {
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
    fr.insert("Cuts", "Coupes");
    fr.insert("Move", "Déplacer");
    fr.insert("Laser", "Laser");
    fr.insert("Layers", "Couches");
    fr.insert("Notes", "Notes");
    fr.insert("Project Notes", "Notes du projet");
    fr.insert("Measure", "Mesurer");
    fr.insert("Group", "Grouper");
    fr.insert("Ungroup", "Dégrouper");
    fr.insert("Air Assist", "Air Assist");
    fr.insert("Exhaust Fan", "Ventilation");
    fr.insert("Power Ramping", "Rampe de puissance");
    fr.insert("Perforation", "Perforation");
    fr.insert("Construction Layer", "Couche de construction");
    fr.insert("Maintenance", "Maintenance");
    fr.insert("Cost Estimate", "Estimation du coût");
    fr.insert("Export SVG", "Exporter SVG");
    fr.insert("Startup Wizard", "Assistant de démarrage");
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
    ja.insert("Cuts", "カット");
    ja.insert("Move", "移動");
    ja.insert("Laser", "レーザー");
    ja.insert("Layers", "レイヤー");
    ja.insert("Notes", "メモ");
    ja.insert("Project Notes", "プロジェクトメモ");
    ja.insert("Measure", "計測");
    ja.insert("Group", "グループ化");
    ja.insert("Ungroup", "グループ解除");
    ja.insert("Air Assist", "エアアシスト");
    ja.insert("Exhaust Fan", "排気ファン");
    ja.insert("Power Ramping", "パワーランピング");
    ja.insert("Perforation", "穴あけ");
    ja.insert("Construction Layer", "補助レイヤー");
    ja.insert("Maintenance", "メンテナンス");
    ja.insert("Cost Estimate", "コスト見積");
    ja.insert("Export SVG", "SVGエクスポート");
    ja.insert("Startup Wizard", "セットアップウィザード");
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
    de.insert("Cuts", "Schnitte");
    de.insert("Move", "Bewegen");
    de.insert("Laser", "Laser");
    de.insert("Layers", "Ebenen");
    de.insert("Notes", "Notizen");
    de.insert("Project Notes", "Projektnotizen");
    de.insert("Measure", "Messen");
    de.insert("Group", "Gruppieren");
    de.insert("Ungroup", "Gruppierung aufheben");
    de.insert("Air Assist", "Luftunterstützung");
    de.insert("Exhaust Fan", "Absaugventilator");
    de.insert("Power Ramping", "Leistungsrampe");
    de.insert("Perforation", "Perforation");
    de.insert("Construction Layer", "Konstruktionsebene");
    de.insert("Maintenance", "Wartung");
    de.insert("Cost Estimate", "Kostenschätzung");
    de.insert("Export SVG", "SVG exportieren");
    de.insert("Startup Wizard", "Einrichtungsassistent");
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
    it.insert("Cuts", "Tagli");
    it.insert("Move", "Sposta");
    it.insert("Laser", "Laser");
    it.insert("Layers", "Livelli");
    it.insert("Notes", "Note");
    it.insert("Project Notes", "Note del progetto");
    it.insert("Measure", "Misura");
    it.insert("Group", "Raggruppa");
    it.insert("Ungroup", "Separa");
    it.insert("Air Assist", "Aria Assistita");
    it.insert("Exhaust Fan", "Ventilatore aspirazione");
    it.insert("Power Ramping", "Rampa di potenza");
    it.insert("Perforation", "Perforazione");
    it.insert("Construction Layer", "Livello costruzione");
    it.insert("Maintenance", "Manutenzione");
    it.insert("Cost Estimate", "Stima dei costi");
    it.insert("Export SVG", "Esporta SVG");
    it.insert("Startup Wizard", "Assistente di avvio");
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
    es.insert("Cuts", "Cortes");
    es.insert("Move", "Mover");
    es.insert("Laser", "Láser");
    es.insert("Layers", "Capas");
    es.insert("Notes", "Notas");
    es.insert("Project Notes", "Notas del proyecto");
    es.insert("Measure", "Medir");
    es.insert("Group", "Agrupar");
    es.insert("Ungroup", "Desagrupar");
    es.insert("Air Assist", "Asistencia de aire");
    es.insert("Exhaust Fan", "Ventilador de extracción");
    es.insert("Power Ramping", "Rampa de potencia");
    es.insert("Perforation", "Perforación");
    es.insert("Construction Layer", "Capa de construcción");
    es.insert("Maintenance", "Mantenimiento");
    es.insert("Cost Estimate", "Estimación de costos");
    es.insert("Export SVG", "Exportar SVG");
    es.insert("Startup Wizard", "Asistente de inicio");
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
    pt.insert("Cuts", "Cortes");
    pt.insert("Move", "Mover");
    pt.insert("Laser", "Laser");
    pt.insert("Layers", "Camadas");
    pt.insert("Notes", "Notas");
    pt.insert("Project Notes", "Notas do projeto");
    pt.insert("Measure", "Medir");
    pt.insert("Group", "Agrupar");
    pt.insert("Ungroup", "Desagrupar");
    pt.insert("Air Assist", "Assistência de ar");
    pt.insert("Exhaust Fan", "Ventilador de exaustão");
    pt.insert("Power Ramping", "Rampa de potência");
    pt.insert("Perforation", "Perfuração");
    pt.insert("Construction Layer", "Camada de construção");
    pt.insert("Maintenance", "Manutenção");
    pt.insert("Cost Estimate", "Estimativa de custo");
    pt.insert("Export SVG", "Exportar SVG");
    pt.insert("Startup Wizard", "Assistente de início");
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
    ar.insert("Cuts", "القطع");
    ar.insert("Move", "نقل");
    ar.insert("Laser", "ليزر");
    ar.insert("Layers", "الطبقات");
    ar.insert("Notes", "ملاحظات");
    ar.insert("Project Notes", "ملاحظات المشروع");
    ar.insert("Measure", "قياس");
    ar.insert("Group", "تجميع");
    ar.insert("Ungroup", "إلغاء التجميع");
    ar.insert("Air Assist", "مساعد الهواء");
    ar.insert("Exhaust Fan", "مروحة العادم");
    ar.insert("Power Ramping", "تدرج الطاقة");
    ar.insert("Perforation", "تثقيب");
    ar.insert("Construction Layer", "طبقة البناء");
    ar.insert("Maintenance", "الصيانة");
    ar.insert("Cost Estimate", "تقدير التكلفة");
    ar.insert("Export SVG", "تصدير SVG");
    ar.insert("Startup Wizard", "معالج البدء");
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
    zh.insert("Air Assist", "气辅");
    zh.insert("Exhaust Fan", "排风扇");
    zh.insert("Power Ramping", "功率渐变");
    zh.insert("Perforation", "穿孔");
    zh.insert("Construction Layer", "辅助图层");
    zh.insert("Maintenance", "维护");
    zh.insert("Cost Estimate", "成本估算");
    zh.insert("Export SVG", "导出SVG");
    zh.insert("Startup Wizard", "启动向导");
    zh.insert("Modern (recommended)", "现代（推荐）");
    zh.insert("Industrial (advanced)", "工业（高级）");
    zh.insert("Modern layout (simple)", "现代布局（简单）");
    zh.insert("Classic layout (expert)", "经典布局（专家）");
    zh.insert("Beginner Mode", "新手模式");
    zh.insert("Connection & Control", "连接与控制");
    zh.insert("Job Preparation", "作业准备");
    zh.insert("Creation & Editing", "创建与编辑");
    zh.insert("Advanced Tools", "高级工具");
    zh.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "新手模式已启用：界面已简化。在视图中禁用以显示所有工具。");
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
    ru.insert("Industrial (advanced)", "Промышленный (продвинутый)");
    ru.insert("Modern layout (simple)", "Современный макет (простой)");
    ru.insert("Classic layout (expert)", "Классический макет (эксперт)");
    ru.insert("Beginner Mode", "Режим новичка");
    ru.insert("Connection & Control", "Подключение и управление");
    ru.insert("Job Preparation", "Подготовка задания");
    ru.insert("Creation & Editing", "Создание и редактирование");
    ru.insert("Advanced Tools", "Расширенные инструменты");
    ru.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "Режим новичка активен: интерфейс упрощён. Отключите в меню Вид для отображения всех инструментов.");
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
    tr_lang.insert("Industrial (advanced)", "Endüstriyel (gelişmiş)");
    tr_lang.insert("Modern layout (simple)", "Modern düzen (basit)");
    tr_lang.insert("Classic layout (expert)", "Klasik düzen (uzman)");
    tr_lang.insert("Beginner Mode", "Başlangıç Modu");
    tr_lang.insert("Connection & Control", "Bağlantı ve Kontrol");
    tr_lang.insert("Job Preparation", "İş Hazırlığı");
    tr_lang.insert("Creation & Editing", "Oluşturma ve Düzenleme");
    tr_lang.insert("Advanced Tools", "Gelişmiş Araçlar");
    tr_lang.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "Başlangıç modu etkin: arayüz basitleştirildi. Tüm araçları görmek için Görünüm'de devre dışı bırakın.");
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
    ko.insert("Industrial (advanced)", "산업용 (고급)");
    ko.insert("Modern layout (simple)", "현대적 레이아웃 (간단)");
    ko.insert("Classic layout (expert)", "클래식 레이아웃 (전문가)");
    ko.insert("Beginner Mode", "초보자 모드");
    ko.insert("Connection & Control", "연결 및 제어");
    ko.insert("Job Preparation", "작업 준비");
    ko.insert("Creation & Editing", "생성 및 편집");
    ko.insert("Advanced Tools", "고급 도구");
    ko.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "초보자 모드 활성화: 인터페이스가 간소화되었습니다. 모든 도구를 보려면 보기에서 비활성화하세요.");
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
    pl.insert("Industrial (advanced)", "Przemysłowy (zaawansowany)");
    pl.insert("Modern layout (simple)", "Nowoczesny układ (prosty)");
    pl.insert("Classic layout (expert)", "Klasyczny układ (ekspert)");
    pl.insert("Beginner Mode", "Tryb początkującego");
    pl.insert("Connection & Control", "Połączenie i sterowanie");
    pl.insert("Job Preparation", "Przygotowanie zadania");
    pl.insert("Creation & Editing", "Tworzenie i edycja");
    pl.insert("Advanced Tools", "Zaawansowane narzędzia");
    pl.insert("Beginner mode active: interface simplified. Disable it in View to show all tools.", "Tryb początkującego aktywny: interfejs uproszczony. Wyłącz w Widok, aby zobaczyć wszystkie narzędzia.");
    m.insert(Language::Polish, pl);

    m
});

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
        set_language(Language::French);
        assert_eq!(tr("Connect"), "Connecter");
        assert_eq!(tr("Open"), "Ouvrir");
        // Reset
        set_language(Language::English);
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
            Language::French, Language::Japanese, Language::German,
            Language::Italian, Language::Spanish, Language::Portuguese,
            Language::Arabic, Language::Chinese, Language::Russian,
            Language::Turkish, Language::Korean, Language::Polish,
        ];
        for lang in languages {
            assert!(DICTIONARY.contains_key(&lang), "Missing dictionary for {:?}", lang);
            let map = DICTIONARY.get(&lang).unwrap();
            assert!(map.contains_key("Connect"), "Missing 'Connect' key for {:?}", lang);
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
