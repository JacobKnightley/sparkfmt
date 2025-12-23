import SqlBaseLexer from './dist/generated/SqlBaseLexer.js';
import antlr4 from 'antlr4';

// Test scientific notation
const sql = 'SELECT 1.23e10 FROM t';
const chars = new antlr4.InputStream(sql);
const lexer = new SqlBaseLexer(chars);
const tokens = new antlr4.CommonTokenStream(lexer);
tokens.fill();

console.log('Tokens for: SELECT 1.23e10 FROM t');
for (const token of tokens.tokens) {
    if (token.type !== antlr4.Token.EOF && token.type !== SqlBaseLexer.WS) {
        const symbolicName = SqlBaseLexer.symbolicNames[token.type];
        console.log(`  Type: ${token.type} (${symbolicName}), Text: "${token.text}"`);
    }
}
