from typing import List


def binary_search(nums: List[int], target: int) -> int:
    low, high = 0, len(nums) - 1
    while low <= high:
        mid = (low + high) // 2
        if nums[mid] == target:
            return mid
        if nums[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1


if __name__ == "__main__":
    data = [2, 4, 7, 9, 11, 15]
    print(binary_search(data, 9))
    print(binary_search(data, 5))
