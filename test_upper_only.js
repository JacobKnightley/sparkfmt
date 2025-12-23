import SqlBaseLexer from './dist/generated/SqlBaseLexer.js';
import antlr4 from 'antlr4';

// Test with uppercase only
const sql = 'SELECT 1.23e10 FROM t';
const upperSql = sql.toUpperCase();

console.log('Original SQL:', sql);
console.log('Upper SQL:', upperSql);

const chars = new antlr4.InputStream(upperSql);
const lexer = new SqlBaseLexer(chars);
const tokens = new antlr4.CommonTokenStream(lexer);
tokens.fill();

console.log('\nTokens from uppercase:');
for (const token of tokens.tokens) {
    if (token.type !== antlr4.Token.EOF && token.type !== SqlBaseLexer.WS) {
        const symbolicName = SqlBaseLexer.symbolicNames[token.type];
        console.log(`  ${symbolicName}: "${token.text}"`);
    }
}

// Now lex original
const origChars = new antlr4.InputStream(sql);
const origLexer = new SqlBaseLexer(origChars);
const origTokens = new antlr4.CommonTokenStream(origLexer);
origTokens.fill();

console.log('\nTokens from original:');
for (const token of origTokens.tokens) {
    if (token.type !== antlr4.Token.EOF && token.type !== SqlBaseLexer.WS) {
        const symbolicName = SqlBaseLexer.symbolicNames[token.type];
        console.log(`  ${symbolicName}: "${token.text}"`);
    }
}

console.log('\nToken count - uppercase:', tokens.tokens.filter(t => t.type !== antlr4.Token.EOF && t.type !== SqlBaseLexer.WS).length);
console.log('Token count - original:', origTokens.tokens.filter(t => t.type !== antlr4.Token.EOF && t.type !== SqlBaseLexer.WS).length);
