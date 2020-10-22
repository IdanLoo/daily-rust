import threeSum from '../three-sum'

test('should pass given example', () => {
  const nums = [-1, 0, 1, 2, -1, -4]

  const actual = threeSum(nums)

  expect(actual).toEqual([
    [-1, 0, 1],
    [-1, -1, 2],
  ])
})
