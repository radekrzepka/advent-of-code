file = open("main-input.txt")
text = ''

for line in file:
    text = line.strip()

def allUniqeChars(text):
    charArr = []
    for char in text:
        if char in charArr: return False
        charArr.append(char)
    return True

index1 = 0
index2 = 0

for i in range(3, len(text)):
    fourDigitText = ''

    for j in range(i - 3, i + 1): 
        fourDigitText += text[j]

    if allUniqeChars(fourDigitText): 
        index1 = i + 1
        break

for i in range(13, len(text)):
    fourteenDigitText = ''
    for j in range(i - 13, i + 1): 
        fourteenDigitText += text[j]

    if allUniqeChars(fourteenDigitText): 
        index2 = i + 1
        break

print(index1, index2)