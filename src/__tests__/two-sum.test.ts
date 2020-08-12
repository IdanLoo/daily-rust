import twoSum from '../two-sum'

test('should pass given example', () => {
  const nums = [2, 7, 11, 15]
  const target = 9

  const actual = twoSum(nums, target)

  expect(actual).toEqual([0, 1])
})

test('should return [2, 3] when given nums [0, 1, 2, 3, 6] and target 5', () => {
  const nums = [0, 1, 2, 3, 6]
  const target = 5

  const actual = twoSum(nums, target)

  expect(actual).toEqual([2, 3])
})
