import random
import string
from typing import Dict

escape_maps = {
    "a":"\a",
    "b":"\b",
    "f":"\f",
    "n":"\n",
    "r":"\r",
    "t":"\t",
    "v":"\v",
    "\\":"\\",
    "'":"'",
    "\"":"\"",
    "?":"?"
}
def extract_string_literal(raw_file: str,string_literals:dict[str,str]):
    in_literal = False
    in_escape = False
    string_literals = {}
    converted = ""
    label = ""
    for raw_c in raw_file:
        if in_literal:
            if in_escape:
                string_literals[label] += escape_maps[raw_c]
                in_escape = False
            elif raw_c == "\\":
                in_escape == True
            elif raw_c == "\"":
                in_literal = False
            else:
                string_literals[label] += raw_c
        else:
            if raw_c == "\"":
                label = ''.join(random.choices(string.ascii_uppercase, k=20))
                string_literals[label] = ""
                converted += label
                in_literal = True
            else:
                converted += raw_c


def preprocess(filename:str, context:Dict[str,str], depth:int):
    new_lines:list[str] = []
    with open(filename,"r") as file:
        raw_file = file.read()
        extract_string_literal(raw_file,  )
        for line in lines:
            if line.startswith("#include"):
                include_file = line.split(" ")[1]
                new_lines += preprocess(include_file,context, depth-1)
            else:
                new_lines += [line]
    return new_lines


if __name__ == '__main__':
    pass