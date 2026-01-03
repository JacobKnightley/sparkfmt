/**
 * Span Calculator - Parse tree span length calculations
 *
 * Pure functions for calculating the formatted length of parse tree contexts.
 * Used for determining when to expand multi-line constructs and for
 * simple query detection.
 */

/**
 * Recursively collect all token texts from a parse tree node.
 */
function collectTokenTexts(node: any, tokens: string[]): void {
  if (!node) return;
  if (node.symbol) {
    // This is a terminal node (token)
    tokens.push(node.symbol.text || '');
  } else if (node.children) {
    for (const child of node.children) {
      collectTokenTexts(child, tokens);
    }
  }
}

/**
 * Calculate the expected formatted span length of a context.
 *
 * This walks all tokens within the context and sums:
 * - Each token's text length
 * - One space between each pair of tokens (standard formatting)
 *
 * This gives an accurate estimate of the formatted output length.
 *
 * @param ctx The parse tree context
 * @param forExpansion If true, returns Infinity for multi-line constructs
 *                     to prevent already-expanded constructs from collapsing.
 *                     If false, calculates actual span (for simple query detection).
 */
export function calculateSpanLength(
  ctx: any,
  forExpansion: boolean = true,
): number {
  if (!ctx || !ctx.start || !ctx.stop) return 0;

  // For expansion checking: if the construct spans multiple lines, return Infinity
  // This ensures idempotency: once expanded, it stays expanded
  // For simple query detection: we want the actual span regardless of input layout
  if (forExpansion) {
    const startLine = ctx.start.line;
    const stopLine = ctx.stop.line;
    if (
      startLine !== undefined &&
      stopLine !== undefined &&
      stopLine > startLine
    ) {
      return Infinity;
    }
  }

  // Collect all tokens within this context by walking the tree
  const tokens: string[] = [];
  collectTokenTexts(ctx, tokens);

  if (tokens.length === 0) {
    // Fallback to character-based
    const startIdx = ctx.start.start;
    const stopIdx = ctx.stop.stop;
    if (startIdx === undefined || stopIdx === undefined) return 0;
    return stopIdx - startIdx + 1;
  }

  // Sum token lengths + (n-1) spaces between tokens
  const tokenLengths = tokens.reduce((sum, t) => sum + t.length, 0);
  const spaceBetween = Math.max(0, tokens.length - 1);

  return tokenLengths + spaceBetween;
}

/**
 * Calculate normalized span length independent of input formatting.
 * This sums up token text lengths + single spaces between tokens,
 * giving a consistent "single-line" representation length.
 *
 * CRITICAL FOR IDEMPOTENCY: Using character positions (calculateSpanLength)
 * varies based on how the input is formatted (line breaks, extra spaces).
 * This causes different expansion decisions on subsequent passes.
 * By using token text lengths, we get consistent results regardless of input formatting.
 */
export function calculateNormalizedSpanLength(ctx: any): number {
  if (!ctx || !ctx.start || !ctx.stop) return 0;

  // Walk through all tokens in the context and sum their text lengths
  let totalLength = 0;
  let tokenCount = 0;

  const collectTokens = (node: any): void => {
    if (!node) return;

    // If this is a terminal (token), add its text length
    if (node.symbol) {
      const text = node.symbol.text;
      if (text) {
        totalLength += text.length;
        tokenCount++;
      }
      return;
    }

    // Recurse into children
    if (node.children) {
      for (const child of node.children) {
        collectTokens(child);
      }
    }
  };

  collectTokens(ctx);

  // Add single space between each token (normalized spacing)
  if (tokenCount > 1) {
    totalLength += tokenCount - 1;
  }

  return totalLength;
}
