"option strict"
RA = {"M": 1000, "CM": 900, "D": 500, "CD": 400, "C": 100, "XC": 90, "L": 50, "XL": 40, "X": 10, "IX": 9, "V": 5, "IV": 4, "I": 1}

toRoman = (number) ->
  letter = []
  for roman, arabic of RA
    while number >= arabic
      number -= arabic
      letter.push roman
  letter.join("")

toArabic = (number) ->
  result = 0;
  romans = number.split ""
  while romans.length > 0
    startLetter = romans.shift()
    doubleLetter = RA[startLetter + romans[0]]
    if doubleLetter
      result += doubleLetter
      romans.shift()
    else
      result += RA[startLetter]

  result

getRomanYear = () ->
  toRoman new Date().getFullYear()

window.romanConverter = {} unless window.romanConverter
romanConverter.toRoman = toRoman
romanConverter.toArabic = toArabic
romanConverter.getRomanYear = getRomanYear