from typing import *
class Solution:
    def finalPrices(self, prices: List[int]) -> List[int]:
        n = len(prices)
        ans = [0] * n
        st = []
        
        for i, p in enumerate(prices):
            while st and p <= prices[st[-1]]:
                idx = st.pop()
                ans[idx] = prices[idx] - prices[i] 
            st.append(i)
            ans[i] = prices[i]
        return ans             


if __name__ == "__main__":
    prices = [8,4,6,2,3]
    print(Solution().finalPrices(prices))