
def process(instr:str, rot:int, knob:int) -> int:
    print(instr)
    if (instr == 'L'):
        knob = (knob - rot) % 100
    else:
        knob = (knob + rot) % 100
    return knob

def main():
  knob = 50
  count = 0
  file = open("input.txt", 'r')
  contents = file.readlines() 
  for x in contents:
    knob = process(x[0], int(x[1:]), knob)
    if knob ==0:
            count += 1 
    print(count)  

main()
