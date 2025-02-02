from ttest import *

class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        n = len(arr)
        ans = [0] * n 
        
        left, right = 0, n-1
        a, b = abs(arr[left]-x), abs(arr[right]-x)
        
        while right - left + 1 > k:
            if a > b:
                left += 1
                a = abs(arr[left]-x)
            else:
                right -= 1
                b = abs(arr[right]-x)
        
        for i in range(k):
            ans[i] = arr[left]
            left += 1
        return ans 