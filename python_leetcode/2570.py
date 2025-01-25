from ttest import *

class Solution:
    def mergeArrays(self, nums1: List[List[int]], nums2: List[List[int]]) -> List[List[int]]:
        ans = collections.Counter()
        
        for x, y in nums1: ans[x] += y
        for x, y in nums2: ans[x] += y
        
        return list(sorted(ans.items()))
        
nums1 = [[1, 2], [2, 3], [4, 5]]
nums2 = [[1, 4], [3, 2], [4, 1]]

print(Solution().mergeArrays(nums1, nums2))
