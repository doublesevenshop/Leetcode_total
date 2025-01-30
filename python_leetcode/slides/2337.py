from ttest import *

class Solution:
    def canChange(self, start: str, target: str) -> bool:
        i = j = 0
        m = len(start)
        
        while i < m or j < m:
            while i < m and start[i] == '_':
                i += 1
            
            while j < m and target[j] == '_':
                j += 1
            if m in (i, j):
                return i ==j == m
            if start[i] != target[j]:
                return False 
            if start[i] == 'L':
                if i < j:
                    return False
            else:
                if i > j:
                    return False
            i += 1
            j += 1
        return True
                
                
start = "_L__R__R_"
target = "L______RR"

print(Solution().canChange(start, target))
        