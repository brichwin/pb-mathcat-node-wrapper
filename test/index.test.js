const { initMathcat, getSpeechTextFromMathcat, setMathcatPreference, getMathcatVersion } = require('../index.js');
const { test } = require('node:test');
const assert = require('node:assert');
const path = require('path');

// Use path.join for cross-platform compatibility
const rulesPath = path.join(__dirname, '..', 'Rules');

initMathcat(rulesPath);

test('should process simple math expression', () => {
    const input = '<math><mi>x</mi></math>';
    const result = getSpeechTextFromMathcat(input);
    console.log('Result:', result);

    assert(!result.startsWith('-!ERROR!-'), 'Should not return error');
    assert(result.length > 0, 'Should return non-empty string');
    assert(result === 'x', `Should return 'x', received '${result}'`);
});

test('should process equation', () => {
    const input = '<math><mi>x</mi><mo>=</mo><mn>5</mn></math>';
    const result = getSpeechTextFromMathcat(input);
    console.log('Result:', result);

    assert(!result.startsWith('-!ERROR!-'));
    assert(result === 'x is equal to 5', `Should return 'x is equal to 5', received '${result}'`);
});

test('should handle invalid input gracefully', () => {
    const input = 'not valid xml';
    const result = getSpeechTextFromMathcat(input);
    console.log('Result:', result);

    // Should either work or return error string (not crash)
    assert(typeof result === 'string');
});

test('should return result according to defaults', () => {
    const input = '<math><mi>x</mi><mo>=</mo><mrow><msqrt><mn>5</mn></msqrt><mo>+</mo><mn>4</mn></mrow></math>';
    const result = getSpeechTextFromMathcat(input);
    console.log('Result:', result);

    assert(result === 'x is equal to, the square root of 5, end root; plus 4');
});

test('should return result according to preference change', () => {
    const input = '<math><mi>x</mi><mo>=</mo><mrow><msqrt><mn>5</mn></msqrt><mo>+</mo><mn>4</mn></mrow></math>';
    setMathcatPreference('Language', 'de');
    const result = getSpeechTextFromMathcat(input);
    console.log('Result:', result);

    assert(result === 'x ist gleich, die quadratwurzel von 5, ende der wurzel; plus 4');
});

test('should return string result matching a version number', () => {
    const result = getMathcatVersion();
    console.log('Mathcat version:', result);

    // Simple regex to match version numbers like "1.2.3" or "10.20.30"
    const versionRegex = /^\d+(\.\d+){2,}$/;
    assert(versionRegex.test(result), `Version string '${result}' does not match expected format`);
});


