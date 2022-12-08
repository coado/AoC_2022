
def part_one():
    file = open("data.txt", "r")
    content = file.read()

    iterations = 0
    codes = []
    for code in content:
        iterations += 1
        if len(codes) < 3:
            codes.append(code)
            continue
        
        codes.append(code)

        if len(set(codes)) == 4:
            return iterations
        else:
            codes.pop(0)

        


# print(part_one())


def part_two():
    file = open("data.txt", "r")
    content = file.read()

    iterations = 0
    codes = []
    for code in content:
        iterations += 1
        if len(codes) < 13:
            codes.append(code)
            continue
        
        codes.append(code)

        if len(set(codes)) == 14:
            return iterations
        else:
            codes.pop(0)

    
print(part_two())