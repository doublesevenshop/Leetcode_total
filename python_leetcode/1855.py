from ttest import *

class Solution:
    def maxDistance(self, nums1: List[int], nums2: List[int]) -> int:
        
        m, n = len(nums1), len(nums2)
        i, j = 0, 0
        anx = 0
        while i < m and j < n:
            if nums1[i] <= nums2[j]:
                anx = max(anx, j-i)
                j += 1
            else:
                i += 1
        return anx
                
            
                
        

nums1 = [55, 30, 5, 4, 2]
nums2 = [100, 20, 10, 10, 5]

print(Solution().maxDistance(nums1, nums2))