from ttest import *

class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        slow, fast = 0, 0
        while fast < len(nums):
            if nums[fast] != val:
                nums[slow] = nums[fast]
                slow += 1
            fast += 1
        return slow
    
nums = [3, 2, 2, 3]
val = 3

sol = Solution()
print(sol.removeElement(nums=nums, val=val))