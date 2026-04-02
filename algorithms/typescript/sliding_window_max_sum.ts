function maxWindowSum(nums: number[], k: number): number {
  if (k <= 0 || k > nums.length) return 0;

  let current = 0;
  for (let i = 0; i < k; i++) current += nums[i];

  let best = current;
  for (let i = k; i < nums.length; i++) {
    current += nums[i] - nums[i - k];
    if (current > best) best = current;
  }

  return best;
}

console.log(maxWindowSum([2, 1, 5, 1, 3, 2], 3));
console.log(maxWindowSum([1, 2, 3], 4));
