import math
def parseFile(filename:str) -> list:
   file = open(filename, "r")
   contents = file.readlines()
   allLines = ""
   for x in contents:
        allLines += x
   return allLines.split(',')


def isInvalid(num:str) -> bool:
     length = len(num)
     #find all divisible numbers 
     
     divisible = []
     for x in range(1, length):
        if (length % x == 0):
            divisible.append(x)
     
     divisible.sort(reverse=True)
    
     for x in divisible:
        splits = []
    
        for i in range(0, length, x):
            splits.append(num[i : i+x])
        splits = set(splits) 
        if(len(splits) == 1):
            return True

     return False

def main():
   ranges = parseFile("input.txt") 
   invalid = 0;
   for x in ranges :
        span = x.split('-')  
        if(len(span) == 2):
          for i in range(int(span[0]), int(span[1]) + 1):
             if ( isInvalid(str(i)) ):
                invalid += i

   print(invalid)

main()

