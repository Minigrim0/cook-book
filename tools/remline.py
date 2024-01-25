import sys

if len(sys.argv) < 2:
    print("Usage: python3 {} <word to remove>".format(sys.argv[0]))
    sys.exit(1)

a = 0
with open("out.txt", "r") as f:
    new_lines = []
    for line in f.readlines():
        if sys.argv[1] in line:
            a += 1
        else:
            new_lines.append(line.strip())

with open("out.txt", "w") as f:
    f.write("\n".join(new_lines))

print(f"Deleted {a} entries")
