import SqlBaseLexer from './dist/generated/SqlBaseLexer.js';
import antlr4 from 'antlr4';

// Test with original case
console.log('=== Original case: 1.23e10 ===');
let sql = 'SELECT 1.23e10 FROM t';
let chars = new antlr4.InputStream(sql);
let lexer = new SqlBaseLexer(chars);
let tokens = new antlr4.CommonTokenStream(lexer);
tokens.fill();
for (const token of tokens.tokens) {
    if (token.type !== antlr4.Token.EOF && token.type !== SqlBaseLexer.WS) {
        const symbolicName = SqlBaseLexer.symbolicNames[token.type];
        console.log(`  ${symbolicName}: "${token.text}"`);
    }
}

// Test with uppercase
console.log('\n=== Uppercase: 1.23E10 ===');
sql = 'SELECT 1.23E10 FROM T';
chars = new antlr4.InputStream(sql);
lexer = new SqlBaseLexer(chars);
tokens = new antlr4.CommonTokenStream(lexer);
tokens.fill();
for (const token of tokens.tokens) {
    if (token.type !== antlr4.Token.EOF && token.type !== SqlBaseLexer.WS) {
        const symbolicName = SqlBaseLexer.symbolicNames[token.type];
        console.log(`  ${symbolicName}: "${token.text}"`);
    }
}
