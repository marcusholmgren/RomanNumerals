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
pub fn to_arabic(roman: String) -> Result<i32, String> {
    if roman.is_empty() {
        return Err("Input Roman numeral string cannot be empty".to_string());
    }

    let mut total = 0;
    let mut prev_group_value = i32::MAX;
    let mut prev_char_val_for_additive_repetition = 0;
    let mut additive_repeat_count = 0;

    let mut char_val_in_subtractive_context = 0; // Stores the value of the char that WAS subtracted (e.g. C in CM)

    // For MCMD rule: if last group was Y-X, next group Z cannot be X < Z < Y
    let mut was_last_group_subtractive = false;
    let mut last_sub_x_val = 0; // Value of X in Y-X pair (snake_case)
    let mut last_sub_y_val = 0; // Value of Y in Y-X pair (snake_case)


    let mut chars = roman.chars().peekable();

    while let Some(current_char_instance) = chars.next() {
        let current_value = roman_char_to_value(current_char_instance)?;

        // Reset char_val_in_subtractive_context if it's not used by the current char processing
        if char_val_in_subtractive_context != 0 && char_val_in_subtractive_context != current_value {
             // If the previous group was subtractive, and current char is different from the one subtracted, reset.
             // This logic might need refinement. The goal is to catch "IXI" if 'I' (from IX) and the next 'I' are problematic.
        }


        if let Some(&next_char_instance) = chars.peek() {
            let next_value = roman_char_to_value(next_char_instance)?;

            if current_value < next_value { // Potential subtractive pair
                let is_valid_subtractive_char_combo = match current_value {
                    1 if next_value == 5 || next_value == 10 => true,    // IV, IX
                    10 if next_value == 50 || next_value == 100 => true,  // XL, XC
                    100 if next_value == 500 || next_value == 1000 => true, // CD, CM
                    _ => false,
                };
                if !is_valid_subtractive_char_combo {
                    return Err(format!("Invalid subtractive pair: {}{}", current_char_instance, next_char_instance));
                }

                let group_value = next_value - current_value;

                if group_value > prev_group_value {
                    return Err(format!("Invalid order: subtractive group {} ({}{}) value is greater than previous group value {}", group_value, current_char_instance, next_char_instance, prev_group_value));
                }
                // Rule: A numeral's value must generally be non-increasing.
                // If a subtractive pair has the same value as the previous group, it's often an error (e.g. "IXIX", "CDCD")
                // "CMCM" (900, 900) is valid, "XCXC" (90,90) is valid.
                // This means if group_value == prev_group_value, it's only ok if current_value is C, X (or I for III vs IV)
                // For now, let's allow same-value groups, tests for IXIX etc. will guide.
                // if group_value == prev_group_value && !(current_value == 100 || current_value == 10 || current_value == 1) {
                //    return Err(format!("Invalid repetition of subtractive group value {}: {}{}", group_value, current_char_instance, next_char_instance));
                // }


                total += group_value;
                prev_group_value = group_value;
                char_val_in_subtractive_context = current_value;

                was_last_group_subtractive = true;
                last_sub_x_val = current_value; // snake_case
                last_sub_y_val = next_value;   // snake_case

                additive_repeat_count = 0;
                prev_char_val_for_additive_repetition = 0;

                chars.next();
                continue;
            }
        }

        // Additive case
        let group_value = current_value;

        // Rule for MCMD: if previous group was Y-X, current group Z cannot be X < Z < Y.
        // This check uses the state from the *previous* iteration if a subtractive pair was formed.
        if was_last_group_subtractive && group_value > last_sub_x_val && group_value < last_sub_y_val {
            return Err(format!(
                "Invalid sequence: group {} ('{}') cannot appear after subtractive pair {}-{} (value {}) because {} < {} < {}",
                group_value, current_char_instance,
                roman_char_to_char(last_sub_y_val), roman_char_to_char(last_sub_x_val),
                (last_sub_y_val - last_sub_x_val),
                last_sub_x_val, group_value, last_sub_y_val
            ));
        }
        // Current group is additive, so for the *next* iteration's check, the last group was not subtractive.
        was_last_group_subtractive = false;

        if group_value > prev_group_value {
            return Err(format!("Invalid order: additive {} ({}) value is greater than previous group value {}", group_value, current_char_instance, prev_group_value));
        }

        if char_val_in_subtractive_context == current_value {
            return Err(format!(
                "Invalid sequence: char '{}' ({}) used additively after being subtracted to form the preceding group of value {}",
                current_char_instance, current_value, prev_group_value
            ));
        }
        char_val_in_subtractive_context = 0;


        if current_value == prev_char_val_for_additive_repetition {
            additive_repeat_count += 1;
        } else {
            additive_repeat_count = 1;
            prev_char_val_for_additive_repetition = current_value;
        }

        match current_value {
            5 | 50 | 500 => {
                if additive_repeat_count > 1 {
                    return Err(format!("Invalid repetition of {}: more than once", current_char_instance));
                }
            }
            1 | 10 | 100 | 1000 => {
                if additive_repeat_count > 3 {
                    return Err(format!("Invalid repetition of {}: more than 3 times", current_char_instance));
                }
            }
            _ => {}
        }

        total += group_value;
        prev_group_value = group_value;
    }

    Ok(total)
}

// Helper function to map value back to char for error messages
fn roman_char_to_char(val: i32) -> char {
    match val {
        1 => 'I', 5 => 'V', 10 => 'X', 50 => 'L', 100 => 'C', 500 => 'D', 1000 => 'M',
        _ => '?', // Should ideally not be reached if logic is correct
    }
}

