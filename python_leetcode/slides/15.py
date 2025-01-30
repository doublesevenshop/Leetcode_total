from ttest import *

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res = []
        
        for l in range(len(nums)):
            if l > 0 and nums[l] == nums[l-1]:
                continue
            m = l + 1
            r = len(nums) - 1
            
            while m < r:
                if nums[l] + nums[m] + nums[r] == 0:
                    res.append([nums[l], nums[m], nums[r]])
                    m += 1
                    r -= 1
                    while m < r and nums[m] == nums[m - 1]:
                        m += 1
                    while m < r and nums[r] == nums[r + 1]:
                        r -= 1
                        
                elif nums[l] + nums[m] + nums[r] < 0:
                    m += 1
                else:
                    r -= 1
                    
        return res
        
    
nums = [-1, 0, 1, 2, -1, -4]
print(Solution().threeSum(nums))
            