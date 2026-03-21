use std::collections::HashMap;

pub fn entries() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("Connect", "Connecter");
    m.insert("Disconnect", "Déconnecter");
    m.insert("Open", "Ouvrir");
    m.insert("Save", "Enregistrer");
    m.insert("Run", "Lancer");
    m.insert("Stop", "Arrêter");
    m.insert("Hold", "Pause");
    m.insert("Resume", "Reprendre");
    m.insert("Home", "Origine");
    m.insert("Unlock", "Débloquer");
    m.insert("Reset", "Réinitialiser");
    m.insert("Settings", "Paramètres");
    m.insert("Machine Profile", "Profil Machine");
    m.insert("Material Library", "Bibliothèque Matériaux");
    m.insert("Preview", "Aperçu");
    m.insert("Console", "Console");
    m.insert("Drawing Tools", "Outils de Dessin");
    m.insert("Job Transformation", "Transformation Job");
    m.insert("Z-Probe", "Sonde Z");
    m.insert("View", "Affichage");
    m.insert("Theme", "Thème");
    m.insert("Layout", "Disposition");
    m.insert("Language", "Langue");
    m.insert("Controller", "Contrôleur");
    m.insert("Modern (recommended)", "Moderne (recommandé)");
    m.insert("Pro (new)", "Pro (nouveau)");
    m.insert("Industrial (advanced)", "Industriel (avancé)");
    m.insert("Modern layout (simple)", "Disposition moderne (simple)");
    m.insert("Classic layout (expert)", "Disposition classique (expert)");
    m.insert(
    "Pro layout (aesthetic & practical)",
    "Disposition Pro (esthétique & pratique)",
    );
    m.insert("Beginner Mode", "Mode débutant");
    m.insert("Connection & Control", "Connexion & Contrôle");
    m.insert("Job Preparation", "Préparation du job");
    m.insert("Creation & Editing", "Création & édition");
    m.insert("Advanced Tools", "Outils avancés");
    m.insert(
    "Beginner mode active: interface simplified. Disable it in View to show all tools.",
    "Mode débutant actif : interface simplifiée. Désactivez-le dans Affichage pour voir tous les outils.",
    );
    m.insert("Cuts", "Coupes");
    m.insert("Move", "Déplacer");
    m.insert("Laser", "Laser");
    m.insert("Layers", "Couches");
    m.insert("Notes", "Notes");
    m.insert("Project Notes", "Notes du projet");
    m.insert("Measure", "Mesurer");
    m.insert("Group", "Grouper");
    m.insert("Ungroup", "Dégrouper");
    m.insert("Copy", "Copier");
    m.insert("Cut", "Couper");
    m.insert("Paste", "Coller");
    m.insert("Duplicate", "Dupliquer");
    m.insert("Select All", "Tout sélectionner");
    m.insert("Air Assist", "Air Assist");
    m.insert("Exhaust Fan", "Ventilation");
    m.insert("Power Ramping", "Rampe de puissance");
    m.insert("Perforation", "Perforation");
    m.insert("Construction Layer", "Couche de construction");
    m.insert("Maintenance", "Maintenance");
    m.insert("Cost Estimate", "Estimation du coût");
    m.insert("Export SVG", "Exporter SVG");
    m.insert("Startup Wizard", "Assistant de démarrage");
    // Toolbar & menus
    m.insert("File", "Fichier");
    m.insert("Edit", "Édition");
    m.insert("Undo", "Annuler");
    m.insert("Redo", "Rétablir");
    m.insert("Zoom In", "Zoom avant");
    m.insert("Zoom Out", "Zoom arrière");
    m.insert("Recent Files", "Fichiers récents");
    m.insert("No recent files", "Aucun fichier récent");
    m.insert("Project", "Projet");
    m.insert("Open Project", "Ouvrir un projet");
    m.insert("Save Project", "Enregistrer le projet");
    m.insert("Export Job Report", "Exporter le rapport");
    m.insert("Frame", "Cadrage");
    m.insert("Dry Run", "Test à vide");
    m.insert("Set Zero", "Définir zéro");
    m.insert("Zero", "Zéro");
    m.insert("Tools", "Outils");
    m.insert("Power/Speed Test", "Test puissance/vitesse");
    m.insert("Test Fire", "Tir d'essai");
    m.insert("GCode Editor", "Éditeur GCode");
    m.insert("Tiling", "Pavage");
    m.insert("Auto Nesting", "Imbrication auto");
    m.insert("Job Queue", "File d'attente");
    m.insert("Shortcuts", "Raccourcis");
    m.insert("Dark UI", "Interface sombre");
    m.insert("Light UI", "Interface claire");
    m.insert("Save Layer Template", "Enregistrer modèle de couche");
    m.insert("Load Layer Template", "Charger modèle de couche");
    m.insert("Help", "Aide");
    m.insert("About", "À propos");
    // Jog panel
    m.insert("Jog Control", "Contrôle de déplacement");
    m.insert("Step:", "Pas :");
    m.insert("Feed:", "Avance :");
    // Preview panel
    m.insert("Rapids", "Rapides");
    m.insert("Fill", "Remplissage");
    m.insert("Risk", "Risque");
    m.insert("Realistic", "Réaliste");
    m.insert("Simulation", "Simulation");
    m.insert("Zoom in", "Zoom avant");
    m.insert("Zoom out", "Zoom arrière");
    m.insert("Fit", "Ajuster");
    // Machine state
    m.insert("Quick Move (Bounds)", "Déplacement rapide (Limites)");
    m.insert("Spindle:", "Broche :");
    // Macros
    m.insert("Macros", "Macros");
    m.insert("New Macro", "Nouvelle macro");
    m.insert("Delete", "Supprimer");
    // Job queue
    m.insert("Pending Queue", "File d'attente");
    m.insert("No queued jobs.", "Aucun job en attente.");
    m.insert("Execution History", "Historique d'exécution");
    m.insert("No history yet.", "Aucun historique.");
    // Alignment
    m.insert("Align:", "Aligner :");
    // Preflight
    m.insert("Launch Anyway", "Lancer quand même");
    m.insert("Cannot launch job with critical errors.", "Impossible de lancer le job avec des erreurs critiques.");
    // Shortcuts dialog
    m.insert("Key", "Touche");
    m.insert("Action", "Action");
    // Generators
    m.insert("Object Generators", "Générateurs d'objets");
    m.insert("QR Code Generator", "Générateur QR Code");
    m.insert("Box Maker (Finger Joints)", "Générateur de boîte (joints à doigts)");
    m.insert("Living Hinge", "Charnière vivante");
    m.insert("Print & Cut Fiducials", "Repères Print & Cut");
    // Drawing
    m.insert("Add Shape", "Ajouter forme");
    m.insert("Clear", "Effacer");
    // Connection
    m.insert("Port:", "Port :");
    m.insert("Baud:", "Débit :");
    m.insert("Refresh", "Actualiser");
    // Camera
    m.insert("Camera", "Caméra");
    m.insert("Live stream active", "Flux vidéo actif");
    // Status bar
    m.insert("Unit Toggle", "Changer unité");
    // Nesting
    m.insert("No selection: fallback to all shapes.", "Aucune sélection : toutes les formes seront utilisées.");
    // General
    m.insert("Cancel", "Annuler");
    m.insert("Apply", "Appliquer");
    m.insert("Close", "Fermer");
    m.insert("Generate", "Générer");
    m.insert("Export", "Exporter");
    m.insert("Import", "Importer");
    // Node editing & selection
    m.insert("Node Editing", "Édition de nœuds");
    m.insert("Selection", "Sélection");
    m.insert("Create", "Créer");
    m.insert("Modify", "Modifier");
    // Drawing tools
    m.insert("Rect", "Rect");
    m.insert("Circle", "Cercle");
    m.insert("Origin X:", "Origine X :");
    m.insert("Radius:", "Rayon :");
    m.insert("Layer:", "Couche :");
    m.insert("Set to Active Layer", "Appliquer au calque actif");
    m.insert("Use the Text Tool panel below to create text paths.", "Utilisez le panneau Outil Texte ci-dessous pour créer des tracés texte.");
    // Text tool
    m.insert("Text Tool", "Outil Texte");
    m.insert("Variable Text (Serial Numbers)", "Texte variable (numéros de série)");
    m.insert("Text:", "Texte :");
    m.insert("Size:", "Taille :");
    m.insert("Source:", "Source :");
    m.insert("Bundled", "Intégrées");
    m.insert("System", "Système");
    m.insert("Font:", "Police :");
    m.insert("Bundled fonts included in project (SIL OFL 1.1, GPLv3-compatible use).", "Polices intégrées au projet (SIL OFL 1.1, utilisation compatible GPLv3).");
    m.insert("Loading font previews...", "Chargement des aperçus de polices…");
    m.insert("No system fonts detected. Use Bundled or File source.", "Aucune police système détectée. Utilisez les polices intégrées ou un fichier.");
    m.insert("Load Font File", "Charger un fichier de police");
    m.insert("Add Text to Drawing", "Ajouter le texte au dessin");
    // Variable text
    m.insert("Serial", "Série");
    m.insert("CSV Column", "Colonne CSV");
    m.insert("Prefix:", "Préfixe :");
    m.insert("Suffix:", "Suffixe :");
    m.insert("Start:", "Début :");
    m.insert("Inc:", "Inc :");
    m.insert("Pad:", "Rembourrage :");
    m.insert("Batch Count:", "Nombre de lots :");
    m.insert("Column:", "Colonne :");
    m.insert("Header row", "Ligne d'en-tête");
    m.insert("Delimiter:", "Délimiteur :");
    m.insert("Load CSV", "Charger CSV");
    // Align / Distribute
    m.insert("Align / Distribute", "Aligner / Distribuer");
    m.insert("Align Left", "Aligner à gauche");
    m.insert("Align Right", "Aligner à droite");
    m.insert("Align Top", "Aligner en haut");
    m.insert("Align Bottom", "Aligner en bas");
    m.insert("Center Horizontal", "Centrer horizontalement");
    m.insert("Center Vertical", "Centrer verticalement");
    m.insert("Distribute H", "Distribuer H");
    m.insert("Distribute V", "Distribuer V");
    // Shape properties
    m.insert("Shape Properties", "Propriétés de la forme");
    m.insert("Select a shape to edit properties.", "Sélectionnez une forme pour modifier ses propriétés.");
    // Session recovery
    m.insert("Session Recovery", "Récupération de session");
    m.insert("A previous session was interrupted. Restore it?", "Une session précédente a été interrompue. La restaurer ?");
    m.insert("Restore", "Restaurer");
    m.insert("Discard", "Ignorer");
    // Preview placeholder
    m.insert("Load a GCode file or draw shapes to preview", "Chargez un fichier GCode ou dessinez des formes pour prévisualiser");
    // Materials
    m.insert("Apply Recommended", "Appliquer les recommandés");
    m.insert("Apply to Active Layer", "Appliquer au calque actif");
    m.insert("Material Presets", "Préréglages matériaux");
    // Cut list headers
    m.insert("Mode", "Mode");
    m.insert("Spd/Pwr", "Vit/Puis");
    m.insert("Out", "Sort");
    // Misc modify buttons
    m.insert("Array", "Réseau");
    m.insert("Grid", "Grille");
    m.insert("Offset", "Décalage");
    m.insert("Boolean", "Booléen");
    m.insert("Circular Array", "Réseau circulaire");
    m.insert("Grid Array", "Réseau en grille");
    m.insert("Offset Path", "Décalage du tracé");
    m.insert("Boolean Operations", "Opérations booléennes");
    // Cut list extra
    m.insert("View", "Affichage");
    // Font source "File" tab (distinct from menu "File"→"Fichier")
    m.insert("File", "Fichier");
    // UI improvement keys
    m.insert("Connection", "Connexion");
    m.insert("W:", "L :");
    m.insert("H:", "H :");
    m.insert("Rotation:", "Rotation :");
    m.insert("Outline Settings", "Réglages de contour");
    m.insert("Dist:", "Dist :");
    m.insert("Join:", "Jointure :");
    m.insert("Create Outline (Cut)", "Créer contour (coupe)");
    m.insert("Optimize Path", "Optimiser le trajet");
    m.insert("Offset X:", "Décalage X :");
    m.insert("Y:", "Y :");
    m.insert("Center Job", "Centrer le job");
    m.insert("Preflight Check", "Vérification pré-lancement");
    m.insert("Run Z-Probe", "Lancer sonde Z");
    m.insert("Focus Point", "Point de mise au point");
    m.insert("Preflight QA", "Contrôle pré-lancement");
    m.insert("Run Preflight", "Lancer le contrôle");
    m.insert("Block critical issues", "Bloquer les problèmes critiques");
    m.insert("No preflight report yet.", "Aucun rapport de contrôle.");
    m.insert("No preflight issues detected.", "Aucun problème détecté.");
    m.insert("Job Complete", "Job terminé");
    m.insert("Program finished successfully!", "Programme terminé avec succès !");
    m.insert("Sound", "Son");
    m.insert("items selected", "éléments sélectionnés");
    m.insert("Line", "Ligne");
    m.insert("Fill+Line", "Rempli+Ligne");
    m.insert("Enable/disable layer output", "Activer/désactiver la sortie du calque");
    m.insert("Increase feed override", "Augmenter le dépassement d'avance");
    m.insert("Decrease feed override", "Diminuer le dépassement d'avance");
    m.insert("Increase laser power override", "Augmenter le dépassement de puissance laser");
    m.insert("Decrease laser power override", "Diminuer le dépassement de puissance laser");
    m.insert("Toggle mm / inches", "Basculer mm / pouces");
    m.insert("Toggle mm/min / mm/s", "Basculer mm/min / mm/s");
    m.insert("Reset simulation", "Réinitialiser la simulation");
    m.insert("Est. Time:", "Temps estimé :");
    m.insert("Add notes about this project...", "Ajouter des notes sur ce projet…");
    m.insert("Feed override", "Dépassement d'avance");
    m.insert("Rapid override 100%", "Dépassement rapide 100%");
    m.insert("Rapid override 25%", "Dépassement rapide 25%");
    m.insert("layer(s) in Cuts", "calque(s) dans Coupes");
    m.insert("Move X:", "Déplacer X :");
    m.insert("Move Y:", "Déplacer Y :");
    m.insert("Set Layer:", "Définir calque :");
    m.insert("Delete Selected", "Supprimer la sélection");
    // Cut Settings
    m.insert("Cut Settings", "Réglages de coupe");
    m.insert("Speed", "Vitesse");
    m.insert("Travel speed of the laser head", "Vitesse de déplacement de la tête laser");
    m.insert("Max Power (%):", "Puissance max (%) :");
    m.insert("Laser power (0-100%)", "Puissance laser (0-100%)");
    m.insert("Output Mode:", "Mode de sortie :");
    m.insert("Output mode description", "Ligne = coupe vectorielle. Remplissage = balayage raster. Décalage = remplissage concentrique.");
    m.insert("Line (Cut)", "Ligne (Coupe)");
    m.insert("Fill (Scan)", "Remplissage (Balayage)");
    m.insert("Fill + Line", "Remplissage + Ligne");
    m.insert("Offset Fill", "Remplissage décalé");
    m.insert("Fill Interval (mm):", "Intervalle de remplissage (mm) :");
    m.insert("Distance between scan lines", "Distance entre les lignes de balayage");
    m.insert("Min Power (%):", "Puissance min (%) :");
    m.insert("Power at accel/decel", "Puissance en accélération/décélération");
    m.insert("Bidirectional Scan:", "Balayage bidirectionnel :");
    m.insert("Scan both directions", "Balayer dans les deux sens");
    m.insert("Overscan (mm):", "Sur-balayage (mm) :");
    m.insert("Extra deceleration travel", "Course de décélération supplémentaire");
    m.insert("Fill Angle (°):", "Angle de remplissage (°) :");
    m.insert("Output Order:", "Ordre de sortie :");
    m.insert("Lead-In (mm):", "Entrée (mm) :");
    m.insert("Approach distance before cut", "Distance d'approche avant la coupe");
    m.insert("Lead-Out (mm):", "Sortie (mm) :");
    m.insert("Exit distance after cut", "Distance de sortie après la coupe");
    m.insert("Kerf Offset (mm):", "Décalage kerf (mm) :");
    m.insert("Beam width compensation", "Compensation de largeur de faisceau");
    m.insert("Passes:", "Passes :");
    m.insert("Z Offset (mm):", "Décalage Z (mm) :");
    m.insert("Tabs / Bridges:", "Ponts / Attaches :");
    m.insert("Enabled", "Activé");
    m.insert("Tab Spacing:", "Espacement des ponts :");
    m.insert("Tab Size (Gap):", "Taille du pont (Gap) :");
    m.insert("Pass offset (mm):", "Décalage par passe (mm) :");
    m.insert("Ramp length:", "Longueur de rampe :");
    m.insert("Start/end power %:", "Puissance début/fin % :");
    m.insert("Perforation / Dashed Mode", "Perforation / Mode pointillé");
    m.insert("Cut length:", "Longueur de coupe :");
    m.insert("Gap length:", "Longueur d'espace :");
    m.insert("Corner Power Reduction", "Réduction de puissance en virage");
    m.insert("Corner power %:", "Puissance en virage % :");
    m.insert("Angle threshold:", "Seuil d'angle :");
    m.insert("Air Assist (M8)", "Assistance air (M8)");
    m.insert("Exhaust Fan (M7)", "Ventilateur d'extraction (M7)");
    m.insert("Post-delay:", "Post-délai :");
    m.insert("Output Enabled", "Sortie activée");
    m.insert("Cut a square with known nominal size, then enter measured result.", "Coupez un carré de taille nominale connue, puis entrez le résultat mesuré.");
    m.insert("Nominal (mm):", "Nominal (mm) :");
    m.insert("Measured (mm):", "Mesuré (mm) :");
    m.insert("Recommended kerf", "Kerf recommandé");
    m.insert("Apply to Kerf Offset", "Appliquer au décalage kerf");
    m.insert("Parameter Snapshot", "Instantané de paramètres");
    m.insert("Take Snapshot", "Prendre un instantané");
    m.insert("Save current parameters for comparison", "Sauvegarder les paramètres actuels pour comparaison");
    m.insert("Hide Compare", "Masquer la comparaison");
    m.insert("Show Compare", "Afficher la comparaison");
    m.insert("Current → Snapshot", "Actuel → Instantané");
    m.insert("OK", "OK");
    m.insert("No layer selected.", "Aucun calque sélectionné.");
    // Image dialog
    m.insert("No preview available", "Aucun aperçu disponible");
    m.insert("Bitmap Import Mode:", "Mode d'import bitmap :");
    m.insert("Raster", "Raster");
    m.insert("Vectorize (Stencil)", "Vectoriser (Pochoir)");
    m.insert("Vectorize Settings", "Réglages de vectorisation");
    m.insert("Raster / Photo Settings", "Réglages raster / photo");
    m.insert("Resolution:", "Résolution :");
    m.insert("DPI", "DPI");
    m.insert("Image Adjustments:", "Ajustements d'image :");
    m.insert("Threshold", "Seuil");
    m.insert("Smoothing", "Lissage");
    m.insert("Contrast", "Contraste");
    m.insert("Centerline / Skeleton", "Ligne médiane / Squelette");
    m.insert("Use thinning algorithm for centerline tracing", "Utiliser un algorithme d'amincissement pour le tracé médian");
    m.insert("Brightness", "Luminosité");
    m.insert("Dithering:", "Tramage :");
    m.insert("Grayscale", "Niveaux de gris");
    m.insert("Flip H", "Miroir H");
    m.insert("Flip V", "Miroir V");
    m.insert("Rotation", "Rotation");
    m.insert("Laser Settings:", "Réglages laser :");
    m.insert("Max Power (%)", "Puissance max (%)");
    m.insert("Cutting Frame (Outline)", "Cadre de découpe (Contour)");
    m.insert("Cut Speed", "Vitesse de coupe");
    m.insert("Cut Power (%)", "Puissance de coupe (%)");
    m.insert("Vector / SVG Settings", "Réglages vectoriel / SVG");
    m.insert("Scaling:", "Mise à l'échelle :");
    m.insert("Scale X", "Échelle X");
    m.insert("Layers / Color Mapping:", "Calques / Mappage couleur :");
    m.insert("Speed:", "Vitesse :");
    m.insert("Power (%):", "Puissance (%) :");
    // Tiling
    m.insert("Tiling — Repeat Job", "Pavage — Répéter le job");
    m.insert("Columns:", "Colonnes :");
    m.insert("Rows:", "Lignes :");
    m.insert("Spacing X (mm):", "Espacement X (mm) :");
    m.insert("Spacing Y (mm):", "Espacement Y (mm) :");
    m.insert("copies total", "copies au total");
    m.insert("Apply Tiling", "Appliquer le pavage");
    // Boolean ops
    m.insert("Union (Combine)", "Union (Combiner)");
    m.insert("Subtract (A - B)", "Soustraire (A - B)");
    m.insert("Intersection", "Intersection");
    m.insert("XOR (Symmetric Difference)", "XOR (Différence symétrique)");
    // Settings dialog
    m.insert("GRBL Settings", "Réglages GRBL");
    m.insert("Machine Firmware Settings", "Réglages firmware machine");
    m.insert("ID", "ID");
    m.insert("Value", "Valeur");
    m.insert("Description", "Description");
    m.insert("Waiting for settings...", "En attente des réglages…");
    m.insert("Save to Board", "Sauvegarder sur la carte");
    // Alignment
    m.insert("Align: (Select objects)", "Aligner : (Sélectionnez des objets)");
    m.insert("Align Center (H)", "Centrer horizontalement");
    m.insert("Align Center (V)", "Centrer verticalement");
    // Job queue
    m.insert("Queue Current Job", "Ajouter le job actuel");
    m.insert("Start Next", "Lancer le suivant");
    m.insert("Retry Last Failed", "Réessayer le dernier échec");
    m.insert("Auto-run next queued job", "Lancer auto le prochain job");
    m.insert("Batch Import", "Import par lot");
    m.insert("Load multiple GCode files into the queue", "Charger plusieurs fichiers GCode dans la file");
    m.insert("Save History", "Sauvegarder l'historique");
    m.insert("Save job history to disk", "Sauvegarder l'historique des jobs sur disque");
    m.insert("Load History", "Charger l'historique");
    m.insert("Restore saved job history", "Restaurer l'historique sauvegardé");
    // Macros
    m.insert("Name:", "Nom :");
    m.insert("GCode (multiline):", "GCode (multiligne) :");
    m.insert("Add at least one executable G-code line.", "Ajoutez au moins une ligne G-code exécutable.");
    // About / Update
    m.insert("About All4Laser", "À propos de All4Laser");
    m.insert("Advanced Laser Control Software", "Logiciel avancé de contrôle laser");
    m.insert("Update Available!", "Mise à jour disponible !");
    m.insert("A new version is available", "Une nouvelle version est disponible");
    m.insert("Download", "Télécharger");
    m.insert("Dismiss", "Ignorer");
    m
}
