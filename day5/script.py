from typing import List


def part_one():
    stacks: List[List[str]] = [
        ['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        ['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        ['H', 'P', 'T', 'R'],
        ['Z', 'S', 'N', 'P', 'H', 'T'],
        ['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        ['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        ['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        ['N', 'D', 'S'],
        ['D', 'Z', 'S', 'F', 'M']
    ]


    with open('data.txt') as f:
        for line in f:
            [_, amount, _, start_index, _, end_index] = line.split(' ')
            amount = int(amount)
            start_index = int(start_index)
            end_index = int(end_index)

            for _ in range(amount):
                moved_el = stacks[start_index - 1].pop(-1)
                stacks[end_index - 1].append(moved_el)


    result = ''
    for stack in stacks:
        result += stack[-1]
    return result


# print(part_one())

def part_two():
    stacks: List[List[str]] = [
        ['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        ['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        ['H', 'P', 'T', 'R'],
        ['Z', 'S', 'N', 'P', 'H', 'T'],
        ['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        ['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        ['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        ['N', 'D', 'S'],
        ['D', 'Z', 'S', 'F', 'M']
    ]


    with open('data.txt') as f:
        for line in f:
            [_, amount, _, start_index, _, end_index] = line.split(' ')
            amount = int(amount)
            start_index = int(start_index)
            end_index = int(end_index)

            elements_to_move = []
            for _ in range(amount):  
                elements_to_move.append(stacks[start_index - 1].pop(-1))
            
            elements_to_move.reverse()

            for el in elements_to_move:
                stacks[end_index - 1].append(el)

    result = ''
    for stack in stacks:
        result += stack[-1]
    return result


print(part_two())