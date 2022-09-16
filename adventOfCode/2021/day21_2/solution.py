import os
from collections import defaultdict
from typing import DefaultDict, Tuple

def mod10(a):
    out = a%10
    return out+10 if out==0 else out


def main():

    game_states:DefaultDict[Tuple[int,int,int,int],int] = defaultdict(int)
    game_states[(8,0,5,0)] += 1
    on_p1 = True
    p1_wins = 0
    p2_wins = 0
    while len(game_states) != 0:
        new_game_states:DefaultDict[Tuple[int,int,int,int],int] = defaultdict(int)
        for state in game_states.keys():
            p1_loc,p1_score,p2_loc,p2_score = state
            num_states = game_states[state]
            if on_p1:
                for roll, universes in [(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)]:
                    next_step = mod10(p1_loc + roll)
                    score = p1_score + next_step
                    if score >= 21:
                        p1_wins += num_states * universes
                    else:
                        new_game_states[(next_step,score,p2_loc,p2_score)] += num_states * universes
            else:
                for roll, universes in [(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)]:
                    next_step = mod10(p2_loc + roll)
                    score = p2_score + next_step
                    if score >= 21:
                        p2_wins += num_states * universes
                    else:
                        new_game_states[(p1_loc,p1_score,next_step,score)] += num_states * universes
        on_p1 = not on_p1
        game_states = new_game_states
    print(f"solution {p1_wins if p1_wins>p2_wins else p2_wins}")

if __name__ == "__main__":
    main()