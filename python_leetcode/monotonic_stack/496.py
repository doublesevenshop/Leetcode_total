from typing import *
class Solution:
    def nextGreaterElement(self, nums1: List[int], nums2: List[int]) -> List[int]:
        def temp(t: List[int]) -> List[int]:
            n = len(t)
            ans = [-1] * n 
            st = []
            
            for i, single in enumerate(t):
                while st and single > t[st[-1]]:
                    idx = st.pop()
                    ans[idx] = t[i]
                st.append(i)
            return ans 
        def find(num: int, nums: List[int]):
            for i in range(len(nums)):
                if num == nums[i]:
                    return i 
            return -1 
        ans = [] * len(nums1)
        list1 = temp(nums2)
        for i in range(len(nums1)):
            idx = find(nums1[i], nums2)
            ans.append(list1[idx])
        return ans 
    
if __name__ == "__main__":
    nums1 = [4,1,2]
    nums2 = [1,3,4,2]
    
    print(Solution().nextGreaterElement(nums1, nums2))