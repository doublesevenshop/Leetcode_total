from ttest import *

class Solution:
    def findTheDistanceValue(self, arr1: List[int], arr2: List[int], d: int) -> int:
        ans = 0
        for num1 in arr1:
            valid = True
            
            for num2 in arr2:
                if abs(num1 - num2) <= d:
                    valid = False
                    break
            if valid:
                ans += 1
        return ans
            
                

arr1 = [4, 5, 8]
arr2 = [10, 9, 1, 8]
d = 2

print(Solution().findTheDistanceValue(arr1, arr2, d))