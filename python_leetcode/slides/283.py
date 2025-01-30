from ttest import * 

class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        slow = -1
        n = len(nums)
        for fast in range(n):
            if nums[fast] != 0:
                slow += 1
                nums[slow] = nums[fast]
        
        for i in range(slow+1, n):
            nums[i] = 0


nums = [0, 1, 0, 3, 12]
sol = Solution()
sol.moveZeroes(nums)
print(nums)

