# pb-mathcat-node-wrapper

A Node.js wrapper for the [MathCAT](https://github.com/NSoiffer/MathCAT) library, designed for converting MathML to peech text. Built specifically for Pressbooks math microservices.

## Overview

This package provides Node.js bindings to the MathCAT Rust library, enabling server-side conversion of mathematical expressions (in MathML format) to speech text for accessibility purposes. It uses NAPI-RS for optimal performance and cross-platform compatibility.

## Technology Stack

- **Core Library**: [MathCAT](https://github.com/NSoiffer/MathCAT) (Rust)
- **Bindings**: [NAPI-RS](https://napi.rs/) v3.0 - Native Node.js addon framework
- **Language**: Rust (for wrapper) + JavaScript/TypeScript (for Node.js integration)
- **Build System**: Cargo + npm
- **Supported Platforms**: macOS (ARM64/x64), Linux (x64), Windows (x64)

## Requirements

- **Node.js**: 18.12.0 or later
- **Rust**: Latest stable version
- **Platform**: macOS, Linux, or Windows

## Installation

### For Development

```bash
# Clone the repository
git clone <repository-url>
cd pb-mathcat-node-wrapper

# Install dependencies
npm install

# Build the native addon
npm run build
```

## Building

```bash
# Development build
npm run build

# The build process will:
# 1. Compile Rust code using Cargo
# 2. Generate platform-specific .node binary
# 3. Create TypeScript definitions
```

**Output**: `pb-mathcat-node-wrapper.{platform}-{arch}.node`

## Testing

```bash
# Run all tests
npm test

# Tests include:
# - Simple math expressions (x)
# - Equations (x = 5)  
# - Complex expressions (square roots, etc.)
# - Error handling for invalid input
# - Language preference changes
# - Getting the MathCAT Version 
```

## API Reference

### `initMathcat(rulesPath?: string): string`

Initializes the MathCAT library with speech generation rules.

```javascript
const { initMathcat } = require('pb-mathcat-node-wrapper');

// Initialize with default rules path
const result = initMathcat('./Rules');

// Or use environment variable MATHCAT_RULES_DIR
const result = initMathcat(); // Uses MATHCAT_RULES_DIR
```

**Parameters:**

- `rulesPath` (optional): Path to MathCAT rules directory. Falls back to `MATHCAT_RULES_DIR` environment variable.

**Returns:** Success message string

### `getSpeechTextFromMathcat(mathml: string): string`

Converts MathML to accessible speech text.

```javascript
const { getSpeechTextFromMathcat } = require('pb-mathcat-node-wrapper');

const mathml = '<math><mi>x</mi><mo>=</mo><mn>5</mn></math>';
const speech = getSpeechTextFromMathcat(mathml);
// Returns: "x is equal to 5"
```

**Parameters:**

- `mathml`: Valid MathML string

**Returns:** Speech text string or error message (prefixed with `-!ERROR!-`)

### `setMathcatPreference(key: string, value: string): string`

Configures MathCAT speech generation preferences.

```javascript
const { setMathcatPreference } = require('pb-mathcat-node-wrapper');

// Change language to German
setMathcatPreference('Language', 'de');

// Change speech style
setMathcatPreference('SpeechStyle', 'SimpleSpeak');

// Useful preferences include:
// - Language: 'en', 'de', 'es', 'fi', 'sv', etc.
// - SpeechStyle: 'ClearSpeak', 'SimpleSpeak'
// - Verbosity: 'Terse', 'Medium', 'Verbose'
```

**Parameters:**

- `key`: Preference name
- `value`: Preference value

**Returns:** Confirmation message string

### `getMathcatVersion(): string`

Returns the version of the underlying MathCAT library.

```javascript
const { getMathcatVersion } = require('pb-mathcat-node-wrapper');

const version = getMathcatVersion();
// Returns: "0.7.2"
```

**Returns:** Version string

## Usage Example

```javascript
const {
  initMathcat,
  getSpeechTextFromMathcat,
  setMathcatPreference,
  getMathcatVersion
} = require('pb-mathcat-node-wrapper');

// Initialize MathCAT
initMathcat('./Rules');

// Check version
console.log('MathCAT version:', getMathcatVersion());

// Convert simple expression
const simpleExpr = '<math><mi>x</mi></math>';
console.log(getSpeechTextFromMathcat(simpleExpr)); // "x"

// Convert equation
const equation = '<math><mi>x</mi><mo>=</mo><mn>5</mn></math>';
console.log(getSpeechTextFromMathcat(equation)); // "x is equal to 5"

// Change to German and try again  
setMathcatPreference('Language', 'de');
console.log(getSpeechTextFromMathcat(equation)); // "x ist gleich 5"

// Complex expression with square root
const complex = '<math><mi>x</mi><mo>=</mo><mrow><msqrt><mn>5</mn></msqrt><mo>+</mo><mn>4</mn></mrow></math>';
console.log(getSpeechTextFromMathcat(complex));
// "x is equal to, the square root of 5, end root; plus 4"
```

## Error Handling

The wrapper handles errors gracefully:

```javascript
// Invalid MathML
const invalid = 'not valid xml';
const result = getSpeechTextFromMathcat(invalid);
// Returns: "-!ERROR!- Invalid MathML input: ..."

// Check for errors
if (result.startsWith('-!ERROR!-')) {
  console.error('MathCAT error:', result);
}
```

## Rules Directory

The `Rules/` directory contains MathCAT's speech generation rules and must be present for the library to function. It includes:

- **Languages**: Speech rules for different languages (en, de, es, fi, sv, etc.)
- **Braille**: Braille translation rules (Nemeth, UEB, etc.) 
- **Intent**: Mathematical intent recognition rules
- **Definitions**: Core mathematical definitions and patterns

## Development

### Project Structure

```txt
├── src/lib.rs              # Rust wrapper implementation
├── Cargo.toml              # Rust dependencies and build config
├── package.json            # Node.js package configuration
├── index.js                # Auto-generated JavaScript bindings  
├── index.d.ts              # TypeScript type definitions
├── Rules/                  # MathCAT speech generation rules
├── test/                   # Test suite
└── pb-mathcat-node-wrapper.*.node  # Compiled binary
```

### Available Scripts

```bash
npm run build       # Build the native addon
npm test           # Run test suite  
```

## License

MIT

## Links

- [MathCAT Library](https://github.com/NSoiffer/MathCAT)
- [NAPI-RS Documentation](https://napi.rs/)
- [Pressbooks](https://pressbooks.org/)
