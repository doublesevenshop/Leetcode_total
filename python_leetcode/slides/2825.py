from ttest import * 
class Solution:
    def canMakeSubsequence(self, s: str, t: str) -> bool:
        i = j = 0
        
        for b in s:
            c = chr(ord(b)+1) if b != 'z' else 'a'
            
            if b == t[j] or c == t[j]:
                j += 1
                
                if j == len(t):
                    return True 
        return False






