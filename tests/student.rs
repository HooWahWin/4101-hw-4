use std::collections::HashMap;
use homework4::*;

/// Test fib with larger values to ensure correctness
#[test]
fn test_fib_larger_values() {
    let result = fib(20);
    assert_eq!(result.len(), 20);
    assert_eq!(result[19], 4181);
    assert_eq!(result[18], 2584);
    
    // Verify consecutive Fibonacci property
    for i in 2..result.len() {
        assert_eq!(result[i], result[i-1] + result[i-2]);
    }
}

/// Test fib edge case with n=1
#[test]
fn test_fib_single_element() {
    assert_eq!(fib(1), vec![0]);
}

/// Test is_palindrome with multi-digit palindromes
#[test]
fn test_is_palindrome_large_numbers() {
    assert!(is_palindrome(12321));
    assert!(is_palindrome(9009));
    assert!(is_palindrome(123454321));
    assert!(!is_palindrome(123456));
    assert!(is_palindrome(1000001));
}

/// Test is_palindrome with single digit numbers
#[test]
fn test_is_palindrome_single_digits() {
    for i in 0..=9 {
        assert!(is_palindrome(i));
    }
}

/// Test is_palindrome with two-digit non-palindromes
#[test]
fn test_is_palindrome_two_digits() {
    assert!(is_palindrome(11));
    assert!(is_palindrome(22));
    assert!(is_palindrome(99));
    assert!(!is_palindrome(12));
    assert!(!is_palindrome(10));
}

/// Test nthmax with negative numbers
#[test]
fn test_nthmax_negative_numbers() {
    assert_eq!(nthmax(0, &[-1, -5, -3, -2]), Some(-1));
    assert_eq!(nthmax(1, &[-1, -5, -3, -2]), Some(-2));
    assert_eq!(nthmax(3, &[-1, -5, -3, -2]), Some(-5));
}

/// Test nthmax with mixed positive and negative
#[test]
fn test_nthmax_mixed_numbers() {
    assert_eq!(nthmax(0, &[5, -3, 0, -7, 2]), Some(5));
    assert_eq!(nthmax(2, &[5, -3, 0, -7, 2]), Some(0));
    assert_eq!(nthmax(4, &[5, -3, 0, -7, 2]), Some(-7));
}

/// Test nthmax with single element array
#[test]
fn test_nthmax_single_element() {
    assert_eq!(nthmax(0, &[42]), Some(42));
    assert_eq!(nthmax(1, &[42]), None);
}

/// Test nthmax with all same values
#[test]
fn test_nthmax_all_same() {
    let arr = [7, 7, 7, 7];
    assert_eq!(nthmax(0, &arr), Some(7));
    assert_eq!(nthmax(1, &arr), Some(7));
    assert_eq!(nthmax(3, &arr), Some(7));
    assert_eq!(nthmax(4, &arr), None);
}

/// Test freq with special characters
#[test]
fn test_freq_special_chars() {
    assert_eq!(freq("!!!abc!"), "!");
    assert_eq!(freq("a b c   "), " ");
}

/// Test freq with numbers and letters
#[test]
fn test_freq_alphanumeric() {
    assert_eq!(freq("111abc"), "1");
    assert_eq!(freq("a1b2c3aaa"), "a");
}

/// Test freq with unicode characters
#[test]
fn test_freq_unicode() {
    assert_eq!(freq("caféé"), "é");
    assert_eq!(freq("aaabbé"), "a");
}

/// Test freq with single character
#[test]
fn test_freq_single_char() {
    assert_eq!(freq("x"), "x");
    assert_eq!(freq("5"), "5");
}

/// Test zip_hash with larger vectors
#[test]
fn test_zip_hash_larger() {
    let keys = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];
    let vals = vec!["1".into(), "2".into(), "3".into(), "4".into(), "5".into()];
    let m = zip_hash(&keys, &vals).unwrap();
    
    assert_eq!(m.len(), 5);
    assert_eq!(m.get("a"), Some(&"1".to_string()));
    assert_eq!(m.get("c"), Some(&"3".to_string()));
    assert_eq!(m.get("e"), Some(&"5".to_string()));
}

/// Test zip_hash with duplicate keys (later value overwrites)
#[test]
fn test_zip_hash_duplicate_keys() {
    let keys = vec!["a".into(), "b".into(), "a".into()];
    let vals = vec!["1".into(), "2".into(), "3".into()];
    let m = zip_hash(&keys, &vals).unwrap();
    
    // The last "a" should overwrite the first
    assert_eq!(m.get("a"), Some(&"3".to_string()));
    assert_eq!(m.get("b"), Some(&"2".to_string()));
}

/// Test zip_hash mismatched lengths (first longer)
#[test]
fn test_zip_hash_first_longer() {
    let keys = vec!["a".into(), "b".into(), "c".into()];
    let vals = vec!["1".into()];
    assert!(zip_hash(&keys, &vals).is_none());
}

