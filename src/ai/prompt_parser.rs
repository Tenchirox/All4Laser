//! Minimal NLP-lite prompt parser for the embedded AI shape generator.
//! Extracts subjects, modifiers and spatial relationships from FR+EN text.

#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum Subject {
    Bird,
    Eagle,
    Cat,
    Dog,
    Fish,
    Butterfly,
    Horse,
    Tree,
    Flower,
    Star,
    Moon,
    Sun,
    Mountain,
    Leaf,
    Heart,
    House,
    Gear,
    Arrow,
    Key,
    Crown,
    Snowflake,
    Spiral,
    Diamond,
    Shield,
    Anchor,
    Lightning,
    Skull,
    Paw,
    Music,
    Flame,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    Big,
    Small,
    Detailed,
    Simple,
    Symmetric,
    Dense,
    Rounded,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Relation {
    On,       // "on", "sur"
    With,     // "with", "avec"
    And,      // "and", "et"
    Inside,   // "inside", "dans"
    Around,   // "around", "autour"
}

#[derive(Clone, Debug)]
pub struct SceneElement {
    pub subject: Subject,
    pub modifiers: Vec<Modifier>,
}

#[derive(Clone, Debug)]
pub struct PromptAnalysis {
    pub elements: Vec<(SceneElement, Option<Relation>)>,
    pub global_modifiers: Vec<Modifier>,
}

struct DictEntry {
    keywords: &'static [&'static str],
    subject: Subject,
}

const DICTIONARY: &[DictEntry] = &[
    DictEntry { keywords: &["eagle", "aigle"], subject: Subject::Eagle },
    DictEntry { keywords: &["bird", "oiseau"], subject: Subject::Bird },
    DictEntry { keywords: &["cat", "chat", "kitten", "chaton"], subject: Subject::Cat },
    DictEntry { keywords: &["dog", "chien", "puppy", "chiot"], subject: Subject::Dog },
    DictEntry { keywords: &["fish", "poisson"], subject: Subject::Fish },
    DictEntry { keywords: &["butterfly", "papillon"], subject: Subject::Butterfly },
    DictEntry { keywords: &["horse", "cheval"], subject: Subject::Horse },
    DictEntry { keywords: &["tree", "arbre", "branche", "branch"], subject: Subject::Tree },
    DictEntry { keywords: &["flower", "fleur"], subject: Subject::Flower },
    DictEntry { keywords: &["star", "etoile", "étoile"], subject: Subject::Star },
    DictEntry { keywords: &["moon", "lune"], subject: Subject::Moon },
    DictEntry { keywords: &["sun", "soleil"], subject: Subject::Sun },
    DictEntry { keywords: &["mountain", "montagne"], subject: Subject::Mountain },
    DictEntry { keywords: &["leaf", "feuille"], subject: Subject::Leaf },
    DictEntry { keywords: &["heart", "coeur", "cœur"], subject: Subject::Heart },
    DictEntry { keywords: &["house", "maison"], subject: Subject::House },
    DictEntry { keywords: &["gear", "engrenage", "rouage"], subject: Subject::Gear },
    DictEntry { keywords: &["arrow", "fleche", "flèche"], subject: Subject::Arrow },
    DictEntry { keywords: &["key", "cle", "clé", "clef"], subject: Subject::Key },
    DictEntry { keywords: &["crown", "couronne"], subject: Subject::Crown },
    DictEntry { keywords: &["snowflake", "flocon"], subject: Subject::Snowflake },
    DictEntry { keywords: &["spiral", "spirale"], subject: Subject::Spiral },
    DictEntry { keywords: &["diamond", "diamant", "losange"], subject: Subject::Diamond },
    DictEntry { keywords: &["shield", "bouclier", "blason"], subject: Subject::Shield },
    DictEntry { keywords: &["anchor", "ancre"], subject: Subject::Anchor },
    DictEntry { keywords: &["lightning", "eclair", "éclair", "foudre"], subject: Subject::Lightning },
    DictEntry { keywords: &["skull", "crane", "crâne", "tete de mort", "tête de mort"], subject: Subject::Skull },
    DictEntry { keywords: &["paw", "patte", "empreinte"], subject: Subject::Paw },
    DictEntry { keywords: &["music", "musique", "note"], subject: Subject::Music },
    DictEntry { keywords: &["flame", "flamme", "feu", "fire"], subject: Subject::Flame },
];

struct ModEntry {
    keywords: &'static [&'static str],
    modifier: Modifier,
}

