from ttest import * 
class Solution:
    def successfulPairs(self, spells: List[int], potions: List[int], success: int) -> List[int]:
        potions.sort()
        len_po = len(potions)        
        ans = []
        
        for i in spells:
            pos = bisect_left(potions, success/i)
            ans.append(len_po - pos)
        return ans 

spells = [5, 1, 3]
potions = [1, 2, 3, 4, 5]
success = 7

print(Solution().successfulPairs(spells, potions, success))

