# 手动实现binary_search


class Binsect:
    def __init__(self, nums, target):
        self.nums = nums
        self.target = target
    
    def left(self):
        left, right = 0, len(self.nums) - 1
        
        while left <= right:
            mid = left + (right - left) // 2
            if self.nums[mid] < self.target:
                left = mid + 1
            else:
                right = mid - 1
        return left 
    
    def right(self):
        left, right = 0, len(self.nums) - 1
        while left <= right:
            mid = left + (right - left) // 2
            if self.nums[mid] > self.target:
                right = mid - 1
            else:
                left = mid + 1
        return right
    
        
        

nums = [1, 1, 2, 2, 2, 4, 5]
target = 3
print(Binsect(nums, target).left())
print(Binsect(nums, target).right())
print(Binsect(nums, target).mid())