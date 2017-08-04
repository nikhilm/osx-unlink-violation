import sys
for i in range(int(sys.argv[1])):
    with open('%d.sample'%i, 'w') as file:
        file.write("hello world\n")
