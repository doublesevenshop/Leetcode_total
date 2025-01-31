from ttest import *

class Solution:
    
    def findSingleValue(self, arr1: List[int], value: int, target: int) -> List[int]:
        left = bisect_left(arr1, value-target)
        right = bisect_right(arr1, value+target)
        return left == right


        
    def findTheDistanceValue(self, arr1: List[int], arr2: List[int], d: int) -> int:
        arr1.sort()
        arr2.sort()
        ans = 0
        for i in arr1:
            if self.findSingleValue(arr2, i, d):
                ans += 1
                
        return ans
arr1 = [4, 5, 8]
arr2 = [10, 9, 1, 8]
d = 2
print(Solution().findTheDistanceValue(arr1, arr2, d))