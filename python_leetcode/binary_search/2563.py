from ttest import *

class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        n = len(nums)
        ans = 0
        if n == 2:
            sum = nums[0] + nums[1]
            if sum >= lower and sum <= upper:
                return 1
            else:
                return 0
        for j, x in enumerate(nums):
            r = bisect_right(nums, upper-x, 0, j)
            l = bisect_left(nums, lower-x, 0, j)
            
            ans += r - l
        return ans 