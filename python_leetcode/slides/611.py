from ttest import *
class Solution:
    def triangleNumber(self, nums: List[int]) -> int:
        res = 0
        nums.sort()
        for k in range(2, len(nums)):
            c = nums[k]
            i, j = 0, k - 1
            while i < j:
                if nums[i] + nums[j] > c:
                    res += j - i
                    j -= 1
                else:
                    i += 1
        
        return res
    
nums = [2,2,3,4]
print(Solution().triangleNumber(nums))
            
                    
        