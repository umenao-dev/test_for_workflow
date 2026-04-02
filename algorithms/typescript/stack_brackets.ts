function isBalanced(text: string): boolean {
  const stack: string[] = [];
  const pairs: Record<string, string> = { ')': '(', ']': '[', '}': '{' };

  for (const ch of text) {
    if (ch === '(' || ch === '[' || ch === '{') stack.push(ch);
    if (ch in pairs) {
      const top = stack.pop();
      if (top !== pairs[ch]) return false;
    }
  }

  return stack.length === 0;
}

console.log(isBalanced('(a+[b*c]-{d/e})'));
console.log(isBalanced('(a+[b*c]-{d/e}'));
