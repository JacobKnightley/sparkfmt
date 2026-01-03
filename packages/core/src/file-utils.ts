/**
 * File extension utilities for supported file types
 */

/** Supported file extensions for formatting */
export const SUPPORTED_EXTENSIONS = ['.sql', '.py', '.scala', '.r'] as const;

/**
 * Check if a file path has a supported extension for formatting.
 * @param filePath - The file path to check
 * @returns true if the file has a supported extension
 */
export function isSupportedFile(filePath: string): boolean {
  const ext = filePath.slice(filePath.lastIndexOf('.')).toLowerCase();
  return (SUPPORTED_EXTENSIONS as readonly string[]).includes(ext);
}
