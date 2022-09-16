import os
from typing import TypedDict, List, Union


class Registers(TypedDict):
    w: Union[int,str]
    x: Union[int,str]
    y: Union[int,str]
    z: Union[int,str]
    arg_ptr: int


class Instruction:
    pass


class Inp_Instruction(Instruction):
    def __init__(self, param1: str):
        self.param1 = param1

    def __call__(self, reg: Registers):
        reg[self.param1] = "arg" + str(reg["arg_ptr"])
        reg["arg_ptr"] += 1


class Add_Instruction(Instruction):
    def __init__(self, param1: str, param2: Union[str, int]):
        self.param1 = param1
        self.param2 = param2

    def __call__(self, reg: Registers):
        val1 = reg[self.param1]
        val2 = reg[self.param2] if type(self.param2) == str else self.param2
        if val1 == 0:
            reg[self.param1] = val2
        elif val2 ==0:
            reg[self.param1] = val1
        elif type(val1) == str or type(val2) == str:
            reg[self.param1] = f"({val1})+({val2})"
        else:
            reg[self.param1] = val1 + val2


class Mul_Instruction(Instruction):
    def __init__(self, param1: str, param2:  Union[str, int]):
        self.param1 = param1
        self.param2 = param2

    def __call__(self, reg: Registers):
        val1 = reg[self.param1]
        val2 = reg[self.param2] if type(self.param2) == str else self.param2
        if val1 == 1:
            reg[self.param1] = val2
        elif val2 ==1:
            reg[self.param1] = val1
        elif type(val1) == str or type(val2) == str:
            reg[self.param1] = f"({val1})*({val2})"
        else:
            reg[self.param1] = val1 * val2


class Div_Instruction(Instruction):
    def __init__(self, param1: str, param2:  Union[str, int]):
        self.param1 = param1
        self.param2 = param2

    def __call__(self, reg: Registers):
        val1 = reg[self.param1]
        val2 = reg[self.param2] if type(self.param2) == str else self.param2
        if val2 == 1:
            reg[self.param1] = val1
        elif type(val1) == str or type(val2) == str:
            reg[self.param1] = f"({val1})/({val2})"
        else:
            reg[self.param1] = val1 * val2


class Mod_Instruction(Instruction):
    def __init__(self, param1: str, param2:  Union[str, int]):
        self.param1 = param1
        self.param2 = param2

    def __call__(self, reg: Registers):
        val = reg[self.param2] if type(self.param2) == str else self.param2
        reg[self.param1] = reg[self.param1] % val


class Eql_Instruction(Instruction):
    def __init__(self, param1: str, param2:  Union[str, int]):
        self.param1 = param1
        self.param2 = param2

    def __call__(self, reg: Registers):
        val = reg[self.param2] if type(self.param2) == str else self.param2
        if reg[self.param1] == val:
            reg[self.param1] = 1
        else:
            reg[self.param1] = 0


class Program:
    def __init__(self):
        self.instructions: List[Instruction] = list()
        self.registers = Registers(w=0, x=0, y=0, z=0, arg_ptr=0, args="")

    def add_instruction(self, new_instruction: Instruction) -> None:
        self.instructions.append(new_instruction)

    def reset(self):
        self.registers["arg_ptr"] = 0
        self.registers["w"] = 0
        self.registers["x"] = 0
        self.registers["y"] = 0
        self.registers["z"] = 0

    def __call__(self, arg: List[int]):
        self.registers["args"] = arg
        for inst in self.instructions:
            inst(self.registers)
        return self.registers


def main():
    with open(f"{os.path.dirname(__file__)}/input.txt", "r") as in_file:
        prog = Program()
        for line in in_file:
            line = line.strip()
            tokens = line.split(" ")
            if tokens[0] == "inp":
                prog.add_instruction(Inp_Instruction(tokens[1]))
                continue
            val = tokens[2] if tokens[2].isalpha() else int(tokens[2])
            if tokens[0] == "add":
                prog.add_instruction(Add_Instruction(tokens[1],val))
            elif tokens[0] == "mul":
                prog.add_instruction(Mul_Instruction(tokens[1],val))
            elif tokens[0] == "div":
                prog.add_instruction(Div_Instruction(tokens[1],val))
            elif tokens[0] == "mod":
                prog.add_instruction(Mod_Instruction(tokens[1],val))
            elif tokens[0] == "eql":
                prog.add_instruction(Eql_Instruction(tokens[1],val))
        test_data = [9,9,9,9,9,9,9,9,9,9,9,9,9,9]
        searching = True
        while searching:
            results = prog(test_data)
            if results['z'] == 0:
                searching = False
            else:
                prog.reset()
                index = -1
                while test_data[index] == 1:
                    test_data[index] = 9
                    index -= 1
                test_data[index] -= 1



if __name__ == "__main__":
    main()
