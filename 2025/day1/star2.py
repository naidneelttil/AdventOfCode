
def process(instr:str, rot:int, knob:int) -> int:
   
    if (instr == 'L'): 
         knob = (knob - rot) % 100       
    else:
       knob = (knob + rot) % 100
    return knob

def numOfClicks(instr:str, rot:int, knob:int) -> int:
    clicked = 0 
    fullRotations = 0
    lastRot = rot % 100
    if (instr == 'L'): 
        fullRotations = int( rot / 100 )
        if (knob - lastRot) < 0 and knob != 0:
           clicked += 1
           
    else:
      fullRotations = int( rot / 100) 
      total = knob + lastRot
      print(f"this is full rot {fullRotations} and {total}")
      if (knob + lastRot) > 100:
         clicked += 1
   
    clicked += fullRotations
    return  clicked


def main():
  knob = 50
  count = 0
  clicked = False
  file = open("input.txt", 'r')
  contents = file.readlines() 
  for x in contents:
    rot = int(x[1:])
    instr = x[0]
    clicked = numOfClicks(instr, rot, knob)
    knob = process(instr, rot, knob)
    if knob == 0 :
            clicked +=1
    count += clicked
  print(count)  

def test():
  knob = 50
  count = 0

  file = open("test.txt", 'r')
  contents = file.readlines() 
  for x in contents:
    rot = int(x[1:])
    instr = x[0]
    clicked = numOfClicks(instr, rot, knob)
    knob = process(instr, rot, knob)
    if knob == 0 :
          clicked +=1
    print(f"{instr}{rot} the changed knob {knob} and clicks {clicked}")

    count += clicked
  print(count)
  assert(count == 6) 



    

main()
test()
