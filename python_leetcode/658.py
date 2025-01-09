from ttest import *
class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        ans = [0] * k
        left, right = 0, len(arr) - 1
        a, b= abs(arr[left]-x), abs(arr[right]-x)
        
        while right - left > k-1:
            if a > b:
                left += 1
                a = abs(arr[left] - x)
            else:
                right -= 1
                b = abs(arr[right] - x)

        for i in range(k):
            ans[i] = arr[left]
            left += 1
        return ans

sol = Solution()

assert sol.findClosestElements([1,2,3,4,5], 4, 3) == [1,2,3,4]
        