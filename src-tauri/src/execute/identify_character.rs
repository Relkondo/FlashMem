pub fn get_char_identification_function(lang: &str) -> fn(char) -> bool {
    match lang {
        "Chinese" | "Chinese Traditional" => is_a_chinese_character,
        "Japanese" => is_a_japanese_character,
        "Korean" => is_a_korean_character,
        "Russian" | "Ukrainian" => is_a_cyrillic_character,
        "Greek" => is_a_greek_character,
        "Arabic" => is_a_arabic_character,
        "Hebrew" => is_a_hebrew_character,
        "Hindi" => is_a_hindi_character,
        "Bengali" => is_a_bengali_character,
        _ => is_an_extended_latin_character
    }
}
fn is_a_chinese_character(c: char) -> bool {
    (c >= '\u{4E00}' && c <= '\u{9FFF}') ||
        (c >= '\u{3400}' && c <= '\u{4DBF}') ||
        (c >= '\u{F900}' && c <= '\u{FAFF}')
}

fn is_a_japanese_character(c: char) -> bool {
    (c >= '\u{3040}' && c <= '\u{30FF}') ||
        is_a_chinese_character(c)
}

fn is_a_korean_character(c: char) -> bool {
    c >= '\u{AC00}' && c <= '\u{D7AF}'
}

fn is_a_cyrillic_character(c: char) -> bool {
    c >= '\u{0400}' && c <= '\u{052F}'
}

fn is_a_greek_character(c: char) -> bool {
    c >= '\u{0370}' && c <= '\u{03FF}' ||
    c >= '\u{1F00}' && c <= '\u{1FFF}'
}

fn is_a_arabic_character(c: char) -> bool {
    c >= '\u{0600}' && c <= '\u{06FF}' ||
        c >= '\u{0750}' && c <= '\u{077F}'
}

fn is_a_hebrew_character(c: char) -> bool {
    c >= '\u{0590}' && c <= '\u{05FF}'
}

fn is_a_hindi_character(c: char) -> bool {
    c >= '\u{0900}' && c <= '\u{097F}'
}

fn is_a_bengali_character(c: char) -> bool {
    c >= '\u{0980}' && c <= '\u{09FF}'
}

fn is_an_extended_latin_character(c: char) -> bool {
    c >= '\u{0041}' && c <= '\u{005A}' ||
        c >= '\u{0061}' && c <= '\u{007A}' ||
        c >= '\u{00C0}' && c <= '\u{024F}' ||
        c >= '\u{1E00}' && c <= '\u{1EFF}'
}