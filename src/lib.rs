mod language;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn syllable_break(text: &str) -> String {
    language::syllable_break(text, language::SyllableBreakType::Morpheme)
}

pub fn syllable_break_phoneme(text: &str) -> String {
    language::syllable_break(text, language::SyllableBreakType::Phoneme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_with_syllable_break() {
        let result = syllable_break("မင်္ဂလာပါ");
        assert_eq!(result, " မင်္ဂ| လာ| ပါ");
    }

    #[test]
    fn it_works_with_syllable_break_phoeme() {
        let result = syllable_break_phoneme("မင်္ဂလာပါ");
        assert_eq!(result, " မင်| ဂ| လာ| ပါ");
    }
}
