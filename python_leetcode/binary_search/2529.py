from ttest import *

class Solution:
    def higher_bound(self, nums: List[str], target: int) -> int:
        n = len(nums)
        
        left, right = 0, n-1
        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] > target:
                right = mid - 1
            else:
                left = mid + 1
        return right
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
    def maximumCount(self, nums: List[int]) -> int:
        left = self.lower_bound(nums, 0)
        right = self.higher_bound(nums, 0)
        
        return max(left, len(nums)-right-1)
    
nums = [0, 0, 0, 0]
print(Solution().maximumCount(nums))
        