from ttest import *

class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        n = len(nums)
        
        if n <= 2:
            return n

        slow = 0
        count = 1
        
        for fast in range(1, n):
            if nums[fast] == nums[fast-1]:
                count += 1
            else:
                count = 1
            if count <= 2:
                slow += 1
                nums[slow] = nums[fast]
        
        return slow+1
    

nums = [1, 1, 1, 2, 2, 3]
sol = Solution()
print(sol.removeDuplicates(nums), nums)

