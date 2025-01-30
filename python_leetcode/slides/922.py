from ttest import *

class Solution:
    def sortArrayByParityII(self, nums: List[int]) -> List[int]:
        slow, fast = 0, 0
        n = len(nums)
        
        for fast in range(1, n, 2):
            while nums[fast] % 2 == 0:
                nums[slow], nums[fast] = nums[fast], nums[slow]
                slow += 2
        return nums
    
nums = [4, 2, 5, 7]
sol = Solution()
print(sol.sortArrayByParityII(nums))
        