/// Test hash_to_array with various key orderings
#[test]
fn test_hash_to_array_ordering() {
    let mut m = HashMap::new();
    m.insert("apple".to_string(), "red".to_string());
    m.insert("banana".to_string(), "yellow".to_string());
    m.insert("grape".to_string(), "purple".to_string());
    
    let v = hash_to_array(&m);
    assert_eq!(v.len(), 3);
    
    // Check that it's sorted by key
    let keys: Vec<String> = v.iter().map(|(k, _)| k.clone()).collect();
    let mut sorted_keys = keys.clone();
    sorted_keys.sort();
    assert_eq!(keys, sorted_keys);
}

/// Test hash_to_array with numeric string keys
#[test]
fn test_hash_to_array_numeric_keys() {
    let mut m = HashMap::new();
    m.insert("10".to_string(), "ten".to_string());
    m.insert("2".to_string(), "two".to_string());
    m.insert("30".to_string(), "thirty".to_string());
    
    let v = hash_to_array(&m);
    // Should be lexicographically sorted: "10", "2", "30"
    assert_eq!(v.len(), 3);
}

/// Test PhoneBook with various invalid phone formats
#[test]
fn test_phonebook_invalid_formats() {
    let mut pb = PhoneBook::new();
    
    // Too short
    assert_eq!(pb.add("A".into(), "123-456-789".into(), true), false);
    
    // Too long
    assert_eq!(pb.add("B".into(), "123-456-78901".into(), true), false);
    
    // Wrong separators
    assert_eq!(pb.add("C".into(), "123.456.7890".into(), true), false);
    assert_eq!(pb.add("D".into(), "123/456/7890".into(), true), false);
    assert_eq!(pb.add("E".into(), "123 456 7890".into(), true), false);
    
    // Missing separators
    assert_eq!(pb.add("F".into(), "1234567890".into(), true), false);
    
    // Letters in number
    assert_eq!(pb.add("G".into(), "ABC-DEF-GHIJ".into(), true), false);
    
    // Empty string
    assert_eq!(pb.add("H".into(), "".into(), true), false);
}

/// Test PhoneBook with valid edge case numbers
#[test]
fn test_phonebook_valid_edge_numbers() {
    let mut pb = PhoneBook::new();
    
    // All zeros
    assert_eq!(pb.add("Zero".into(), "000-000-0000".into(), true), true);
    
    // All nines
    assert_eq!(pb.add("Nine".into(), "999-999-9999".into(), true), true);
    
    // Mixed
    assert_eq!(pb.add("Mix".into(), "123-456-7890".into(), true), true);
}

/// Test PhoneBook name uniqueness across listed/unlisted
#[test]
fn test_phonebook_name_uniqueness() {
    let mut pb = PhoneBook::new();
    
    assert_eq!(pb.add("Alice".into(), "111-111-1111".into(), true), true);
    
    // Same name, different number, different listing status
    assert_eq!(pb.add("Alice".into(), "222-222-2222".into(), false), false);
    
    // Same name, same number
    assert_eq!(pb.add("Alice".into(), "111-111-1111".into(), true), false);
}

/// Test PhoneBook with multiple unlisted entries with same number
#[test]
fn test_phonebook_multiple_unlisted_same_number() {
    let mut pb = PhoneBook::new();
    
    let num = "555-555-5555".to_string();
    
    // Multiple unlisted entries with same number should all succeed
    assert_eq!(pb.add("Person1".into(), num.clone(), false), true);
    assert_eq!(pb.add("Person2".into(), num.clone(), false), true);
    assert_eq!(pb.add("Person3".into(), num.clone(), false), true);
    
    // None should be found via lookup (all unlisted)
    assert_eq!(pb.lookup("Person1"), None);
    assert_eq!(pb.lookup("Person2"), None);
    assert_eq!(pb.lookup("Person3"), None);
    
    // Reverse lookup should also return None
    assert_eq!(pb.lookup_by_num(&num), None);
}

/// Test PhoneBook with listed number, then try to add another listed with same number
#[test]
fn test_phonebook_duplicate_listed_number() {
    let mut pb = PhoneBook::new();
    
    let num = "777-777-7777".to_string();
    
    // First listed entry
    assert_eq!(pb.add("First".into(), num.clone(), true), true);
    
    // Second listed entry with same number should fail
    assert_eq!(pb.add("Second".into(), num.clone(), true), false);
    
    // But unlisted entry with same number should succeed
    assert_eq!(pb.add("Third".into(), num.clone(), false), true);
    
    // Lookup should return the first listed name
    assert_eq!(pb.lookup_by_num(&num), Some("First".into()));
}

/// Test PhoneBook lookup with empty phonebook
#[test]
fn test_phonebook_empty_lookup() {
    let pb = PhoneBook::new();
    
    assert_eq!(pb.lookup("Anyone"), None);
    assert_eq!(pb.lookup_by_num("123-456-7890"), None);
    assert_eq!(pb.names_by_ac("123").len(), 0);
}

