file = open("main-input.txt")
elfCarrying = [0]
index = 0

for line in file:
    if (line != '\n'): 
        elfCarrying[index] += int(line.strip())
    else: 
        elfCarrying.append(0)
        index+=1

elfCarrying.sort()
print(elfCarrying[-1] + elfCarrying[-2] + elfCarrying[-3])