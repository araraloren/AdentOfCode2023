spaces =[y.split('\n')  for y in  open('input.txt').read().split('\n\n')]
transp = [[''.join([l[i] for l in s  ]) for i in range(len(s[0]))] for s in spaces]
total = 0

def get_col(space):
    size = len(space[0])
    for i in range(0,size - 1):
        d = min(size - i - 2 , i)
        r1 = (i - d, i + 1)
        r2 = (i + 1, i + 1 + d + 1)
        good = True
        print("checking ", r1, " cmpare to ", r2)
        for j in range(len(space)):
            if space[j][r1[0]:r1[1]] != space[j][r2[0]:r2[1]][::-1]:
                good = False
        if good:
            return(i+1)        
    return(False)
   
for i in range(len(spaces)):
    ans = get_col(spaces[i])
    if not ans:
        print("........")
        total += 100 * get_col(transp[i])
    else:
        total += ans

print(total)