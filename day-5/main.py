file = open("main-input.txt")

lines = []
instructions = []
stacks = []

for line in file:
    lines.append(line)

for i in range(len(lines) - 1, -1, -1):
    if(lines[i][0] == "m"): instructions.append(lines[i].strip())
    if("1" in lines[i] and lines[i][0] != "m"): 
        for char in lines[i]:
            if char.isnumeric(): stacks.append([])
    if("[" in lines[i]):
        for j in range(len(lines[i])):
            if (j % 4 == 1 and lines[i][j] != " "): stacks[int(j/4)].append(lines[i][j])

instructions.reverse()

def checkIfNumeric(char):
    if char.isnumeric(): return True
    return False

def changeToInt(char):
    return int(char)

#first part
# for instructuion in instructions:
#     numbers = filter(checkIfNumeric, instructuion.split(" "))
#     intNumbers = map(changeToInt, numbers)

#     (howMuchToMove, fromWhereMove, toWhereMove) = list(intNumbers)

#     for i in range (howMuchToMove):
#         record = stacks[fromWhereMove - 1].pop()
#         stacks[toWhereMove - 1].append(record)

#secondPart
for instruction in instructions:
    numbers = filter(checkIfNumeric, instruction.split(" "))
    intNumbers = map(changeToInt, numbers)

    (howMuchToMove, fromWhereMove, toWhereMove) = list(intNumbers)

    stack = stacks[fromWhereMove - 1][-howMuchToMove:]
    del stacks[fromWhereMove - 1][-howMuchToMove:]
    stacks[toWhereMove - 1] += stack

lastElements = ''

for stack in stacks:
    lastElements += stack[-1]

print(lastElements)