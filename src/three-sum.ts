function removeDuplicated(nums: number[]) {
  return Array.from(new Set(nums))
}

function threeSum(nums: number[]): number[][] {
  const map = new Map<number, [number, number][]>()
  nums = removeDuplicated(nums).sort()
  const result: number[][] = []

  for (let i = 0; i < nums.length; i += 1) {
    const a = nums[i]

    for (let j = i + 1; j < nums.length; j += 1) {
      const b = nums[j]
      const expected = -b

      if (map.has(expected)) {
        result.pu
      }
    }
  }
}

export default threeSum