/// Test PhoneBook lookup with case sensitivity
#[test]
fn test_phonebook_case_sensitivity() {
    let mut pb = PhoneBook::new();
    
    assert_eq!(pb.add("Alice".into(), "111-111-1111".into(), true), true);
    
    // Different case should not match
    assert_eq!(pb.lookup("alice"), None);
    assert_eq!(pb.lookup("ALICE"), None);
    
    // Exact match should work
    assert_eq!(pb.lookup("Alice"), Some("111-111-1111".into()));
}

/// Test PhoneBook names_by_ac with various area codes
#[test]
fn test_phonebook_names_by_ac_comprehensive() {
    let mut pb = PhoneBook::new();
    
    assert_eq!(pb.add("A1".into(), "100-000-0001".into(), true), true);
    assert_eq!(pb.add("A2".into(), "100-000-0002".into(), false), true);
    assert_eq!(pb.add("B1".into(), "200-000-0001".into(), true), true);
    assert_eq!(pb.add("B2".into(), "200-000-0002".into(), false), true);
    assert_eq!(pb.add("C1".into(), "300-000-0001".into(), false), true);
    
    // Check area code 100
    let mut names_100 = pb.names_by_ac("100");
    names_100.sort();
    assert_eq!(names_100, vec!["A1", "A2"]);
    
    // Check area code 200
    let mut names_200 = pb.names_by_ac("200");
    names_200.sort();
    assert_eq!(names_200, vec!["B1", "B2"]);
    
    // Check area code 300
    let names_300 = pb.names_by_ac("300");
    assert_eq!(names_300.len(), 1);
    assert_eq!(names_300[0], "C1");
    
    // Check non-existent area code
    assert_eq!(pb.names_by_ac("999").len(), 0);
}

/// Test PhoneBook names_by_ac includes both listed and unlisted
#[test]
fn test_phonebook_names_by_ac_mixed_listing() {
    let mut pb = PhoneBook::new();
    
    assert_eq!(pb.add("Listed1".into(), "555-123-4567".into(), true), true);
    assert_eq!(pb.add("Unlisted1".into(), "555-234-5678".into(), false), true);
    assert_eq!(pb.add("Listed2".into(), "555-345-6789".into(), true), true);
    
    // All three should be returned regardless of listing status
    let names = pb.names_by_ac("555");
    assert_eq!(names.len(), 3);
    
    // But lookup should only return listed
    assert_eq!(pb.lookup("Listed1"), Some("555-123-4567".into()));
    assert_eq!(pb.lookup("Unlisted1"), None);
    assert_eq!(pb.lookup("Listed2"), Some("555-345-6789".into()));
}

/// Test PhoneBook with empty name
#[test]
fn test_phonebook_empty_name() {
    let mut pb = PhoneBook::new();
    
    // Empty name should still be added if allowed by implementation
    let _result = pb.add("".into(), "123-456-7890".into(), true);
    // This tests implementation-specific behavior
    // If it succeeds, lookup should work with empty string
}

/// Test PhoneBook with whitespace in names
#[test]
fn test_phonebook_names_with_spaces() {
    let mut pb = PhoneBook::new();
    
    assert_eq!(pb.add("John Doe".into(), "111-222-3333".into(), true), true);
    assert_eq!(pb.add("Jane Smith".into(), "444-555-6666".into(), true), true);
    
    assert_eq!(pb.lookup("John Doe"), Some("111-222-3333".into()));
    assert_eq!(pb.lookup("Jane Smith"), Some("444-555-6666".into()));
}

/// Test PhoneBook lookup_by_num with unlisted numbers
#[test]
fn test_phonebook_lookup_by_num_unlisted_only() {
    let mut pb = PhoneBook::new();
    
    // Add several unlisted entries
    assert_eq!(pb.add("U1".into(), "111-111-1111".into(), false), true);
    assert_eq!(pb.add("U2".into(), "222-222-2222".into(), false), true);
    
    // None should be found
    assert_eq!(pb.lookup_by_num("111-111-1111"), None);
    assert_eq!(pb.lookup_by_num("222-222-2222"), None);
}

/// Test PhoneBook area code edge cases
#[test]
fn test_phonebook_area_code_edge_cases() {
    let mut pb = PhoneBook::new();
    
    assert_eq!(pb.add("Test000".into(), "000-123-4567".into(), true), true);
    assert_eq!(pb.add("Test999".into(), "999-123-4567".into(), true), true);
    
    let names_000 = pb.names_by_ac("000");
    assert_eq!(names_000.len(), 1);
    assert_eq!(names_000[0], "Test000");
    
    let names_999 = pb.names_by_ac("999");
    assert_eq!(names_999.len(), 1);
    assert_eq!(names_999[0], "Test999");
}
