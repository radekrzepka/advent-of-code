file = open("main-input.txt")

winningPaterns = {
    "A": "Y",
    "B": "Z",
    "C": "X"
}

loosingPaterns = {
    "A": "Z",
    "B": "X",
    "C": "Y"
}

score1 = 0

for line in file:
    opponentChoice = line.strip()[0]
    playerChoice = line.strip()[2]
    score1 += ord(playerChoice) - 87
    if (winningPaterns[opponentChoice] == playerChoice): score1 += 6
    elif (ord(opponentChoice) == ord(playerChoice) - 23): score1 += 3


print(score1)

score2 = 0
file.seek(0)

for line in file:
    opponentChoice = line.strip()[0]
    result = line.strip()[2]

    match result:
        case "X":
            playerChoice = loosingPaterns[opponentChoice]
            score2 += ord(playerChoice) - 87
        case "Y":
            score2 +=  3 + ord(opponentChoice) - 64
        case "Z":
            playerChoice = winningPaterns[opponentChoice]
            score2 += 6 + ord(playerChoice) - 87

print(score2)