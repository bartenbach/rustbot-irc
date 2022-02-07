use hyphenation::{Hyphenator, Standard};
use hyphenation::Iter;
extern crate rand;
use rand::{thread_rng,Rng};

pub fn get_buttified_sentence(input: String, chance: usize) -> String {
    let hyphenator = crate::load_dict().unwrap();
    let mut rng = thread_rng();
    let mut split: Vec<String> = input
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();
    for e in split.iter_mut() {
        if rng.gen_range(0..chance) == 0 {
            let result = get_buttified_word(&hyphenator, e);
            *e = result;
        }
    }
    return split.join(" ");
}

fn get_buttified_word(hyphenator: &Standard, input: &str) -> String {
    let mut rng = thread_rng();
    let hyphenated = hyphenator.hyphenate(&input);
    let mut syllables: Vec<&str> = hyphenated.iter().segments().collect();
    let i: usize = rng.gen_range(0..syllables.len());
    syllables[i] = "butt";
    return syllables.concat();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_buttified_word_butts_words() {
        let hyphenator = crate::load_dict().unwrap();
        let test_cases = vec!("propaganda", "delerious", "delicious", "inherent");

        for word in test_cases {
            let result = get_buttified_word(&hyphenator, word);
            println!("{}", result);
            assert_eq!(result.contains("butt"), true);
        }
    }

    #[test]
    fn test_get_buttified_sentence_butts_sentence() {
        let test_cases = vec!(
            "this is a test sentence",
            "this is another test sentence",
            "this is yet ANOTHER test sentence that is slightly longer!",
            "this is just the most appallingly long test sentence that has ever been created and quite frankly the thought of it just makes me sick to my stomach that anyone could make a sentence so long"
        );

        for sentence in test_cases {
            let result = get_buttified_sentence(sentence.to_string(), 1);
            println!("{}", result);
            assert_eq!(result.contains("butt"), true);
        }
    }
}
