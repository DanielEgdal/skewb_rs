n = 263117474003
# n= 43744

def corners(n):

    binn = bin(n)[2:]
    while len(binn)%5 != 0:
        binn = '0'+binn

    print(binn,len(binn))
    for i in range(8):

        print((binn[(i*5):(i*5)+5]),int(binn[(i*5):(i*5)+2],2),int(binn[(i*5)+2:(i*5)+5],2))

def edges(n):

    binn = bin(n)[2:]
    while len(binn)%3 != 0:
        binn = '0'+binn

    print(binn,len(binn))
    for i in range(6):

        print((binn[(i*3):(i*3)+3]),int(binn[(i*3):(i*3)+3],2))

corners(n)

# edges(n)