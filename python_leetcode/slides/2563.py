from ttest import *

class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        ans = 0
        nums.sort()
        n = len(nums)
        left = right = n
        
        for j, x in enumerate(nums):
            while right and nums[right-1] > upper-x:
                right -= 1
            while left and nums[left - 1] >= lower-x:
                left -= 1
            ans += min(right, j) - min(left, j)
        return ans 

nums = [0, 1, 7, 4, 4, 5]
lower = 3
upper = 6
print(Solution().countFairPairs(nums, lower, upper))