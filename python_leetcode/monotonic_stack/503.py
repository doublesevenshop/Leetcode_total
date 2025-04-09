from typing import *
class Solution:
    def nextGreaterElements(self, nums: List[int]) -> List[int]:
        n = len(nums)
        ans = [-1] * n 
        st = []

        for i in range(2*n):
            t = nums[i%n]
            while st and t > nums[st[-1]]:
                idx = st.pop()
                ans[idx] = nums[i%n]
            st.append(i%n)
        return ans 