from ttest import *

class Solution:
    def recoverArray(self, nums: List[int]) -> List[int]:
        nums.sort()
        
        n = len(nums)
        
        for i in range(1, n):
            if nums[i] == nums[0] or (nums[i]-nums[0]) % 2 != 0:
                continue
            
            used = [False] * n
            used[0] = used[i] = True
            k = (nums[i] - nums[0]) // 2
            
            ans = [nums[0]+k]
            left, right = 0, i
            
            for j in range(1, n//2):
                while used[left]:
                    left += 1
                while right < n and (used[right] or nums[right] - nums[left]) != k * 2 :
                    right += 1
                if right == n:
                    break
                ans.append(nums[left]+k)
                used[left] = used[right] = True
            if len(ans) == n // 2:
                return ans
        return None
                
    
    
nums = [2, 10, 6, 4, 8, 12]
sol = Solution()
print(sol.recoverArray(nums))