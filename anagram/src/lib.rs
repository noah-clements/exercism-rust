use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for anagram in possible_anagrams {
        if is_anagram(word, anagram) {
            anagrams.insert(anagram.clone());
        }
    }
    anagrams
}

pub fn is_anagram(word: &str, anagram: &str) -> bool {
    let wd = word.to_lowercase();
    let ag = anagram.to_lowercase();
    if wd == ag {
        return false;
    }
    let mut w_chars: Vec<char> = wd.chars().collect();
    let mut a_chars: Vec<char> = ag.chars().collect();

    w_chars.sort_unstable();
    a_chars.sort_unstable();
    let w_str: String = w_chars.into_iter().collect();
    let a_str: String = a_chars.into_iter().collect();
    w_str.eq(&a_str)
}
