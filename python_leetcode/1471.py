from ttest import *

class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        left, right = 0, len(numbers) - 1
        ans = []
        while left < right:
            if numbers[left] + numbers[right] == target:
                ans = [left + 1, right + 1]
                break
            elif numbers[left] + numbers[right] < target:
                left += 1
            else:
                right -= 1
        return ans

sol = Solution()
print(sol.twoSum([2,7,11,15], 9))