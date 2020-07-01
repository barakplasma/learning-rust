def first_non_consecutive(arr):
    previous = None
    for n in arr:
        if previous and n-1 != previous:
            return n
        previous = n
    return None