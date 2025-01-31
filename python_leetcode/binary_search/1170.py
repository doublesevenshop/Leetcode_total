from ttest import *

class Solution:
    def f(self, arr: List[str]) -> int:
        arr = list(arr)
        arr.sort()
        dic = [0]*26
        for i in arr:
            dic[ord(i)-ord('a')] += 1
        return dic[ord(arr[0])-ord('a')]
    
    def numSmallerByFrequency(self, queries: List[str], words: List[str]) -> List[int]:
        lq = [0]*len(queries)
        lw = [0]*len(words)
        
        for i in range(len(queries)):
            lq[i] = self.f(queries[i])
        for i in range(len(words)):
            lw[i] = self.f(words[i])
        lw.sort()
        
        ans =[0]*len(queries)
        
        for i in range(len(lq)):
            ans[i] = len(words)-bisect_right(lw, lq[i])
        return ans         
nums = ["bbb", "cc"]
words = ["a", "aa", "aaa", "aaaa"]
# nums = ["cbd"]
# words = ["zaaaz"]
print(Solution().numSmallerByFrequency(nums, words))