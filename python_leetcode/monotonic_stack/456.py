from typing import * 
from sortedcontainers import SortedList

"""
对于三个节点，同时for循环三个会十分麻烦，可以只遍历一个，同时维护左右两个点    
"""

class Solution:
    def find132pattern(self, nums: List[int]) -> bool:
        n = len(nums)
        if n < 3:
            return False 
        
        left_min = nums[0]
        
        right_all = SortedList(nums[2:])
        
        for j in range(1, n-1):
            if left_min < nums[j]:
                index = right_all.bisect_right(left_min)
                if index < len(right_all) and right_all[index] < nums[j]:
                    return True 
            left_min = min(left_min, nums[j])
            right_all.remove(nums[j+1])
        
        return False
    

        
        
    
if __name__ == "__main__":
    sol = Solution()
    
    nums = [-2,1,2,-2,1,2]
    print(nums)
    print(sol.find132pattern(nums))