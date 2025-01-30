from ttest import *

class Solution:
    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        nums.sort()
        res = []
        for i in range(len(nums)):
            j = i + 1
            while j < len(nums):
                k = j + 1
                
                l = len(nums) - 1
                while k < l:
                    sum_ = nums[i] + nums[j] + nums[k] + nums[l]
                    
                    if sum_ == target:
                        res.append([nums[i], nums[j], nums[k], nums[l]])
                        k += 1
                        l -= 1
                    elif sum_ < target:
                        k += 1
                    else:
                        l -= 1
                j += 1
        res = list(set([tuple(sorted(x)) for x in res]))
        return res
    
nums = [2,2,2,2,2]
target = 8
print(Solution().fourSum(nums, target))
                
                
        
                    
                    