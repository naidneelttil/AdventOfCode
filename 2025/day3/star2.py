import math
from typing import Tuple
def parseFile(filename:str) -> list:
   file = open(filename, "r")
   contents = file.readlines()
   return contents


def findlargest(battery:list, start:int, stop) -> int :

 largest = start;
 #must change 
 for i in range(start, stop):
    if battery[largest] < battery[i]:
       largest = i
 return largest
   
def findLargestVoltage(battery:list):

    battery = list(map(int, battery)) 
    #find largest
    
    positions= [] 
    pos = 0
    
    for i in range(11, -1, -1):
      pos = findlargest(battery, pos, len(battery) - i)
      positions.append(pos)
      pos += 1
    
    return positions 

def main():
   batteries = parseFile("input.txt") 
   voltageTotal = 0;
   #batteries = batteries[0:-2]
   for battery in batteries:
     positons = findLargestVoltage(list(battery.strip()))       
     print("bat:")
     print(battery)
     print("pos:")
     #print(positons)
     
     listnum = [battery[i] for i in positons]
     listnum = int("".join(listnum))
     print(listnum)
     voltageTotal += listnum 
   print(voltageTotal)

main()

