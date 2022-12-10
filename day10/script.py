


def part_one():

    signal_strength = 0
    register_value = 1
    current_cycle = 0
    cycles_multiplier = [20, 60, 100, 140, 180, 220]

    with open('data.txt') as f:
        for line in f:
            signal = line.split(' ')  

            
            if signal[0].replace("\n", "") == "noop":
                current_cycle += 1  
                if current_cycle in cycles_multiplier:
                    signal_strength += register_value * current_cycle
                continue
                
            current_cycle += 1

            if current_cycle in cycles_multiplier:
                signal_strength += register_value * current_cycle  


            current_cycle += 1
            value = int(signal[1])

            if current_cycle in cycles_multiplier:
                signal_strength += register_value * current_cycle
            
            register_value += value
    
    return signal_strength

# print(part_one())  


def draw(draw_position, sprite_position, crt_screen):
    if draw_position >= sprite_position - 1 and draw_position <= sprite_position + 1:
        crt_screen[len(crt_screen) - 1] += '#'
    else:
        crt_screen[len(crt_screen) - 1] += '.'

def part_two():

    sprite_position = 1
    sprite_width = 3
    crt_screen = ['']
    crt_width = 40
    cycle_count = -1

    with open('data.txt') as f:
        for line in f:
            cycle_count += 1

            if cycle_count != 0 and cycle_count % 40 == 0:
                crt_screen.append('')
                cycle_count = 0

            signal = line.split(' ')    
            
            draw(cycle_count, sprite_position, crt_screen)

            if signal[0].replace("\n", "") == "noop":        
                continue

            cycle_count += 1

            if cycle_count % 40 == 0:
                crt_screen.append('')
                cycle_count = 0

            draw(cycle_count, sprite_position, crt_screen)

            value = int(signal[1])
            sprite_position += value
    
    
    for row in crt_screen:
        print(row)


part_two()