from ttest import *

class Solution:
    def arithmeticTriplets(self, nums: List[int], diff: int) -> int:
        left = 0
        mid = 1
        right = 2
        n = len(nums)
        ans = 0
        while mid < n-1:
            left = mid - 1
            right = mid + 1
            
            while left >= 0 and right < n:
                li = nums[mid]-nums[left]
                ri = nums[right]-nums[mid]
                
                if ri > diff:
                    break 
                elif ri < diff:
                    right += 1
                else:
                    if li > diff:
                        break
                    elif li < diff:
                        left -= 1
                    else:
                        ans += 1
                        left -= 1
                        right += 1
            mid += 1
        return ans 
nums = [0, 1, 4, 6, 7, 10]
diff = 3
print(Solution().arithmeticTriplets(nums, diff))
            
            
        
        