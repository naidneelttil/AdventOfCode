


def getFirstNum(line):
    for i in range( 0, len(line), 1):
        if line[i].isdigit():
            return line[i]
        

def getLastNum(line):
    for i in range(len(line) -1, -1, -1) :
        if line[i].isdigit():
            return line[i]
        

def main():
 file = open("input.txt", "r")
 answer = 0

 lines = file.readlines()
 numbers = []
 for line in lines:
    fullNum = int(getFirstNum(line) + getLastNum(line))
    answer += fullNum
    print("this is the full line:"+ line)
    print("this is the full num:", fullNum)

 print( "the answer is: ", answer)
    
        


main()
