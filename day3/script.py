
def check_priority(letter: str) -> int:
    if ord(letter) >= 97 and ord(letter) <= 122:
        return ord(letter) - 96
    if ord(letter) >= 65 and ord(letter) <= 90:
        return ord(letter) - 64 + 26
    return 0

def part_one():
    priority = 0

    with open('data.txt') as f:
        for line in f:
            indexed_items = {}
            compartment_one = line[slice(0, len(line) // 2)]
            compartment_two = line[slice(len(line) // 2, len(line))]
            
            doubled_item = ''

            for first_item in compartment_one:
                found = False
                if first_item in indexed_items:
                    continue

                for second_item in compartment_two:
                    if first_item == second_item:
                        doubled_item = first_item
                        found = True
                        break
                
                if found:
                    break
                indexed_items[first_item] = True

            priority += check_priority(doubled_item)
    return priority

# print(part_one())

def part_two():
    priority = 0

    with open('data.txt') as f:
        while True:
            indexed_items = {}
            rucksack_one = f.readline()
            rucksack_two = f.readline()
            rucksack_three = f.readline()

            if rucksack_one == '' or rucksack_two == '' or rucksack_three == '':
                break
            
            badge = ''

            for item_one in rucksack_one:
                if item_one in indexed_items:
                  continue
                
                for item_two in rucksack_two:
                    if item_one == item_two:
                        for item_three in rucksack_three:
                            if item_three == item_two:
                                badge = item_three
                                break
                        break

                if badge != '':
                    break

                indexed_items[item_one] = True
            priority += check_priority(badge)
    return priority
    

print(part_two())

