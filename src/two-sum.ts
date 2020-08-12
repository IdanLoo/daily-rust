function twoSum(nums: number[], target: number): number[] {
  const map = new Map<number, number>()

  for (let i = 0; i < nums.length; i += 1) {
    const current = nums[i]
    const expected = target - current

    if (map.has(expected)) {
      return [map.get(expected), i]
    }

    map.set(current, i)
  }

  return []
}

export default twoSum
