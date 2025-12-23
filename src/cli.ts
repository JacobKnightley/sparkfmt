/**
 * CLI for Spark SQL Formatter
 */
import { formatSql } from './formatter.js';
import { formatFabricNotebook } from './magic-sql-extractor.js';
import * as fs from 'fs';
import * as readline from 'readline';

const args = process.argv.slice(2);

/**
 * Format content based on file extension.
 * .py files are treated as potential Fabric notebooks.
 */
function formatContent(content: string, filePath?: string): string {
    if (filePath && filePath.endsWith('.py')) {
        return formatFabricNotebook(content, formatSql);
    }
    return formatSql(content);
}

// Helper to output formatted SQL (to file or stdout)
function output(formatted: string, outputFile?: string) {
    if (outputFile) {
        fs.writeFileSync(outputFile, formatted, 'utf-8');
    } else {
        console.log(formatted);
    }
}

/**
 * Detect the line ending style used in content.
 * Returns '\r\n' for Windows or '\n' for Unix.
 */
function detectLineEnding(content: string): '\r\n' | '\n' {
    // Check for CRLF first (Windows)
    if (content.includes('\r\n')) {
        return '\r\n';
    }
    return '\n';
}

/**
 * Normalize line endings in content to Unix style (\n).
 */
function normalizeLineEndings(content: string): string {
    return content.replace(/\r\n/g, '\n');
}

/**
 * Convert line endings in content to the specified style.
 */
function convertLineEndings(content: string, lineEnding: '\r\n' | '\n'): string {
    // First normalize to \n, then convert to target
    const normalized = normalizeLineEndings(content);
    if (lineEnding === '\r\n') {
        return normalized.replace(/\n/g, '\r\n');
    }
    return normalized;
}

/**
 * Batch mode: format multiple files in-place
 * Returns JSON with results for each file
 */
function batchFormat(files: string[]): void {
    const results: { file: string; status: 'formatted' | 'unchanged' | 'error'; error?: string }[] = [];
    
    for (const file of files) {
        try {
            const content = fs.readFileSync(file, 'utf-8');
            const originalLineEnding = detectLineEnding(content);
            
            // Normalize to \n for formatting, then restore original line endings
            const normalizedContent = normalizeLineEndings(content);
            const formatted = formatContent(normalizedContent, file);
            const formattedWithOriginalEndings = convertLineEndings(formatted, originalLineEnding);
            
            if (formattedWithOriginalEndings !== content) {
                fs.writeFileSync(file, formattedWithOriginalEndings, 'utf-8');
                results.push({ file, status: 'formatted' });
            } else {
                results.push({ file, status: 'unchanged' });
            }
        } catch (e: any) {
            results.push({ file, status: 'error', error: e.message });
        }
    }
    
    // Output JSON for easy parsing by caller
    console.log(JSON.stringify(results));
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
  sparkfmt [options] [file...]
  echo "select * from t" | sparkfmt

Options:
  -h, --help          Show this help message
  -o, --output FILE   Write output to FILE instead of stdout
  -c, --check         Check if file needs formatting (exit 1 if so)
  -i, --inline        Format SQL provided as argument
  -b, --batch         Batch mode: format multiple files in-place (JSON output)

Examples:
  sparkfmt query.sql                    Format file to stdout
  sparkfmt query.sql -o formatted.sql   Format file to output file
  sparkfmt -i "select * from t"         Format inline SQL
  sparkfmt -c query.sql                 Check if formatting needed
  sparkfmt -b file1.sql file2.sql       Batch format files in-place
`);
} else if (remainingArgs[0] === '-b' || remainingArgs[0] === '--batch') {
    // Batch mode: format multiple files in-place
    const files = remainingArgs.slice(1);
    if (files.length === 0) {
        console.error('Error: No files specified for --batch');
        process.exit(2);
    }
    batchFormat(files);
} else if (remainingArgs[0] === '-i' || remainingArgs[0] === '--inline') {
    const sql = remainingArgs.slice(1).join(' ');
    output(formatSql(sql), outputFile);
} else if (remainingArgs[0] === '-c' || remainingArgs[0] === '--check') {
    const file = remainingArgs[1];
    if (!file) {
        console.error('Error: No file specified for --check');
        process.exit(2);
    }
    const content = fs.readFileSync(file, 'utf-8');
    const formatted = formatContent(content, file);
    if (formatted !== content) {
        console.log(`File ${file} needs formatting`);
        process.exit(1);
    }
    console.log(`File ${file} is properly formatted`);
} else {
    // File argument
    const file = remainingArgs[0];
    try {
        const content = fs.readFileSync(file, 'utf-8');
        output(formatContent(content, file), outputFile);
    } catch (e: any) {
        console.error(`Error reading file: ${e.message}`);
        process.exit(1);
    }
}
