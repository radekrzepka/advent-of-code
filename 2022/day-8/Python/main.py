file = open("main-input.txt")
numbers = []

for line in file:
    line = line.strip()
    row = []
    for char in line:
        row.append(int(char))
    numbers.append(row)

numbersColumnLength = len(numbers)
numbersRowLength = len(numbers[0])    
howManyVisible = (2 * numbersColumnLength) + (2 * numbersRowLength) - 4

allScenicScores = []

def isVisible (number, allDirectionTrees):
    for trees in allDirectionTrees:  
        if all(treeSize < number for treeSize in trees): return True
    return False

def countTreeScenicScore (number, allDirectionTrees):
    scenicScore = 1

    for trees in allDirectionTrees:
        sideScore = 0
        for tree in trees:
            sideScore += 1
            if number <= tree: 
                break
        scenicScore *= sideScore  

    return scenicScore

for i in range (1, len(numbers) - 1):
    for j in range (1, len(numbers) - 1):
        testedElement = numbers[i][j]
        
        leftTrees = numbers[i][:j]
        rightTrees = numbers[i][j+1:]
        upTress = [numbers[k][j] for k in range(len(numbers)) if k < i]
        downTress = [numbers[k][j] for k in range(len(numbers)) if k > i]
        allDirectionTrees = [leftTrees, rightTrees, upTress, downTress]

        if isVisible(testedElement, allDirectionTrees):
            allDirectionTrees[0].reverse()
            allDirectionTrees[2].reverse()
            
            scenicScore = countTreeScenicScore(testedElement, allDirectionTrees)
            allScenicScores.append(scenicScore)
            
            howManyVisible += 1

print(howManyVisible, max(allScenicScores))