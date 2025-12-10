import math

def parseFile(filename:str) -> list:
   file = open(filename, "r")
   contents = file.readlines()
   contents = [ i.strip() for i in contents ]
   return contents


def replaceValid(y:int, x:int, map:list) -> int:
   count = 0

   if(map[y][x] == '.'):
        return 5 
   count += checkDown(y, x, map)
   count += checkUp(y, x, map) 
   count += checkLeftRight(y, x, map)
 

   return count       
 
def checkDown(y:int, x:int, map:list)-> int:

    count = 0;
    leftPossible = (x - 1) >= 0
    rightPossible = (x + 1) < len(map[0]) 

    if (y + 1) >= len(map):
      return 0

    else:
      if(map[y+1][x] == '@'):
         count += 1 
        
      if(rightPossible and map[y+1][x+1] == '@'):
         count +=1

      if(leftPossible and map[y+1][x-1] == '@'):
         count +=1

    return count 

def checkUp(y:int, x:int, map:list)-> int:
    count = 0;
    leftPossible = (x - 1) >= 0
    rightPossible = (x + 1) < len(map[0]) 

    if (y - 1) < 0:
      return 0

    else:
      if(map[y-1][x] == '@'):
         count += 1 
      if(rightPossible and map[y-1][x+1] == '@'):
         count +=1

      if(leftPossible and map[y-1][x-1] == '@'):
         count +=1

    return count

def checkLeftRight(y:int, x:int, map:list)-> int:
    count = 0;
 
    if( x+1 < len(map[0]) and map[y][x+1] == '@'):
         count +=1
    if( x-1 >= 0 and map[y][x-1] == '@'):
         count +=1
    return count

def main():
  numValid = 0;
  map = parseFile("input.txt") 
  for y in range(len(map)):
    for x in range(len(map[0])):
      count = replaceValid(y, x, map)
      if count < 4 :
         numValid += 1

  print(numValid)
  
       
        
main()

