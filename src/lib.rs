use regex::Regex;
use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(_n: u32) -> Vec<u32> {
    if _n == 0 {
        return vec![];
    }
    if _n == 1 {
        return vec![0];
    }

    let mut result = vec![0, 1];
    for i in 2.._n {
        let next = result[(i - 1) as usize] + result[(i - 2) as usize];
        result.push(next);
    }
    result
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(n: usize, a: &[i32]) -> Option<i32> {
    if a.is_empty() || n >= a.len() {
        return None;
    }

    let mut sorted = a.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    Some(sorted[n])
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut max_char = s.chars().next().unwrap();
    let mut max_count = 0;

    for (c, count) in counts.iter() {
        if *count > max_count {
            max_count = *count;
            max_char = *c;
        }
    }

    max_char.to_string()
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(arr1: &[String], arr2: &[String]) -> Option<HashMap<String, String>> {
    if arr1.len() != arr2.len() {
        return None;
    }

    let mut map = HashMap::new();
    for i in 0..arr1.len() {
        map.insert(arr1[i].clone(), arr2[i].clone());
    }

    Some(map)
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> =
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        // You may also use `Self::default()`
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    pub fn add(&mut self, name: String, number: String, is_listed: bool) -> bool {
        // Check if name already exists
        for entry in &self.entries {
            if entry.name == name {
                return false;
            }
        }

        // Validate phone number format: NNN-NNN-NNNN
        if !Self::is_valid_number(&number) {
            return false;
        }

        // Check listed number rules
        if is_listed {
            // Check if this number already exists as listed
            for entry in &self.entries {
                if entry.number == number && entry.is_listed {
                    return false;
                }
            }
        }

        // Add the entry
        self.entries.push(PhoneEntry {
            name,
            number,
            is_listed,
        });

        true
    }

    /// Helper function to validate phone number format
    fn is_valid_number(number: &str) -> bool {
        let re = Regex::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();
        return re.is_match(number);
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, name: &str) -> Option<String> {
        for entry in &self.entries {
            if entry.name == name && entry.is_listed {
                return Some(entry.number.clone());
            }
        }
        None
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, num: &str) -> Option<String> {
        for entry in &self.entries {
            if entry.number == num && entry.is_listed {
                return Some(entry.name.clone());
            }
        }
        None
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, areacode: &str) -> Vec<String> {
        let mut names = Vec::new();
        for entry in &self.entries {
            if entry.number.starts_with(areacode) {
                names.push(entry.name.clone());
            }
        }
        names
    }
}