const MODIFIERS: &[ModEntry] = &[
    ModEntry { keywords: &["big", "large", "grand", "gros"], modifier: Modifier::Big },
    ModEntry { keywords: &["small", "petit", "tiny", "mini"], modifier: Modifier::Small },
    ModEntry { keywords: &["detailed", "détaillé", "detaille", "complex"], modifier: Modifier::Detailed },
    ModEntry { keywords: &["simple", "minimal", "minimaliste"], modifier: Modifier::Simple },
    ModEntry { keywords: &["symmetric", "symétrique", "symetrique"], modifier: Modifier::Symmetric },
    ModEntry { keywords: &["dense", "plein", "filled"], modifier: Modifier::Dense },
    ModEntry { keywords: &["rounded", "arrondi", "round", "rond"], modifier: Modifier::Rounded },
];

struct RelEntry {
    keywords: &'static [&'static str],
    relation: Relation,
}

const RELATIONS: &[RelEntry] = &[
    RelEntry { keywords: &["on", "sur", "on top"], relation: Relation::On },
    RelEntry { keywords: &["with", "avec"], relation: Relation::With },
    RelEntry { keywords: &["and", "et", "&"], relation: Relation::And },
    RelEntry { keywords: &["inside", "dans", "in", "within"], relation: Relation::Inside },
    RelEntry { keywords: &["around", "autour"], relation: Relation::Around },
];

pub fn parse_prompt(prompt: &str) -> PromptAnalysis {
    let lower = prompt.to_ascii_lowercase();
    let lower = lower
        .replace(',', " ")
        .replace('/', " ")
        .replace("’", " ");

    // 1. Extract global modifiers
    let mut global_mods = Vec::new();
    for me in MODIFIERS {
        for kw in me.keywords {
            if lower.contains(kw) {
                if !global_mods.contains(&me.modifier) {
                    global_mods.push(me.modifier.clone());
                }
            }
        }
    }

    // 2. Find subjects in order of appearance
    let mut found: Vec<(usize, Subject)> = Vec::new();
    for entry in DICTIONARY {
        for kw in entry.keywords {
            if let Some(pos) = lower.find(kw) {
                // Avoid duplicating if eagle also matches bird
                let dominated = found.iter().any(|(p, _)| (*p as i32 - pos as i32).unsigned_abs() < 3);
                if !dominated {
                    found.push((pos, entry.subject.clone()));
                }
            }
        }
    }
    found.sort_by_key(|(pos, _)| *pos);
    // Deduplicate overlapping matches (keep first at each position)
    found.dedup_by(|a, b| a.0 == b.0);

    // 3. Find relations between subjects
    let mut elements: Vec<(SceneElement, Option<Relation>)> = Vec::new();
    for (i, (_pos, subj)) in found.iter().enumerate() {
        let elem = SceneElement {
            subject: subj.clone(),
            modifiers: global_mods.clone(),
        };
        // Look for relation between this subject and the next
        let rel = if i + 1 < found.len() {
            let next_pos = found[i + 1].0;
            let between = &lower[_pos.saturating_add(1)..next_pos.min(lower.len())];
            find_relation(between)
        } else {
            None
        };
        elements.push((elem, rel));
    }

    // If no subject found, add a star as default
    if elements.is_empty() {
        elements.push((
            SceneElement {
                subject: Subject::Star,
                modifiers: global_mods.clone(),
            },
            None,
        ));
    }

    PromptAnalysis {
        elements,
        global_modifiers: global_mods,
    }
}

fn find_relation(text: &str) -> Option<Relation> {
    // Check multi-word first, then single-word
    for re in RELATIONS {
        for kw in re.keywords {
            if text.contains(kw) {
                return Some(re.relation.clone());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_eagle_on_branch() {
        let a = parse_prompt("aigle sur une branche");
        assert!(a.elements.len() >= 2);
        assert_eq!(a.elements[0].0.subject, Subject::Eagle);
        assert_eq!(a.elements[0].1, Some(Relation::On));
        assert_eq!(a.elements[1].0.subject, Subject::Tree);
    }

    #[test]
    fn parse_heart_with_star() {
        let a = parse_prompt("heart with star");
        assert!(a.elements.len() >= 2);
        assert_eq!(a.elements[0].0.subject, Subject::Heart);
        assert_eq!(a.elements[1].0.subject, Subject::Star);
    }

    #[test]
    fn parse_unknown_gives_default() {
        let a = parse_prompt("xyz");
        assert_eq!(a.elements.len(), 1);
        assert_eq!(a.elements[0].0.subject, Subject::Star);
    }

    #[test]
    fn parse_french_cat() {
        let a = parse_prompt("un chat simple");
        assert_eq!(a.elements[0].0.subject, Subject::Cat);
        assert!(a.global_modifiers.contains(&Modifier::Simple));
    }
}
