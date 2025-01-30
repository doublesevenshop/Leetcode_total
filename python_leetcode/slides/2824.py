from ttest import *

class Solution:
    def countPairs(self, nums: List[int], target: int) -> int:
        # 统计和小于目标的下标对数目，要求下标严格递增，所以可以用双指针
        nums.sort()
        n = len(nums)
        l, r = 0, n-1
        res = 0
        while l < r:
            if nums[l] + nums[r] < target:
                res += r - l
                l += 1
            else:
                r -= 1
        return res
        
        
        
    
nums = [-1,1,2,3,1]
target = 2
print(Solution().countPairs(nums, target)) # 1