fn roman_char_to_value(c: char) -> Result<i32, String> {
    match c {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err(format!("Unknown Roman numeral character: {}", c)),
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
        // New test cases
        assert_eq!(to_roman(0), "", "Test for input 0");
        assert_eq!(to_roman(3999), "MMMCMXCIX", "Test for input 3999");
    }

    #[test]
    fn test_to_arabic() {
        // Valid cases (adapted)
        assert_eq!(to_arabic(String::from("I")), Ok(1));
        assert_eq!(to_arabic(String::from("III")), Ok(3));
        assert_eq!(to_arabic(String::from("IV")), Ok(4));
        assert_eq!(to_arabic(String::from("IX")), Ok(9));
        assert_eq!(to_arabic(String::from("LVIII")), Ok(58));
        assert_eq!(to_arabic(String::from("MCMXCIV")), Ok(1994));
        assert_eq!(to_arabic(String::from("MCMXCVI")), Ok(1996));
        assert_eq!(to_arabic(String::from("MCMXCVII")), Ok(1997));
        assert_eq!(to_arabic(String::from("MCMXCVIII")), Ok(1998));
        assert_eq!(to_arabic(String::from("MCMXCIX")), Ok(1999));
        assert_eq!(to_arabic(String::from("MMXIV")), Ok(2014));
        assert_eq!(to_arabic(String::from("MMXV")), Ok(2015));
        assert_eq!(to_arabic(String::from("MMMCMXCIX")), Ok(3999)); // Max standard Roman numeral
        assert_eq!(to_arabic(String::from("MDCCCLXXXVIII")), Ok(1888));
        assert_eq!(to_arabic(String::from("D")), Ok(500));
        assert_eq!(to_arabic(String::from("C")), Ok(100));
        assert_eq!(to_arabic(String::from("L")), Ok(50));
        assert_eq!(to_arabic(String::from("X")), Ok(10));
        assert_eq!(to_arabic(String::from("V")), Ok(5));
        // IIIX is invalid by standard Roman numeral rules (should be XI).
        // The refined parser correctly identifies this as an order violation.
        assert!(to_arabic(String::from("IIIX")).is_err(), "IIIX should be an error due to order violation");
        assert_eq!(to_arabic(String::from("XIII")), Ok(13)); // X=10, I=1, I=1, I=1. res = 10+1+1+1=13.


        // Invalid character cases
        assert!(to_arabic(String::from("")).is_err(), "Empty string should be an error");
        assert!(to_arabic(String::from("mcmxciv")).is_err(), "Lowercase 'm' should be an error"); // Test with actual lowercase
        assert!(to_arabic(String::from("IIIx")).is_err(), "Lowercase 'x' should be an error"); // Test with actual lowercase
        assert!(to_arabic(String::from("IIA")).is_err(), "Invalid character 'A'");
        assert!(to_arabic(String::from("X Y")).is_err(), "Space is an invalid character");

        // Invalid sequence cases
        // The current implementation of to_arabic might not catch all of these semantic errors yet,
        // as it primarily focuses on character-by-character parsing and subtractive logic.
        // These tests define the *desired* behavior.
        assert!(to_arabic(String::from("IIII")).is_err(), "IIII is invalid"); // Should be IV
        assert!(to_arabic(String::from("VV")).is_err(), "VV is invalid");     // Should be X
        assert!(to_arabic(String::from("IC")).is_err(), "IC is invalid");
        assert!(to_arabic(String::from("IL")).is_err(), "IL is invalid");
        assert!(to_arabic(String::from("ID")).is_err(), "ID is invalid");
        assert!(to_arabic(String::from("IM")).is_err(), "IM is invalid");
        assert!(to_arabic(String::from("XD")).is_err(), "XD is invalid");
        assert!(to_arabic(String::from("XM")).is_err(), "XM is invalid"); // CD and CM are valid
        assert!(to_arabic(String::from("VX")).is_err(), "VX is invalid");
        assert!(to_arabic(String::from("VL")).is_err(), "VL is invalid");
        assert!(to_arabic(String::from("VC")).is_err(), "VC is invalid");
        assert!(to_arabic(String::from("VD")).is_err(), "VD is invalid");
        assert!(to_arabic(String::from("VM")).is_err(), "VM is invalid");
        assert!(to_arabic(String::from("LC")).is_err(), "LC is invalid");
        assert!(to_arabic(String::from("LD")).is_err(), "LD is invalid");
        assert!(to_arabic(String::from("LM")).is_err(), "LM is invalid");
        assert!(to_arabic(String::from("DM")).is_err(), "DM is invalid"); // CM is valid
        assert!(to_arabic(String::from("IXI")).is_err(), "IXI is invalid"); // I value can't be before X then I
        assert!(to_arabic(String::from("IVI")).is_err(), "IVI is invalid");
        assert!(to_arabic(String::from("MCMD")).is_err(), "MCMD is invalid"); // D cannot follow CM
        assert!(to_arabic(String::from("CCCC")).is_err(), "CCCC is invalid"); // Should be CD
        assert!(to_arabic(String::from("XXXX")).is_err(), "XXXX is invalid"); // Should be XL

        // Test cases that might pass with current logic but are semantically incorrect
        // (depending on how strictly "invalid sequence" is interpreted by current code)
        // For example, "IC" currently gives Ok(99), "IIII" gives Ok(4).
        // These are marked .is_err() as per problem spec for desired state.
    }
}
