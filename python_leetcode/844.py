from ttest import *

class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        m, n = len(s), len(t)
        list_s = list(s)
        list_t = list(t)
        idx_a = 0
        for i in range(m):
            if list_s[i] != '#':
                list_s[idx_a] = list_s[i]
                idx_a += 1
            else:
                idx_a -= 1 if idx_a > 0 else 0
        idx_b = 0
        
        for i in range(n):
            if list_t[i] != '#':
                list_t[idx_b] = list_t[i]
                idx_b += 1
            else:
                idx_b -= 1 if idx_b > 0 else 0
        return list_s[:idx_a] == list_t[:idx_b] 
            

s = "ab#c"
t = "ad#c"

print(Solution().backspaceCompare(s, t))