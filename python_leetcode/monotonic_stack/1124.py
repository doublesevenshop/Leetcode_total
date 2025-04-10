from typing import * 
class Solution:
    def longestWPI(self, hours: List[int]) -> int:
        st = [0]
        n = len(hours)
        nums = [0] * (n+1)
        
        for i, h in enumerate(hours, 1):
            nums[i] = nums[i-1] + (1 if h > 8 else -1)
            if nums[i] < nums[st[-1]]:
                st.append(i)
        
        ans = 0
        
        for i in range(n, 0, -1):
            while st and nums[i] > nums[st[-1]]:
                ans = max(ans, i - st.pop())
        return ans 
        
                


if __name__ == "__main__":
    sol = Solution()
    
    print(sol.longestWPI([9,9,6,0,6,6,9]))