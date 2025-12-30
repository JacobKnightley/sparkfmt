/**
 * Delta Lake Extension Tests
 * 
 * These tests cover Delta Lake-specific SQL extensions that are NOT in the
 * Apache Spark SQL grammar. The formatter correctly treats these as identifiers
 * (preserves original casing) since they're not recognized keywords.
 * 
 * Note: These tests document expected behavior for Delta Lake syntax,
 * not bugs that need fixing.
 */
import { TestSuite } from './framework.js';

export const deltaLakeTests: TestSuite = {
    name: 'Delta Lake Extensions (Not in Grammar)',
    tests: [
        // VACUUM is a Delta Lake extension for garbage collection
        {
            name: 'VACUUM (not in grammar)',
            input: 'vacuum t',
            expected: 'vacuum t',  // vacuum is not a keyword
        },
        {
            name: 'VACUUM RETAIN (not in grammar)',
            input: 'vacuum t retain 168 hours',
            expected: 'vacuum t retain 168 HOURS',  // HOURS is in grammar, others aren't
        },

        // RESTORE is a Delta Lake time travel feature
        {
            name: 'RESTORE TABLE (not in grammar)',
            input: 'restore table t to version as of 1',
            expected: 'restore TABLE t TO VERSION AS OF 1',  // RESTORE not in grammar
        },

        // CLONE is a Delta Lake table cloning feature
        {
            name: 'CLONE (not in grammar)',
            input: 'create table t clone s',
            expected: 'CREATE TABLE t clone s',  // CLONE not in grammar
        },
        {
            name: 'SHALLOW CLONE (not in grammar)',
            input: 'create table t shallow clone s',
            expected: 'CREATE TABLE t shallow clone s',  // Not in grammar
        },
        {
            name: 'DEEP CLONE (not in grammar)',
            input: 'create table t deep clone s',
            expected: 'CREATE TABLE t deep clone s',  // Not in grammar
        },

        // OPTIMIZE is a Delta Lake command for file compaction
        {
            name: 'OPTIMIZE ZORDER BY (not in grammar)',
            input: 'optimize t zorder by (a, b)',
            expected: 'optimize t zorder BY (a, b)',  // BY is in grammar, others aren't
        },
    ],
};
