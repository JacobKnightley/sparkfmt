/**
 * CLI for Spark SQL Formatter
 */
import { formatSql } from './formatter.js';
import * as fs from 'fs';
import * as readline from 'readline';

const args = process.argv.slice(2);

// Helper to output formatted SQL (to file or stdout)
function output(formatted: string, outputFile?: string) {
    if (outputFile) {
        fs.writeFileSync(outputFile, formatted, 'utf-8');
        console.log(`Formatted output written to ${outputFile}`);
    } else {
        console.log(formatted);
    }
}

// Parse output file option
function parseOutputOption(args: string[]): { outputFile?: string; remainingArgs: string[] } {
    const outputIdx = args.findIndex(a => a === '-o' || a === '--output');
    if (outputIdx !== -1 && args[outputIdx + 1]) {
        const outputFile = args[outputIdx + 1];
        const remainingArgs = [...args.slice(0, outputIdx), ...args.slice(outputIdx + 2)];
        return { outputFile, remainingArgs };
    }
    return { remainingArgs: args };
}

const { outputFile, remainingArgs } = parseOutputOption(args);

if (remainingArgs.length === 0) {
    // Interactive mode - read from stdin
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });
    
    let sql = '';
    
    rl.on('line', (line) => {
        sql += line + '\n';
    });
    
    rl.on('close', () => {
        const formatted = formatSql(sql.trim());
        output(formatted, outputFile);
    });
} else if (remainingArgs[0] === '--help' || remainingArgs[0] === '-h') {
    console.log(`Spark SQL Formatter

Usage:
  sparkfmt [options] [file]
  echo "select * from t" | sparkfmt

Options:
  -h, --help          Show this help message
  -o, --output FILE   Write output to FILE instead of stdout
  -c, --check         Check if file needs formatting (exit 1 if so)
  -i, --inline        Format SQL provided as argument

Examples:
  sparkfmt query.sql                    Format file to stdout
  sparkfmt query.sql -o formatted.sql   Format file to output file
  sparkfmt -i "select * from t"         Format inline SQL
  sparkfmt -c query.sql                 Check if formatting needed
`);
} else if (remainingArgs[0] === '-i' || remainingArgs[0] === '--inline') {
    const sql = remainingArgs.slice(1).join(' ');
    output(formatSql(sql), outputFile);
} else if (remainingArgs[0] === '-c' || remainingArgs[0] === '--check') {
    const file = remainingArgs[1];
    if (!file) {
        console.error('Error: No file specified for --check');
        process.exit(2);
    }
    const sql = fs.readFileSync(file, 'utf-8');
    const formatted = formatSql(sql);
    if (formatted !== sql) {
        console.log(`File ${file} needs formatting`);
        process.exit(1);
    }
    console.log(`File ${file} is properly formatted`);
} else {
    // File argument
    const file = remainingArgs[0];
    try {
        const sql = fs.readFileSync(file, 'utf-8');
        output(formatSql(sql), outputFile);
    } catch (e: any) {
        console.error(`Error reading file: ${e.message}`);
        process.exit(1);
    }
}
