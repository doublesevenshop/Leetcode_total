from typing import *
from math import *
class StockSpanner:
    
    def __init__(self):
        st = [(-1, inf)]
        self.cur = -1
        

    def next(self, price: int) -> int:
        while price >= self.st[-1][1]:
            self.st.pop()
        self.cur += 1
        self.st.append((self.cur, price))
        return self.cur - self.st[-2][0]
        
        
        