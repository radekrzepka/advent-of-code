file = open("main-input.txt")
instructions = []

for line in file:
    instructions.append(line.strip())

class Directory:
    def __init__(self, name, parent = None):
        self.name = name
        self.files = []
        if parent is None:
            self.parent = ""
        else:
            self.parent = parent
    
    def addFile(self, file):
        self.files.append(file)

    def countDirectorySize(self):
        size = 0
        for file in self.files:
            size += file.size
        return size


class File:
    def __init__ (self, name, size):
        self.name = name
        self.size = size

allDirectories = []
allDirectories.append(Directory("/"))

currentDirectories = []

for i in range(len(instructions)):
    if (instructions[i][:4] == "$ cd"):
        if (".." in instructions[i]):
            currentDirectories.pop()
        else:
            currentDirectories.append(instructions[i].split()[2])   
    elif (instructions[i][:4] == "$ ls"):
        lsResult = []

        for j in range(i+1, len(instructions)):
            if (instructions[j][0] == "$"): break
            lsResult.append(instructions[j])
        
        for result in lsResult:
            if (result.split()[0] == "dir"):
                newDirectoryName = result.split()[1]
                if not any(directory.name == newDirectoryName and directory.parent == currentDirectories[-1] for directory in allDirectories):
                    allDirectories.append(Directory(newDirectoryName, currentDirectories[-1]))
            else:
                (fileSize, fileName) = result.split()
                fileSize = int(fileSize)
                for index, currentDirectory in enumerate(currentDirectories):
                    for directory in allDirectories:
                        if directory.name == currentDirectory and (directory.parent == currentDirectories[index - 1] or directory.parent == ""):
                            directory.addFile(File(fileName, fileSize))
                            break
    
directoriesSizes = [directory.countDirectorySize() for directory in allDirectories]

sum = 0
for size in directoriesSizes:
    if size <= 100000: sum += size
print(sum)

allFilesSize = [directory.countDirectorySize() for directory in allDirectories if directory.name == "/"][0]
howMuchSpaceToClean = 30000000 - (70000000 - allFilesSize)
smmalestDir = min([directory.countDirectorySize() for directory in allDirectories if directory.countDirectorySize() >= howMuchSpaceToClean])
print(smmalestDir)