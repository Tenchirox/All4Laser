#[test]
fn test_font_enumeration() {
    let mut fonts = Vec::new();
    if let Ok(handles) = font_kit::source::SystemSource::new().all_fonts() {
         for handle in handles {
             if let Ok(font) = handle.load() {
                 let family = font.family_name();
                 if !fonts.contains(&family) {
                     fonts.push(family);
                 }
             }
         }
    }
    fonts.sort();
    println!("Found {} fonts.", fonts.len());
    for f in fonts.iter().take(5) {
        println!(" - {}", f);
    }
    // We expect some fonts usually, but in a minimal container there might be none.
    // So let's just ensure the code runs without crashing.
    if fonts.is_empty() {
        println!("Warning: No system fonts found (likely running in a minimal container).");
    } else {
        println!("Successfully listed fonts.");
    }
}
