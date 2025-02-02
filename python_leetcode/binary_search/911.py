from ttest import *

class TopVotedCandidate:
    
    def __init__(self, persons: List[int], times: List[int]):
        tops = []
        voteCounts = defaultdict(int)
        voteCounts[-1] = -1
        top = -1
        for p in persons:
            voteCounts[p] += 1
            if voteCounts[p] >= voteCounts[top]:
                top = p
            tops.append(top)
        self.tops = tops
        self.times = times
        
    def q(self, t: int) -> int:
        l, r = 0, len(self.times) - 1
        # 找到满足 times[l] <= t 的最大的 l
        # 也可以使用内置的二分查找的包来确定 l
        l = bisect.bisect(self.times, t) - 1
        return self.tops[l]

# Your TopVotedCandidate object will be instantiated and called as such:
# obj = TopVotedCandidate(persons, times)
# param_1 = obj.q(t)