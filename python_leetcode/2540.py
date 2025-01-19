from ttest import *

class Solution:
    def getCommon(self, nums1: List[int], nums2: List[int]) -> int:
        l1, l2 = 0, 0
        m, n = len(nums1), len(nums2)
        
        while l1 < m and l2 < n:
            
            if nums1[l1] < nums2[l2]:
                l1 += 1
            elif nums1[l1] > nums2[l2]:
                l2 += 1
            elif nums1[l1] == nums2[l2]:
                return nums1[l1]
        return -1    
        
    
nums1 = [1, 2, 3, 6]
nums2 = [2, 3, 4, 5]

print(Solution().getCommon(nums1, nums2))