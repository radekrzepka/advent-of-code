file = open("main-input.txt")

words = []

for line in file:
    words.append(line.strip())

def findSameLetterTwoWords(firstWord, secondWord):
    firstWordLetters = []
    for letter in firstWord:
        if(letter not in firstWordLetters): firstWordLetters.append(letter)

    for letter in secondWord:
        if(letter in firstWordLetters): 
            return letter

def findSameLetterThreeWords(firstWord, secondWord, thirdWord):
    firstWordLetters = []
    for letter in firstWord:
        if(letter not in firstWordLetters): firstWordLetters.append(letter)

    firstAndSecondWordSameLetters = []   
    for letter in secondWord:
        if(letter in firstWordLetters): firstAndSecondWordSameLetters.append(letter)

    for letter in thirdWord:
        if(letter in firstAndSecondWordSameLetters): return letter   

def letterValue(letter):
    if(ord(letter) >= 97): return ord(letter) - 96
    else: return ord(letter) - 38

result1 = 0

for word in words: 
    halfWorld = int(len(word)/2)

    firstWord = word[:halfWorld]
    secondWord = word[halfWorld:]

    sameLetter = findSameLetterTwoWords(firstWord, secondWord)
    result1 += letterValue(sameLetter)

print(result1)

result2 = 0

for i in range(0, len(words), 3):
    sameLetter = findSameLetterThreeWords(words[i], words[i+1], words[i+2])
    result2 += letterValue(sameLetter)

print(result2)