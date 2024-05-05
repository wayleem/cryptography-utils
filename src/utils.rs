fn extract_substrings(text: &str) -> Substrings {
    let cleaned_text = text.replace(" ", "");
    let mut fourth = String::new();
    let mut fifth = String::new();
    let mut sixth = String::new();

    for (i, ch) in cleaned_text.chars().enumerate() {
        if (i + 1) % 4 == 0 {
            fourth.push(ch);
        }
        if (i + 1) % 5 == 0 {
            fifth.push(ch);
        }
        if (i + 1) % 6 == 0 {
            sixth.push(ch);
        }
    }

    Substrings { fourth, fifth, sixth }
}

fn calculate_letter_frequency(input_string: &str) -> HashMap<char, i32> {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    for ch in input_string.chars().filter(|c| c.is_alphabetic() && c.is_uppercase()) {
        *frequency.entry(ch).or_insert(0) += 1;
    }
    frequency
}
