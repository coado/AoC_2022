


def part_one():
    visited_nodes = {}
    
    # initial position
    # [row, col]
    position_T = [0, 0]
    position_H = [0, 0]
    visited_nodes["0,0"] = 1
    
    with open('data.txt') as f:
        for line in f:
            [direction, number_of_steps] = line.split(' ')
            number_of_steps = int(number_of_steps)

            for _ in range(number_of_steps):
                match direction:
                    case "R":
                        position_H[1] += 1
                    case "L":
                        position_H[1] -= 1
                    case "U":
                        position_H[0] += 1
                    case "D":
                        position_H[0] -= 1

                distance = [position_H[0] - position_T[0], position_H[1] - position_T[1]]
                if abs(distance[0]) in (0, 1) and abs(distance[1]) in (0, 1):
                    continue

                if distance[0] == 0:
                    position_T[1] += int(distance[1] * 0.5)
                
                elif distance[1] == 0:
                    position_T[0] += int(distance[0] * 0.5)
                
                elif abs(distance[0]) == 2:
                    position_T[1] += distance[1]
                    position_T[0] += int(distance[0] * 0.5)

                elif abs(distance[1]) == 2:
                    position_T[0] += distance[0]
                    position_T[1] += int(distance[1] * 0.5)
                
                visited_nodes["{},{}".format(position_T[0], position_T[1])] = 1

    total_visited_nodes = 0
    for _ in visited_nodes:
        total_visited_nodes += 1
    print(total_visited_nodes) 


# part_one()


def update_position(position_1, position_2):
    distance = [position_1[0] - position_2[0], position_1[1] - position_2[1]]
    if abs(distance[0]) in (0, 1) and abs(distance[1]) in (0, 1):
        return False
    if distance[0] == 0:
        position_2[1] += int(distance[1] * 0.5)
    
    elif distance[1] == 0:
        position_2[0] += int(distance[0] * 0.5)
    
    elif abs(distance[0]) == 2:
        position_2[1] += distance[1]
        position_2[0] += int(distance[0] * 0.5)
        
    elif abs(distance[1]) == 2:
        position_2[0] += distance[0]
        position_2[1] += int(distance[1] * 0.5)

    return True



def part_two():
    visited_nodes = {}
    
    # [row, col]
    positions = {}

    # initial positions
    for i in range(10):
        positions[i] = [0,0]

    visited_nodes["0,0"] = 1
    
    with open('data.txt') as f:
        for line in f:
            [direction, number_of_steps] = line.split(' ')
            number_of_steps = int(number_of_steps)

            for _ in range(number_of_steps):
                match direction:
                    case "R":
                        positions[0][1] += 1
                    case "L":
                        positions[0][1] -= 1
                    case "U":
                        positions[0][0] += 1
                    case "D":
                        positions[0][0] -= 1

                for i in range (1, 9):
                    is_updated = update_position(positions[i - 1], positions[i])
                    if not is_updated:
                        break
                    if i == 8:
                        visited_nodes["{},{}".format(positions[8][0], positions[8][1])] = 1
                        


    total_visited_nodes = 0
    for _ in visited_nodes:
        total_visited_nodes += 1
    print(total_visited_nodes) 

part_two()