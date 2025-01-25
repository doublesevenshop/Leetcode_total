from ttest import *
class Solution:
    def breakfastNumber(self, staple: List[int], drinks: List[int], x: int) -> int:
        staple.sort()
        drinks.sort()
        m, n = len(staple), len(drinks)
        (i, j) = 0, n-1
        ans = 0
        while i < m and j >= 0:
            if staple[i] + drinks[j] <= x:
                ans += j + 1
                i += 1
            else:
                j -= 1
        return ans % 1000000007


staple = [10, 20, 5]
drinks = [5, 5, 2]
x = 15
print(Solution().breakfastNumber(staple, drinks, x))