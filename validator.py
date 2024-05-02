import sys
global input_file, Q1, Q2
import ast

checkmark = "\u2713"
crossout = "\u2717"

def constraint_ok(title):
    print(f'''          {checkmark} {title} constraint''')
    
def constraint_nok(line, title):
    print(f'''          {crossout} {line} {title}''')

def check_q1(data):
    num_teams = len(data[0])
    banlist = []
    result = True
    
    for i in range(num_teams):
        banlist.append([])
    
    for rnd_index, rnd in enumerate(data):
        for umpire, entry in enumerate(rnd):
            if rnd_index > Q1 - 1:
                del banlist[umpire][0]
                
            if entry in banlist[umpire]:
                result = False
                constraint_nok(f"Umpireteam {umpire} failed round {rnd_index} → {rnd}", f"of Q1 constraint")
                
            banlist[umpire].append(entry)
    
    return result
    
def check_q2(data):
    num_teams = len(data[0])
    banlist = []
    result = True
    
    for i in range(num_teams):
        banlist.append([])
    
    for rnd_index, rnd in enumerate(data):
        for umpire, entry in enumerate(rnd):
            if rnd_index > Q2 - 1:
                del banlist[umpire][0]
                
            if entry in banlist[umpire]:
                result = False
                constraint_nok(f"Umpireteam {umpire} failed round {rnd_index} → {rnd}", f"of Q2 constraint")
                
            banlist[umpire].append(entry)
    
    return result
    
def check_global(data):    
    visited = []
    result = True
    
    for i in range(len(data[0])):
        visited.append([])
        for j in range(len(data[0]) * 2):
            visited[i].append(False)
    
    for rnd in data:
        for i in range(len(rnd)):
            visited[i][rnd[i][0] - 1] = True
    
    for i, visit_check in enumerate(visited):
        if False in visit_check:
            result = False
            constraint_nok(f"Umpireteam {i} failed", "of global constraint")
    
    return result

if len(sys.argv) == 4:
    input_file = sys.argv[1]
    Q1 = int(sys.argv[2])
    Q2 = int(sys.argv[3])
    
    title = f"Processing {input_file} with Q1 = {Q1}, Q2 = {Q2}"
    width = len(title)
    spacer = "-" * width
    print(f'''          {spacer}
          {title}
          {spacer}''')
    
    with open(input_file, 'r') as f:
        lines = f.readlines()

    data = [ast.literal_eval(line) for line in lines]
    for i, line in enumerate(data):
        width = max(len(str(line)), width)
        print(f"          {line} {i + 1}")
    
    print(f'''          {spacer}''')
   
    # Q1 CONSTRAINT
    result = check_q1(data)
    if result:
        constraint_ok("Q1")
    print(f'''          {spacer}''')
    
    # Q2 CONSTRAINT
    result = check_q2(data)
    if result:
        constraint_ok("Q2")
    print(f'''          {spacer}''')
    
    # GLOBAL CONSTRAINT
    result = check_global(data)
    if result:
        constraint_ok("Global")
    print(f'''          {spacer}''')
    
else:
    print('''
        Provide the needed arguments:
        python3 validator.py <input_file>.txt <q1> <q2>
          ''')