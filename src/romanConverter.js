(function() {
    "option strict";  var RA, getRomanYear, toArabic, toRoman;
    RA = {
        "M": 1000,
        "CM": 900,
        "D": 500,
        "CD": 400,
        "C": 100,
        "XC": 90,
        "L": 50,
        "XL": 40,
        "X": 10,
        "IX": 9,
        "V": 5,
        "IV": 4,
        "I": 1
    };
    toRoman = function(number) {
        var arabic, letter, roman;
        letter = [];
        for (roman in RA) {
            arabic = RA[roman];
            while (number >= arabic) {
                number -= arabic;
                letter.push(roman);
            }
        }
        return letter.join("");
    };
    toArabic = function(number) {
        var doubleLetter, result, romans, startLetter;
        result = 0;
        romans = number.split("");
        while (romans.length > 0) {
            startLetter = romans.shift();
            doubleLetter = RA[startLetter + romans[0]];
            if (doubleLetter) {
                result += doubleLetter;
                romans.shift();
            } else {
                result += RA[startLetter];
            }
        }
        return result;
    };
    getRomanYear = function() {
        return toRoman(new Date().getFullYear());
    };
    if (!window.romanConverter) {
        window.romanConverter = {};
    }
    romanConverter.toRoman = toRoman;
    romanConverter.toArabic = toArabic;
    romanConverter.getRomanYear = getRomanYear;
}).call(this);
