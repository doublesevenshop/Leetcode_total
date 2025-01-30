from ttest import *

class Solution:
    def duplicateZeros(self, arr: List[int]) -> None:
        n = len(arr)
        
        i = 0
        while i < n:
            if arr[i] == 0:
                arr.insert(i, 0)
                i += 1
            i += 1
        for i in range(n, len(arr)):
            arr.pop()
        

arr = [1, 0, 2, 3, 0, 4, 5, 0]
Solution().duplicateZeros(arr)

print(arr)
        