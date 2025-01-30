from ttest import *

class Solution:
    def camelMatch(self, queries: List[str], pattern: str) -> List[bool]:
        n = len(queries)
        
        ans = [True] * n
        
        for i, query in enumerate(queries):
            j = 0
            for q in query:
                if (j == len(pattern) and q.isupper() or (j < len(pattern) and q.isupper() and q != pattern[j])):
                    ans[i] = False
                    break
                if j < len(pattern) and q == pattern[j]:
                    j += 1
            if j < len(pattern):
                ans[i] = False 
        return ans 