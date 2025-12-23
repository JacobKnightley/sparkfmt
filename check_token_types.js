import SqlBaseLexer from './dist/generated/SqlBaseLexer.js';
import antlr4 from 'antlr4';

// Test with double-quoted string
const sql = 'SELECT "MyColumn" FROM t';
const chars = new antlr4.InputStream(sql);
const lexer = new SqlBaseLexer(chars);
const tokens = new antlr4.CommonTokenStream(lexer);
tokens.fill();

console.log('Tokens for: SELECT "MyColumn" FROM t');
for (const token of tokens.tokens) {
    if (token.type !== antlr4.Token.EOF) {
        const symbolicName = SqlBaseLexer.symbolicNames[token.type];
        console.log(`  Type: ${token.type} (${symbolicName}), Text: "${token.text}"`);
    }
}
