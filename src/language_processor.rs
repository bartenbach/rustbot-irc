use hyphenation::{Hyphenator, Standard};
use hyphenation::Iter;

pub fn get_hyphenation(hyphenator: &Standard, input: String) -> usize {
    return hyphenator.hyphenate(&input).iter().collect::<Vec<String>>().len() + 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;


    #[test]
    fn test_get_syllablle_count() {
        let hyphenator = crate::load_dict().unwrap();
        let mut test_cases: HashMap<String, usize> = HashMap::new();
        test_cases.insert(
            "propaganda".to_string(), 4
        );

        for (word, syllables) in test_cases {
            assert_eq!(get_hyphenation(&hyphenator, word), syllables);
        }
    }
}
