from ttest import *

class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        i, j = 0, 0
        m, n = len(name), len(typed)
        for j in range(n):
            if i < m and name[i] == typed[j]:
                i += 1
            elif j == 0 or typed[j] != typed[j-1]:
                return False 
        return i == m
        
            

name = "alex"
typed = "aaleex"

print(Solution().isLongPressedName(name, typed))