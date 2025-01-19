from ttest import *

class Solution:
    def incremovableSubarrayCount(self, nums: List[int]) -> int:
        ans = 0
        size = len(nums)
        left = 0
        
        while left < size - 1:
            if nums[left] >= nums[left+1]:
                break
            left += 1
        
        if left == size - 1:
            return 1 * size * (size + 1) // 2
        
        # n+2是精髓
        ans += left + 2
        
        for right in range(size-1, 0, -1):
            if right < size-1 and nums[right] >= nums[right+1]:
                break
            while left >= 0 and nums[left] >= nums[right]:
                left -= 1
            ans += left + 2
        return ans
        
        
        
        

nums = [6, 5, 7, 8]
sol = Solution()
print(sol.incremovableSubarrayCount(nums))
        