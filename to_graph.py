import re

file = open("run_1_date_20-11/data.wide_k.csv")
lines = file.readlines()

def to_mil(in_str):
    number_part = re.match("[\d\.]+",in_str)[0]
    if re.search("µs",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])/1000
        
    elif re.search("ms",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])
    elif re.search("s",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])*1000
    return str(number_part)
    

for i in range(1,len(lines)):
    lines[i] = lines[i].split(", ")
    lines[i][1] = to_mil(lines[i][1])
    lines[i][2] = to_mil(lines[i][2])
    lines[i][3] = to_mil(lines[i][3])
    lines[i][4] = to_mil(lines[i][4])
    lines[i][5] = to_mil(lines[i][5])
    lines[i][6] = to_mil(lines[i][6])

    lines[i] = ("; ".join(lines[i])).replace(".",",")
lines[0] = lines[0].replace(",",";")
print("\n".join(lines))

