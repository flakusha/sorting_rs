from typing import Sequence, List

def bitonic_sort(up: bool, x: Sequence[int]) -> List[int]:
    """Bitonic sort.

    Args:
      up: ascending if ''up'' is true, and decreasing otherwise.
      x: A sequence of integers.

    Returns:
      Sorted sequence of integers.
    """
    if len(x) <= 1:
        return x
    else: 
        first = bitonic_sort(True, x[:len(x) // 2])
        second = bitonic_sort(False, x[len(x) // 2:])
        return bitonic_merge(up, first + second)

def bitonic_merge(up: bool, x) -> List[int]: 
    # Assume input x is bitonic, and sorted list is returned 
    if len(x) == 1:
        return x
    else:
        bitonic_compare(up, x)
        first = bitonic_merge(up, x[:len(x) // 2])
        second = bitonic_merge(up, x[len(x) // 2:])
        return first + second

def bitonic_compare(up: bool, x) -> None:
    dist = len(x) // 2
    for i in range(dist):
        if (x[i] > x[i + dist]) == up:
            x[i], x[i + dist] = x[i + dist], x[i]  # Swap

array = [10, 20, 11, 24, 15]
array = bitonic_sort(True, array)
print(array)