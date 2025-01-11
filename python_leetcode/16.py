from ttest import *


class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums.sort()
        
        closest = int(1e9)
        
        for i in range(len(nums)):
            left, right = i+1, len(nums)-1
            
            while left < right and closest != target:
                sum = nums[i] + nums[left] + nums[right]
                
                if abs(target - sum) < abs(target - closest):
                    closest = sum
                
                if sum < target:
                    left += 1
                else:
                    right -= 1
                
        return closest
    

        
nums = [0,0,0]
target = 1
assert Solution().threeSumClosest(nums, target) == 0