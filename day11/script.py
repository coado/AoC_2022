import functools

def part_one():
    monkeys = {}

    file = open("data.txt", "r")
    line = 'init'
    while line != '':
        line = file.readline()

        monkey = int(line.split(' ')[1].replace(':\n', ''))
        monkeys[monkey] = {}

        line = file.readline()
        items = [int(item.replace(',', '')) for item in line.split(' ')[4:]]
        monkeys[monkey]['items'] = items

        line = file.readline()
        operation = line.split(' ')[6:8]
        monkeys[monkey]['operation'] = operation

        line = file.readline()
        divisible = int(line.split(' ')[-1])
        monkeys[monkey]['divisible'] = divisible

        line = file.readline()
        monkey_to_throw = int(line.split(' ')[-1])
        monkeys[monkey]['true_next_monkey'] = monkey_to_throw

        line = file.readline()
        monkey_to_throw = int(line.split(' ')[-1])
        monkeys[monkey]['false_next_monkey'] = monkey_to_throw

        monkeys[monkey]['inspected_items'] = 0

        line = file.readline()


    modulo = 1
    for _, monkey in monkeys.items():
        modulo *= monkey['divisible']

    for _ in range(10000):
        for _, monkey in monkeys.items():
            for _ in range(len(monkey['items'])):
                item = monkey['items'].pop(0)
                new_value = 0
                operation = monkey['operation']
                coefficient = operation[1].replace('\n', '')
                if operation[0] == '+':
                    if coefficient == 'old':
                        new_value = item + item
                    else:
                        new_value = item + int(coefficient)
                else:
                    if coefficient == 'old':
                        new_value = item * item
                    else:
                        new_value = item * int(coefficient)
                
                # new_value //= 3
                new_value %= modulo

                monkey['inspected_items'] += 1
                if new_value % monkey['divisible'] == 0:
                    monkeys[monkey['true_next_monkey']]['items'].append(new_value)
                else:
                    monkeys[monkey['false_next_monkey']]['items'].append(new_value)


    inspected_items = []
    for _, monkey in monkeys.items():
        inspected_items.append(monkey['inspected_items'])

    inspected_items.sort()
    inspected_items.reverse()
    print(inspected_items)
    print(inspected_items[0] * inspected_items[1])

part_one()