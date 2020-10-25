# https://www.codewars.com/kata/reviews/54aebf4d1ea50a119c0001a9/groups/5f9555ec58b84e000168d958
# https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1/train/python
def snail(snail_map):
    vector = []
    while len(snail_map) > 0:
        # Steal the first row.
        vector.extend(snail_map.pop(0))
        # Steal the right items.
        for i in range(len(snail_map)):
            vector.append(snail_map[i].pop())
        # Steal the bottom row.
        vector.extend(list(reversed(snail_map.pop() if snail_map else [])))
        # Steal the left items.
        for i in reversed(range(len(snail_map))):
            vector.append(snail_map[i].pop(0))
    return vector

def assert_equals(a,e):
  return print(a==e)

array = [[1,2,3],
         [4,5,6],
         [7,8,9]]
expected = [1,2,3,6,9,8,7,4,5]
assert_equals(snail(array), expected)


array = [[1,2,3],
         [8,9,4],
         [7,6,5]]
expected = [1,2,3,4,5,6,7,8,9]
assert_equals(snail(array), expected)

array = [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15], [16, 17, 18, 19, 20], [21, 22, 23, 24, 25]]
assert_equals(snail(array), [1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12, 13])