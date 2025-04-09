from ttest import *

class Solution:
    def getTriggerTime(self, increase: List[List[int]], requirements: List[List[int]]) -> List[int]:
        crh = [[0, 0, 0]] * (len(increase)+1)
        res = []
        
        for i in range(len(increase)):
            crh[i+1] = list(map(lambda x, y:x+y, crh[i], increase[i]))
        keys = [x[0] for x in crh]
        
        for req in requirements:
            pos = bisect_left(keys, req[0])
            is_satisfied = False
            
            while pos < len(crh):
                if crh[pos][0] >= req[0] and crh[pos][1] >= req[1] and crh[pos][2] >= req[2]:
                    is_satisfied = True 
                    break 
                pos += 1
            if is_satisfied:
                res.append(pos)
            else:
                res.append(-1)
        return res