from ttest import *


class Solution:
    def threeSumMulti(self, arr: List[int], target: int) -> int:
        arr.sort()
        res = 0
        
        for i in range(len(arr)-2):
            l, r = i + 1, len(arr) - 1
            
            while l < r :
                if arr[i] + arr[l] + arr[r] < target:
                    l += 1
                elif arr[i] + arr[l] + arr[r] > target:
                    r -= 1
                else:
                    if arr[l] == arr[r]:
                        res += (r - l + 1) * (r - l) // 2
                        break
                    else:
                        left = 1
                        right = 1
                        while l + left < r and arr[l] == arr[l + left]:
                            left += 1
                        while r - right > l and arr[r] == arr[r - right]:
                            right += 1
                        res += left * right
                        l += left
                        r -= right
                    
        return res % (10**9 + 7)
    

arr = [1,1,2,2,3,3,4,4,5,5]
target = 8
print(Solution().threeSumMulti(arr, target)) # 20