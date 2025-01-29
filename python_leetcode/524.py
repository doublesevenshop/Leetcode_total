from ttest import *

class Solution:
    def findLongestWord(self, s: str, dictionary: List[str]) -> str:
        
        def sub(word):
            i = j = 0
            while i < len(s) and j < len(word):
                if s[i] == word[j]:
                    j += 1
                i += 1
            return j == len(word)
        
        dictionary.sort(key=lambda x: (-len(x), x))
        
        for word in dictionary:
            if sub(word):
                return word
        return ""