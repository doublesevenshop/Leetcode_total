from ttest import *

class Solution:
    def answerQueries(self, nums: List[int], queries: List[int]) -> List[int]:
        nums.sort()
        
        for i in range(1, len(nums)):
            nums[i] += nums[i-1]
            
        for i, q in enumerate(queries):
            queries[i] = bisect_right(nums, q)
        return queries


nums = [2, 3, 4, 5]
queries = [1]
print(Solution().answerQueries(nums, queries))