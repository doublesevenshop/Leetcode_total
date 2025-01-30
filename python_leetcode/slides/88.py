from ttest import *

class Solution:
    def merge(self, n1: List[int], m: int, n2: List[int], n: int) -> None:
        (i, j, k) = (m-1, n-1, m+n-1)
        
        while i >= 0 and j >= 0:
            if n1[i] >= n2[j]:
                n1[k] = n1[i]
                i -= 1
            else:
                n1[k] = n2[j]
                j -= 1
            k -= 1
        
        while j >= 0:
            n1[k] = n2[j]
            j -= 1
            k -= 1
        
    
nums1 = [1, 2, 3, 0, 0, 0]
nums2 = [2, 5, 6]
m, n = 3, 3
Solution().merge(nums1, m, nums2, n)
print(nums1)
