

class FileSytem:
    # current_directory = {}
    # previous_directory = {}
    file_system = {}
    current_directory = ''

    def part_one(self):
        current_file_size = 0
        ls_active = False

        with open('data.txt') as f:
            for line in f:
                command = line.split()

                if command[0] == '$':
                    if ls_active == True:
                        current_directory = self.current_directory
                        if current_directory == '':
                            current_directory = '/'
                        if current_directory in self.file_system:
                            self.file_system[current_directory] += current_file_size
                        else:
                            self.file_system[current_directory] = current_file_size
                        current_file_size = 0
                        ls_active = False

                    if (command[1] == 'cd'):
                        self.change_directory(command[2])
                    if command[1] == 'ls':
                        ls_active = True
                else:
                    if command[0] == 'dir':
                        continue
                    else:
                        current_file_size += int(command[0])
        
        sizes = []
        total_space = 70000000

        for key_1, value_1 in self.file_system.items():
            dir_size = 0
            for key_2, value_2 in self.file_system.items():
                if key_1 in key_2:
                    dir_size += value_2
            sizes.append(dir_size)
            # print(key_1, dir_size)
        
        sizes.sort()
        enough_space = 30000000 - (total_space - sizes[-1])
        print(enough_space, total_space - sizes[-1])
        print(sizes)
        candidate = float('inf')
        for size in sizes:
            # print(size)
            if size < candidate and size >= enough_space:
                candidate = size
        print(candidate)


    def change_directory(self, new_dir):
        if new_dir == '/':
            self.current_directory = ''
        
        elif new_dir == '..':        
            directory_list = self.current_directory.split('/')
            directory_list_filtered = list(filter(lambda el: el != '', directory_list))
            self.current_directory =  '/' + '/'.join(directory_list_filtered[0:-1])

            if (self.current_directory == '/'):
                self.current_directory = ''
        
        else:
            self.current_directory += '/' + new_dir

file_system = FileSytem()
print(file_system.part_one())