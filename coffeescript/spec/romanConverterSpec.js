(function() {
    describe("Convert arabic to roman numbers", function() {
        it("Converts 1000 to letter 'M'", function() {
            return expect(window.romanConverter.toRoman(1000)).toEqual("M");
        });
        it("Convert 1 to letter 'I'", function() {
            return expect(window.romanConverter.toRoman(1)).toEqual("I");
        });
        it("Convert 3 to letter 'III'", function() {
            return expect(window.romanConverter.toRoman(3)).toEqual("III");
        });
        it("Convert 6 to letter 'VI'", function() {
            return expect(window.romanConverter.toRoman(6)).toEqual("VI");
        });
        it("Convert 6.5 to letter 'VI'", function() {
            return expect(window.romanConverter.toRoman(6.5)).toEqual("VI");
        });
        return it("Convert 0.5 to empty string", function() {
            return expect(window.romanConverter.toRoman(0.5)).toEqual("");
        });
    });
    describe("Convert string values to roman numbers", function() {
        it("Convert '12' to letter 'XII'", function() {
            return expect(window.romanConverter.toRoman("12")).toEqual("XII");
        });
        it("Convert 'A12' returns empty string", function() {
            return expect(window.romanConverter.toRoman("A12")).toEqual("");
        });
        return it("Convert empty string and returns empty string", function() {
            return expect(window.romanConverter.toRoman("")).toEqual("");
        });
    });
    describe("Convert roman to arabic number", function() {
        beforeEach(function() {
            return this.addMatchers({
                toBeNaN: function(expected) {
                    return isNaN(expected);
                }
            });
        });
        it("Convert roman number 'I' to 1", function() {
            return expect(window.romanConverter.toArabic("I")).toEqual(1);
        });
        it("Convert roman number 'IV' to 4", function() {
            return expect(window.romanConverter.toArabic("IV")).toEqual(4);
        });
        it("Convert roman number 'XXIV' to 24", function() {
            return expect(window.romanConverter.toArabic("XXIV")).toEqual(24);
        });
        it("Convert roman number 'XL' to 40", function() {
            return expect(window.romanConverter.toArabic("XL")).toEqual(40);
        });
        it("Convert roman number 'CDXLVIII' to 448", function() {
            return expect(window.romanConverter.toArabic("CDXLVIII")).toEqual(448);
        });
        it("Convert roman number 'MMDCCLI' to 2751", function() {
            return expect(window.romanConverter.toArabic("MMDCCLI")).toEqual(2751);
        });
        it("Convert empty string fails and returns not a number", function() {
            return expect(window.romanConverter.toArabic("")).toBeNaN();
        });
        return it("Convert 'XVAA' fails and returns not a number", function() {
            return expect(window.romanConverter.toArabic("XVAA")).toBeNaN();
        });
    });
    describe("Current year in roman numbers", function() {
        return it("Convert current year into roman numbers", function() {
            return expect(window.romanConverter.getRomanYear()).toBeDefined();
        });
    });
}).call(this);
