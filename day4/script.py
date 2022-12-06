

def part_one():
    fully_overlapping_total = 0;

    with open('data.txt') as f:
        for line in f: 
            [range_1, range_2] = line.split(',')
            [range_1_left, range_1_right] = range_1.split('-')
            [range_2_left, range_2_right] = range_2.split('-')
            range_1_left = int(range_1_left)
            range_1_right = int(range_1_right)
            range_2_left = int(range_2_left)
            range_2_right = int(range_2_right)
            
            if range_1_left >= range_2_left and range_1_right <= range_2_right:
                fully_overlapping_total += 1

            elif range_2_left >= range_1_left and range_2_right <= range_1_right:
                fully_overlapping_total += 1

    return fully_overlapping_total


# print(part_one())


def part_two():
    overlapping_total = 0;

    with open('data.txt') as f:
        for line in f: 
            [range_1, range_2] = line.split(',')
            [range_1_left, range_1_right] = range_1.split('-')
            [range_2_left, range_2_right] = range_2.split('-')
            range_1_left = int(range_1_left)
            range_1_right = int(range_1_right)
            range_2_left = int(range_2_left)
            range_2_right = int(range_2_right)

            # adding 1 in consequentive circumnstances:
            # range 1 overlap with range 2
            # range 2 consists of range 1
            if range_1_right >= range_2_left and range_1_right <= range_2_right:
                overlapping_total += 1

            # adding 1 in consequentive circumnstances:
            # range 2 overlap with range 1
            # range 1 consists of range 2
            elif range_2_right >= range_1_left and range_2_right <= range_1_right:
                overlapping_total += 1

    return overlapping_total


print(part_two())