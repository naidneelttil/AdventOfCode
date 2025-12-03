import math
def parseFile(filename:str) -> list:
   file = open(filename, "r")
   contents = file.readlines()
   allLines = ""
   for x in contents:
        allLines += x
   return allLines.split(',')

def isInvalid(num:int) -> bool:
    number = str(num)
    if len(number)%2 != 0:
        return False

    middle = int(len(number) / 2)  
    return (number[0:middle] == number[middle:])

   
def main():
   ranges = parseFile("input.txt") 
   invalid = 0;
   for x in ranges :
        span = x.split('-')  
        if(len(span) == 2):
          for i in range(int(span[0]), int(span[1]) + 1):
             if ( isInvalid(i) ):
                invalid += i

   print(invalid)

main()

