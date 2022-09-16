import os

def main():
    with open(f"{os.path.dirname(__file__)}/input.txt", "r") as in_file:
        count = 0
        for line in in_file:
            line = line.strip()
            tokens = line.split(" ")
            
            out = f"inst({count},"
            if tokens[0] == "inp":
                out = f"1{{{out}input,{tokens[1]},1..9)}}1."
                print(out)
            else:
                tag = "R" if tokens[2].isalpha() else "I"
                out = f"{out}{tokens[0]}{tag},{tokens[1]},{tokens[2]})."
                print(out)
            count+=1

if __name__ == "__main__":
    main()