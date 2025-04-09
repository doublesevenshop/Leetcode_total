from typing import * 
class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        n = len(temperatures)
        ans = [0] * n 
        st = []
        
        for i, t in enumerate(temperatures):
            while st and t > temperatures[st[-1]]:
                idx = st.pop()
                ans[idx] = i - idx 
            st.append(i)
        return ans 
    


if __name__ == "__main__":
    sol = Solution()
    temp = [73,74,75,71,69,72,76,73]
    print(sol.dailyTemperatures(temp))