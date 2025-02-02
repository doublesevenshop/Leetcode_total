from ttest import *

class Solution:
    def minAbsoluteSumDiff(self, a: List[int], b: List[int]) -> int:
        n = len(a)
        diff = sum(abs(a[i]-b[i]) for i in range(n))
        
        ans = math.inf 
        MOD = 10**9+7
        asorted = sorted(a)
        for i, num in enumerate(b):
            idx = bisect_left(asorted, num)
            tmp = diff - abs(a[i]-b[i])
            
            if idx > 0:
                ans = min(ans, tmp+abs(a[idx]-b[i]))
            if idx < n:
                ans = min(ans, tmp+abs(a[idx-1]-b[i]))
        return ans 

        
    
nums1 = [1, 7, 5]
nums2 = [2, 3, 5]
print(Solution().minAbsoluteSumDiff(nums1, nums2))
                