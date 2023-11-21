import re
import sys
import matplotlib.pyplot as plt 


path = sys.argv[1]
file = open(path)
lines = file.readlines()

def to_mil(in_str):
    number_part = re.match("[\d\.]+",in_str)[0]
    if re.search("µs",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])/1000
        
    elif re.search("ms",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])
    elif re.search("s",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])*1000
    return number_part
    
n = []
k_n = []
k_n_div_2 = []
k_n_div_4 = []
k_n_div_16 = []
k_n_div_32 = []
k_const_12 = []

lines = lines[2:]
for i in range(0,len(lines)):
    lines[i] = lines[i].split(", ")
    n.append(int(lines[i][0]))
    k_n.append(to_mil(lines[i][1]))
    k_n_div_2.append(to_mil(lines[i][2]))
    k_n_div_4.append(to_mil(lines[i][3]))
    k_n_div_16.append(to_mil(lines[i][4]))
    k_n_div_32.append(to_mil(lines[i][5]))
    k_const_12.append(to_mil(lines[i][6]))
plt.plot(n,k_n,label="k=n")
plt.plot(n,k_n_div_2,label="k=n/2")
plt.plot(n,k_n_div_4,label="k=n/4")
plt.plot(n,k_n_div_16,label="k=n/16")
plt.plot(n,k_n_div_32,label="k=n/32")
plt.plot(n,k_const_12,label="k=12")
plt.legend(["k=n","k=n/2","k=n/4","k=n/16","k=n/32","k=12"])
plt.xlabel("n")
plt.ylabel("Time ms")
plt.savefig(path+".png")

