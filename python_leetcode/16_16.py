from ttest import *

class Solution:
    def smallestDifference(self, a: List[int], b: List[int]) -> int:
        a.sort()
        b.sort()
        m, n = len(a), len(b)
        mmin = 2147483648
        i = j = 0
        while i < m and j < n:
            mmin = min(mmin, abs(a[i]-b[j]))
            if a[i] > b[j]:
                j += 1
            elif a[i] < b[j]:
                i += 1
            else:
                return 0
        return mmin
        
                
a = [1, 3, 15, 11, 2]
b = [23, 127, 235, 19, 8]
print(Solution().smallestDifference(a, b))
