use fancy_regex::Regex;

pub enum SyllableBreakType {
    /// Phoneme-based syllable breaking (applies patsint exploding)
    Phoneme,
    /// Morpheme-based syllable breaking (preserves original morphemes)
    Morpheme,
}

fn create_break_pattern() -> Regex {
    let my_consonant = "က-အ";
    let en_char = "a-zA-Z0-9";
    let other_char = "ဣဤဥဦဧဩဪဿ၌၍၏၀-၉၊။\\!-\\/\\:-@\\[-`\\{-~\\s";
    let subscript_symbol = "္";
    let a_that = "်";

    let pattern = format!(
        r"((?<!{subscript_symbol})[{my_consonant}](?![{a_that}{subscript_symbol}])|[{en_char}{other_char}])"
    );

    match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(err) => {
            eprintln!("Regex error: {} with pattern: {}", err, pattern);
            panic!("Invalid regex pattern");
        }
    }
}

fn explode_patsint(test: &str) -> String {
    let patterns = [(r"([က-အ])္([က-အ])", "$1်$2"), (r"([က-အ]်?)္([က-အ])", "$1$2")];

    let mut modified_text = test.to_string();

    // Apply all patterns sequentially
    for (pattern, replacement) in patterns.iter() {
        match Regex::new(pattern) {
            Ok(regex) => {
                modified_text = regex.replace_all(&modified_text, *replacement).to_string();
            }
            Err(err) => {
                eprintln!("Regex error: {} with pattern: {}", err, pattern);
                // Continue with next pattern instead of panicking
            }
        }
    }
    return modified_text;
}

fn break_syllables(line: &str, break_pattern: &Regex, separator: &str) -> String {
    // Trim and normalize whitespace
    let line = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    // Apply the break pattern
    let segmented_line = break_pattern.replace_all(&line, format!("{}{}", separator, "$1"));

    // Remove leading delimiter if it exists
    let segmented_line = if segmented_line.starts_with(separator) {
        segmented_line[separator.len()..].to_string()
    } else {
        segmented_line.to_string()
    };

    // Replace delimiter+space+delimiter with a single space
    let double_delimiter = format!("{}{}", separator, separator);
    let segmented_line = segmented_line.replace(&double_delimiter, " ");

    segmented_line
}

pub fn syllable_break(text: &str, separator: Option<&str>, break_type: SyllableBreakType ) -> String {
    let break_pattern = create_break_pattern();
    let separator = separator.unwrap_or("|");

    // Apply explode_patsint only for Phoneme breaking
    let input = match break_type {
        SyllableBreakType::Phoneme => explode_patsint(text),
        SyllableBreakType::Morpheme => text.to_string(),
    };

    break_syllables(&input, &break_pattern, separator)
}
