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
    def nextGreatestLetter(self, letters: List[str], target: str) -> str:
        i = self.higher_bound(letters, target)
        
        return letters[i+1] if i+1 < len(letters) else letters[0] 

nums = ['c', 'f', 'j']
target = 'c'
print(Solution().nextGreatestLetter(nums, target))