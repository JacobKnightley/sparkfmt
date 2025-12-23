import { isKeywordToken, getSymbolicName } from './dist/token-utils.js';

// Check if DOUBLEQUOTED_STRING is being treated as a keyword
const tokenType = 439; // DOUBLEQUOTED_STRING
const text = '"MyColumn"';
const symbolicName = getSymbolicName(tokenType);

console.log(`Token type: ${tokenType}`);
console.log(`Symbolic name: ${symbolicName}`);
console.log(`Text: ${text}`);
console.log(`isKeywordToken: ${isKeywordToken(tokenType, text)}`);
