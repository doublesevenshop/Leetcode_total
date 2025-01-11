from ttest import *

class Solution:
    def numTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        # 三元组的组成方式有两种，一种是两个数相等，一种是两个数不相等
        nums1.sort()
        nums2.sort()
        
        res = 0
        for n in nums1:
            res += self.count(n*n, nums2)
        for n in nums2:
            res += self.count(n*n, nums1)
        
        return res
    
    def count(self, n, nums):
        l, r = 0, len(nums) - 1
        res = 0
        while l < r:
            if nums[l] * nums[r] == n:
                # 如果左右指针指向的数相等，那么这个区间内的数都可以组成三元组
                if nums[l] == nums[r]:
                    res += (r - l + 1) * (r - l) // 2
                    break
                # 如果左右指针指向的数不相等，那么左指针向右移动，右指针向左移动
                else:
                    l1, r1 = l, r
                    # 删除左右指针指向的相同的数
                    while l1 < r and nums[l1] == nums[l]:
                        l1 += 1
                    while r1 > l and nums[r1] == nums[r]:
                        r1 -= 1
                    res += (l1 - l) * (r - r1)
                    l, r = l1, r1
            elif nums[l] * nums[r] < n:
                l += 1
            else:
                r -= 1
        return res
    
nums1 = [1,1]
nums2 = [1,1,1]
print(Solution().numTriplets(nums1, nums2))
