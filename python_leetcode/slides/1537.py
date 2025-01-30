from typing import List

class Solution:
    def maxSum(self, nums1: List[int], nums2: List[int]) -> int:
        sum1, sum2 = 0, 0
        i = j = 0
        MOD = (10**9+7)
        m, n = len(nums1), len(nums2)

        while i < m and j < n:
            if nums1[i] < nums2[j]:
                sum1 += nums1[i]
                i += 1
            elif nums2[j] < nums1[i]:
                sum2 += nums2[j]
                j += 1
            else:
                sum1 = sum2 = (max(sum1, sum2) +nums1[i]) % MOD 
                i += 1
                j += 1
        sum1, sum2 = sum1 + sum(nums1[i:]), sum2 + sum(nums2[j:])

        return max(sum1, sum2) % MOD

# 测试
nums1 = [6, 8, 15, 17, 20, 27, 31, 33]
nums2 = [2, 9, 13, 21, 25, 32, 34]

print(Solution().maxSum(nums1, nums2))  # 应该输出结果
