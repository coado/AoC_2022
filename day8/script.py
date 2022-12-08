from typing import List


def parse_input() -> List[str]:
    trees = []
    
    with open('data.txt') as f:
        for line in f: 
            trees.append(line.replace('\n', ''))

    return trees


def part_one():
    trees = parse_input()
    uncovered_trees = {}
    
    for row_index, row in enumerate(trees):
        if row_index == 0 or row_index == len(trees) - 1:
            continue

        # currently the highest tree
        max_tree = int(row[0])
        for i in range(1, len(row) - 1):

            if int(row[i]) > max_tree:
                max_tree = int(row[i])
                uncovered_trees["({}, {})".format(row_index, i)] = 1

        max_tree = int(row[len(row) - 1])
        for i in range (len(row) - 2, 0, -1):
            if int(row[i]) > max_tree:
                max_tree = int(row[i])
                uncovered_trees["({}, {})".format(row_index, i)] = 1
    for col_index in range(1, len(trees[0]) - 1):
        max_tree = int(trees[0][col_index])
        for i in range(1, len(trees) - 1):
            if int(trees[i][col_index]) > max_tree:
                max_tree = int(trees[i][col_index])
                uncovered_trees["({}, {})".format(i, col_index)] = 1

        max_tree = int(trees[len(trees) - 1][col_index])
        for i in range (len(trees) - 2, 0, -1):
            if int(trees[i][col_index]) > max_tree:
                max_tree = int(trees[i][col_index])
                uncovered_trees["({}, {})".format(i, col_index)] = 1

    total = 0
    for tree in uncovered_trees:
        total += 1

    total += 2*len(trees[0]) + 2*len(trees) - 4
    
    print(total)

# part_one()

def part_two():
    trees = parse_input()

    max_score = float('-inf')

    for row_index in range(0, len(trees[0])):
        for col_index in range(0, len(trees)):
            # result will be 0
            if row_index == 0 or row_index == len(trees) - 1:
                continue
            if col_index == 0 or col_index == len(trees[0]) - 1:
                continue

            current_tree = trees[row_index][col_index]
            result = 1

            traversing_index = 0
            # up
            while row_index - traversing_index - 1 >= 0:
                if trees[row_index - traversing_index - 1][col_index] < current_tree:
                    traversing_index += 1
                else:
                    traversing_index += 1
                    break
            
            result *= traversing_index

            traversing_index = 0
            #down
            while row_index + traversing_index + 1 <= len(trees) - 1:
                if trees[row_index + traversing_index + 1][col_index] < current_tree:
                    traversing_index += 1
                else:
                    traversing_index += 1
                    break

            result *= traversing_index

            traversing_index = 0
            # left
            while col_index - traversing_index - 1 >= 0:
                if trees[row_index][col_index - traversing_index - 1] < current_tree:
                    traversing_index += 1
                else:
                    traversing_index += 1
                    break
            result *= traversing_index

            traversing_index = 0
            # right
            while col_index + traversing_index + 1 <= len(trees[0]) - 1:
                if trees[row_index][col_index + traversing_index + 1] < current_tree:
                    traversing_index += 1
                else:
                    traversing_index += 1
                    break
            result *= traversing_index
            
            max_score = max(max_score, result)

    return max_score

print(part_two())