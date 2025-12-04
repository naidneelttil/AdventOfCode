import math
from typing import Tuple
def parseFile(filename:str) -> list:
   file = open(filename, "r")
   contents = file.readlines()
   return contents


def findLargestVoltage(battery:list):

    battery = list(map(int, battery)) 
    #find largest
    print(battery)
    largest = 0;
    secLargest = 0; #must change 
    for i in range(len(battery)):
        if battery[largest] < battery[i]:
            largest = i
   
    
    if largest != len(battery) - 1:
     secLargest = largest + 1  
     for i in range(largest, len(battery)):
        if battery[secLargest] < battery[i] and i != largest:
            secLargest = i

    else:
       print("swap") 
       for i in range(0, len(battery) - 1):
          if battery[secLargest] < battery[i] and i != largest:
            secLargest = i

       temp = largest
       largest = secLargest
       secLargest = temp

    return battery[largest], battery[ secLargest ]

def main():
   batteries = parseFile("input.txt") 
   voltageTotal = 0;
   #batteries = batteries[0:-2]
   for battery in batteries:
     largest, secLargest = findLargestVoltage(list(battery.strip()))       
     print(int(str(largest) + str(secLargest)))

     voltageTotal +=  int(str(largest) + str(secLargest))
   print(voltageTotal)

main()

