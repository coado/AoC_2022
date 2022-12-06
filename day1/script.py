


def part_one():
    max_calories = 0
    current_calories_value = 0

    with open('data.txt') as f:
        for line in f:
            if line == '\n':
                max_calories = max(max_calories, current_calories_value)
                current_calories_value = 0
            else:
                current_calories_value += int(line)
        
    print(max_calories)


def part_two():
    max_calories_bt = [float('-inf'), float('-inf'), float('-inf')]

    current_calories_value = 0
    with open('data.txt') as f:
        for line in f:
            if line == '\n':
                # inserting value into binary tree
                if current_calories_value > max_calories_bt[0]:
                    if current_calories_value > max_calories_bt[2]:
                        max_calories_bt[1] = max_calories_bt[0]
                        max_calories_bt[0] = max_calories_bt[2]
                        max_calories_bt[2] = current_calories_value
                    else:
                        max_calories_bt[1] = max_calories_bt[0]
                        max_calories_bt[0] = current_calories_value
                else:
                    if current_calories_value > max_calories_bt[1]:
                        max_calories_bt[1] = current_calories_value
        
                current_calories_value = 0
            else:
                current_calories_value += int(line)
    
    return sum(max_calories_bt)


print(part_two())