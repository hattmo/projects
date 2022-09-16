import os

def mod10(a):
    out = a%10
    return out+10 if out==0 else out

class Deter_Dice:
    def __init__(self):
        self.roll_count = 0
        self.val = 1
    def roll_3(self):
        self.roll_count += 3
        distance = 0
        for _ in range(3):
            if self.val > 100:
                self.val - 100
            distance += self.val
            self.val += 1
        return distance

def main():
    dice = Deter_Dice()
    p1_loc = 8
    p1_score = 0
    p2_loc = 5
    p2_score = 0
    on_p1 = True
    while True:
        if on_p1:
            p1_loc = mod10(p1_loc + dice.roll_3())
            p1_score += p1_loc
        else:
            p2_loc = mod10(p2_loc + dice.roll_3())
            p2_score += p2_loc
        on_p1 = not on_p1
        if p1_score >= 1000 or p2_score >= 1000:
            print(f"solution: {dice.roll_count * (p1_score if p1_score < p2_score else p2_score)}")
            exit()


if __name__ == "__main__":
    dice = Deter_Dice()
    main()