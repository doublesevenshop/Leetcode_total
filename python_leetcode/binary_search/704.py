from ttest import *

class Solution:
    def lower_bound(self, nums: List[int], target: int) -> int:
        n = len(nums)
        left, right = 0, n - 1
        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] >= target:
                right = mid - 1
            else:
                left = mid + 1
        return left
    def search(self, nums: List[int], target: int) -> int:
        
        i = self.lower_bound(nums, target)
        return i if i < len(nums) and nums[i] == target else -1
nums = [-1, 0, 3, 5, 9, 12]
target = 9

print(Solution().search(nums, target))