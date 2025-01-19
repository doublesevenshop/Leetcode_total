from ttest import *

class Solution:
    def applyOperations(self, nums: List[int]) -> List[int]:
        n = len(nums)
        
        for i in range(n-1):
            if nums[i] == nums[i+1]:
                nums[i] = nums[i]*2
                nums[i+1] = 0
        
        slow, fast = -1, 0
        for fast in range(n):
            if nums[fast] != 0:
                slow += 1
                nums[slow] = nums[fast]
        
        for i in range(slow+1, n):
            nums[i] = 0
                
            
        
        return nums
    
nums = [1, 2, 2, 1, 1, 0]
print(Solution().applyOperations(nums))