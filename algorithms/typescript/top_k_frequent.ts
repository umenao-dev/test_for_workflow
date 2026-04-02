function topKFrequent(nums: number[], k: number): number[] {
  const freq = new Map<number, number>();
  for (const n of nums) freq.set(n, (freq.get(n) ?? 0) + 1);

  const entries = [...freq.entries()];
  entries.sort((a, b) => b[1] - a[1]);

  const result: number[] = [];
  for (let i = 0; i < Math.min(k, entries.length); i++) {
    result.push(entries[i][0]);
  }

  return result;
}

console.log(topKFrequent([1, 1, 1, 2, 2, 3], 2));
console.log(topKFrequent([4, 4, 5], 5));
