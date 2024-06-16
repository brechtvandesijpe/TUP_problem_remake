import sys
import pandas as pd
import re

def read_data(filename):
    with open(f"resources/{filename}", 'r') as file:
        content = file.read()

    # Attempt to extract nTeams value
    nTeams_match = re.search(r'nTeams\s*=\s*(\d+)\s*;', content)
    if nTeams_match:
        nTeams = int(nTeams_match.group(1))
    else:
        raise ValueError("nTeams not found in the file.")

    # Attempt to extract dist matrix
    dist_matches = re.search(r'dist\s*=\s*\[\s*([\s\S]+?)\s*\];', content, re.DOTALL)
    if dist_matches:
        dist_str = dist_matches.group(1)
        dist = [list(map(int, re.findall(r'\d+', row))) for row in dist_str.strip().split('\n')]
    else:
        raise ValueError("dist matrix not found in the file.")

    # Attempt to extract opponents matrix
    opponents_matches = re.search(r'opponents\s*=\s*\[\s*([\s\S]+?)\s*\];', content, re.DOTALL)
    if opponents_matches:
        opponents_str = opponents_matches.group(1)
        opponents = [list(map(int, re.findall(r'[-\d]+', row))) for row in opponents_str.strip().split('\n')]
    else:
        raise ValueError("opponents matrix not found in the file.")

    return nTeams, dist, opponents


file_name = sys.argv[1]
parts = file_name.split("_")

input_filename = f"{parts[0]}.txt"
q1 = int(parts[1])
q2 = int(parts[2].split(".")[0])

df = pd.read_csv(f"results/{file_name}")
df['Home_Out'] = list(zip(df['Home'], df['Out']))
pivot_df = df.pivot_table(index='Round', columns='Umpire', values='Home_Out', aggfunc=lambda x: x)
pivot_df.fillna('None', inplace=True)
matrix = pivot_df.to_numpy()

nTeams, dist, opponents = read_data(input_filename)

# PREVIOUS ASSIGNMENTS
for rnd in matrix:
    checklist = []
    for umpire in rnd:
        if umpire[0] in checklist:
            print(f"({umpire[0]}, _) was assigned twice in the same round!")
            sys.exit(1)
        checklist.append(umpire[0])
        if umpire[1] in checklist:
            print(f"(_, {umpire[1]}) was assigned twice in the same round!")
            sys.exit(1)
        checklist.append(umpire[1])

# GLOBAL CONSTRAINT
for umpire_index in range(len(matrix[0])):
    checklist = [i + 1 for i in range(nTeams)]
    for rnd in matrix:
        assignment = rnd[umpire_index]
        if assignment[0] in checklist:
            checklist.remove(assignment[0])
    
    if len(checklist) != 0:
        print(f"Umpire {umpire_index + 1} global constraint not ok!")
        sys.exit(1)
        
# Q1 CONSTRAINT
for umpire_index in range(len(matrix[0])):
    for start_round in range(len(matrix)):
        checklist = []
        try:
            for i in range(q1):
                assignment = matrix[start_round + i][umpire_index]
                if assignment[0] not in checklist:
                    checklist.append(assignment[0])
                else:
                    print(f"Umpire {umpire_index + 1}, start_round {start_round} Q1 constraint not ok!")
                    sys.exit(1)
        except IndexError:
            pass

# Q2 CONSTRAINT
for umpire_index in range(len(matrix[0])):
    for start_round in range(len(matrix)):
        checklist = []
        try:
            for i in range(q2):
                assignment = matrix[start_round + i][umpire_index]
                if assignment[0] not in checklist:
                    checklist.append(assignment[0])
                else:
                    print(f"Umpire {umpire_index + 1}, start_round {start_round}, home location Q2 constraint not ok!")
                    sys.exit(1)
                    
                if assignment[1] not in checklist:
                    checklist.append(assignment[1])
                else:
                    print(f"Umpire {umpire_index + 1}, start_round {start_round}, out location Q2 constraint not ok!")
                    sys.exit(1)
        except IndexError:
            pass
    
total_distance = 0
for umpire_index in range(len(matrix[0])):
    previous_location = matrix[0][umpire_index][0] - 1
    for index in range(1, len(matrix), 1):
        current_location = matrix[index][umpire_index][0] - 1
        total_distance += dist[previous_location][current_location]
        previous_location = current_location
        
print(f"Solution is feasible, total distance = {total_distance}")