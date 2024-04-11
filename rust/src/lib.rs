use std::collections::VecDeque;

///
///
/// # Arguments
///
/// * `input`: number to convert to roman numerals
///
/// returns: String
///
/// # Examples
///
/// ```
/// use roman_numerals::to_roman;
/// let roman = to_roman(2023);
/// ```
pub fn to_roman(input: u32) -> String {
    const LOOKUP: [(&str, u32); 13] = [
        ("M", 1_000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    let mut roman_phrase = String::new();
    let mut number = input;
    for (roman, arabic) in LOOKUP {
        while number >= arabic {
            number -= arabic;
            roman_phrase.push_str(roman);
        }
    }
    roman_phrase
}

///
///
/// # Arguments
///
/// * `roman`: roman number to convert to arabic
///
/// returns: i32
///
/// # Examples
///
/// ```
/// use roman_numerals::to_arabic;
/// let number = to_arabic(String::from("MCMXCIV"));
/// ```
pub fn to_arabic(roman: String) -> i32 {
    let mut result = 0;
    let mut letters = roman.chars().collect::<VecDeque<char>>();
    while !letters.is_empty() {
        let start_letter = letters.pop_front();
        let double = map_double_chars((start_letter, letters.front()));
        match double {
            Some(digit) => {
                result += digit;
                letters.pop_front();
            }
            None => {
                let digit = map_single_char(start_letter);
                result += digit.unwrap_or(0);
            }
        }
    }
    result
}

fn map_single_char(roman_char: Option<char>) -> Option<i32> {
    match roman_char {
        Some('M') => Some(1000),
        Some('D') => Some(500),
        Some('C') => Some(100),
        Some('L') => Some(50),
        Some('X') => Some(10),
        Some('V') => Some(5),
        Some('I') => Some(1),
        _ => None,
    }
}

fn map_double_chars(roman_chars: (Option<char>, Option<&char>)) -> Option<i32> {
    match roman_chars {
        (Some('C'), Some('M')) => Some(900),
        (Some('C'), Some('D')) => Some(400),
        (Some('X'), Some('C')) => Some(90),
        (Some('X'), Some('L')) => Some(40),
        (Some('I'), Some('X')) => Some(9),
        (Some('I'), Some('V')) => Some(4),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_roman() {
        assert_eq!(to_roman(3), "III");
        assert_eq!(to_roman(4), "IV");
        assert_eq!(to_roman(9), "IX");
        assert_eq!(to_roman(58), "LVIII");
        assert_eq!(to_roman(1974), "MCMLXXIV");
        assert_eq!(to_roman(1994), "MCMXCIV");
        assert_eq!(to_roman(1996), "MCMXCVI");
        assert_eq!(to_roman(1997), "MCMXCVII");
        assert_eq!(to_roman(1998), "MCMXCVIII");
        assert_eq!(to_roman(1999), "MCMXCIX");
        assert_eq!(to_roman(2014), "MMXIV");
        assert_eq!(to_roman(2015), "MMXV");
        assert_eq!(to_roman(2023), "MMXXIII");
    }

    #[test]
    fn test_to_arabic() {
        assert_eq!(to_arabic(String::from("III")), 3);
        assert_eq!(to_arabic(String::from("IV")), 4);
        assert_eq!(to_arabic(String::from("IX")), 9);
        assert_eq!(to_arabic(String::from("LVIII")), 58);
        assert_eq!(to_arabic(String::from("MCMXCIV")), 1994);
        assert_eq!(to_arabic(String::from("MCMXCVI")), 1996);
        assert_eq!(to_arabic(String::from("MCMXCVII")), 1997);
        assert_eq!(to_arabic(String::from("MCMXCVIII")), 1998);
        assert_eq!(to_arabic(String::from("MCMXCIX")), 1999);
        assert_eq!(to_arabic(String::from("MMXIV")), 2014);
        assert_eq!(to_arabic(String::from("MMXV")), 2015);
    }
}
