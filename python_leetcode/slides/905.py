from ttest import *

class Solution:
    def sortArrayByParity(self, nums: List[int]) -> List[int]:
        slow = -1
        n = len(nums)
        
        for fast in range(n):
            if nums[fast] % 2 == 0:
                slow += 1
                nums[slow], nums[fast] = nums[fast], nums[slow]
        return nums



nums = [3, 1, 2, 4]
sol = Solution()
print(sol.sortArrayByParity(nums))