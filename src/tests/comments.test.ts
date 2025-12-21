/**
 * Comment and Hint Tests
 */
import { TestSuite } from './framework.js';

export const commentTests: TestSuite = {
    name: 'Comments',
    tests: [
        {
            name: 'Leading line comment',
            input: '-- header comment\nselect x from t',
            expected: '-- header comment\nSELECT x\nFROM t',
        },
        {
            name: 'Leading block comment',
            input: '/* multi\nline */\nselect x from t',
            expected: '/* multi\nline */\nSELECT x\nFROM t',
        },
        {
            name: 'Trailing inline comment',
            input: 'select x from t -- inline comment',
            expected: 'SELECT x\nFROM t -- inline comment',
        },
        {
            name: 'Comment in WHERE clause',
            input: 'select x from t where /* filter */ a = 1',
            expected: 'SELECT x\nFROM t\nWHERE /* filter */ a = 1',
        },
    ],
};

export const hintTests: TestSuite = {
    name: 'Hints',
    tests: [
        {
            name: 'BROADCAST hint',
            input: 'select /*+ broadcast(t) */ * from t',
            expected: 'SELECT /*+ BROADCAST(t) */ *\nFROM t',
        },
        {
            name: 'Multiple hints',
            input: 'select /*+ broadcast(t1), merge(t2) */ * from t',
            expected: 'SELECT /*+ BROADCAST(t1), MERGE(t2) */ *\nFROM t',
        },
        {
            name: 'Hint preserves table name casing',
            input: 'select /*+ broadcast(MyTable) */ * from MyTable',
            expected: 'SELECT /*+ BROADCAST(MyTable) */ *\nFROM MyTable',
        },
    ],
};
