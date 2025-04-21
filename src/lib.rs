mod language;

pub fn syllable_break(text: &str, separator: Option<&str>) -> String {
    language::syllable_break(text, separator,  language::SyllableBreakType::Morpheme)
}

pub fn syllable_break_phoneme(text: &str, separator: Option<&str>) -> String {
    language::syllable_break(text,separator, language::SyllableBreakType::Phoneme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_syllable_break() {
        let result = syllable_break("မင်္ဂလာပါ", Some("|"));
        assert_eq!(result, "မင်္ဂ|လာ|ပါ");
    }

    #[test]
    fn it_works_with_syllable_break_phoeme() {
        let result = syllable_break_phoneme("မင်္ဂလာပါ", Some("|"));
        assert_eq!(result, "မင်|ဂ|လာ|ပါ");
    }
}
