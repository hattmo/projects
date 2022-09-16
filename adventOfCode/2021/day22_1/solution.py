import os

def main():
    with open(f"{os.path.dirname(__file__)}/input.txt","r") as in_file:
        status = dict()
        for line in in_file:
            line = line.strip()
            [on_off,rest] = line.split(" ")
            entries = rest.split(",")
            box = dict()
            bad = False
            for entry in entries:
                [axis,coords] = entry.split("=")
                [low,high] = [int(x) for x in coords.split("..")]
                if (low > 50) or (low < -50) or (high > 50) or (high < -50):
                    bad = True
                box[axis] = (low,high)
            if bad:
                continue
            for x in range(box["x"][0],box["x"][1]+1):
                for y in range(box["y"][0],box["y"][1]+1):
                    for z in range(box["z"][0],box["z"][1]+1):
                        status[(x,y,z)] = True if on_off == "on" else False
        solution = len([x for x in status.values() if x == True])
        print(f"solution: {solution}")



if __name__ == "__main__":
    main()