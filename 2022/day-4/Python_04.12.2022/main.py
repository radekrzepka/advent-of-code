file = open("main-input.txt")

fullContainsPairs = 0
overlapPairs = 0

for line in file:
    firstRange, secondRange = line.strip().split(",")
    firstRangeStart, firstRangeEnd = firstRange.split("-")
    secondRangeStart, secondRangeEnd = secondRange.split("-")

    firstRangeStart = int(firstRangeStart)
    firstRangeEnd = int(firstRangeEnd)
    secondRangeStart = int(secondRangeStart)
    secondRangeEnd = int(secondRangeEnd)

    fullyContain = True

    if ((firstRangeEnd - firstRangeStart) > (secondRangeEnd - secondRangeStart)):
        for i in range (secondRangeStart, secondRangeEnd+1):
            if (i < firstRangeStart or i > firstRangeEnd): fullyContain = False
    else:
        for i in range (firstRangeStart, firstRangeEnd+1):
            if (i < secondRangeStart or i > secondRangeEnd): fullyContain = False

    if fullyContain: fullContainsPairs += 1

    overlapPair = False

    for i in range (firstRangeStart, firstRangeEnd+1):
        if (i >= secondRangeStart and i <= secondRangeEnd ): overlapPair = True

    if overlapPair: overlapPairs += 1

print(fullContainsPairs, overlapPairs)
    