import os
from typing import List, Tuple

class Intersection_Result:
    def __init__(self,intersection:"Box"):
        self.inside_test:List["Box"] = list()
        self.inside_self:List["Box"] = list()
        self.intersection: Box = intersection


class Box:
    def __init__(self,low:Tuple[int,int,int],high:Tuple[int,int,int]):
        self.low = low
        self.high = high
    def size(self)->int:
        x = (self.high[0] - self.low[0])+1
        y = (self.high[1] - self.low[1])+1
        z = (self.high[2] - self.low[2])+1
        return x * y * z

    def intersects(self,test:"Box")->bool:
        if self.high[0] < test.low[0]:
            return False
        if self.low[0] > test.high[0]:
            return False
        if self.high[1] < test.low[1]:
            return False
        if self.low[1] > test.high[1]:
            return False
        if self.high[2] < test.low[2]:
            return False
        if self.low[2] > test.high[2]:
            return False
        return True

    def intersection(self,test:"Box")->Intersection_Result:
        
        shx,shy,shz = self.high
        slx,sly,slz = self.low
        thx,thy,thz = test.high
        tlx,tly,tlz = test.low
        rlz = tlz if tlz > slz else slz
        rhz = thz if thz < shz else shz
        rly = tly if tly > sly else sly
        rhy = thy if thy < shy else shy
        rlx = tlx if tlx > slx else slx
        rhx = thx if thx < shx else shx
        result = Intersection_Result(Box((rlx,rly,rlz),(rhx,rhy,rhz)))

#Corners

        if thx < shx and thy < shy and thz < shz:
            result.inside_self.append(Box((thx+1,thy+1,thz+1),(shx,shy,shz)))
        if thx > shx and thy > shy and thz > shz:
            result.inside_test.append(Box((shx+1,shy+1,shz+1),(thx,thy,thz)))

        if tlx > slx and thy < shy and thz < shz:
            result.inside_self.append(Box((slx,thy+1,thz+1),(tlx-1,shy,shz)))
        if tlx < slx and thy > shy and thz > shz:
            result.inside_test.append(Box((tlx,shy+1,shz+1),(slx-1,thy,thz)))

        if thx < shx and tly > sly and thz < shz:
            result.inside_self.append(Box((thx+1,sly,thz+1),(shx,tly-1,shz)))
        if thx > shx and tly < sly and thz > shz:
            result.inside_test.append(Box((shx+1,tly,shz+1),(thx,sly-1,thz)))

        if tlx > slx and tly > sly and thz < shz:
            result.inside_self.append(Box((slx,sly,thz+1),(tlx-1,tly-1,shz)))
        if tlx < slx and tly < sly and thz > shz:
            result.inside_test.append(Box((tlx,tly,shz+1),(slx-1,sly-1,thz)))

        if thx < shx and thy < shy and tlz > slz:
            result.inside_self.append(Box((thx+1,thy+1,slz),(shx,shy,tlz-1)))
        if thx > shx and thy > shy and tlz < slz:
            result.inside_test.append(Box((shx+1,shy+1,tlz),(thx,thy,slz-1)))

        if tlx > slx and thy < shy and tlz > slz:
            result.inside_self.append(Box((slx,thy+1,slz),(tlx-1,shy,tlz-1)))
        if tlx < slx and thy > shy and tlz < slz:
            result.inside_test.append(Box((tlx,shy+1,tlz),(slx-1,thy,slz-1)))

        if thx < shx and tly > sly and tlz > slz:
            result.inside_self.append(Box((thx+1,sly,slz),(shx,tly-1,tlz-1)))
        if thx > shx and tly < sly and tlz < slz:
            result.inside_test.append(Box((shx+1,tly,tlz),(thx,sly-1,slz-1)))

        if tlx > slx and tly > sly and tlz > slz:
            result.inside_self.append(Box((slx,sly,slz),(tlx-1,tly-1,tlz-1)))
        if tlx < slx and tly < sly and tlz < slz:
            result.inside_test.append(Box((tlx,tly,tlz),(slx-1,sly-1,slz-1)))

#Edges

        if tlx < slx and tly < sly:
            result.inside_test.append(Box((tlx,tly,rlz),(slx-1,sly-1,rhz)))

        if tlx > slx and tly > sly:
            result.inside_self.append(Box((slx,sly,rlz),(tlx-1,tly-1,rhz)))

        if tlx < slx and thy > shy:
            result.inside_test.append(Box((tlx,shy+1,rlz),(slx-1,thy,rhz)))

        if tlx > slx and thy < shy:
            result.inside_self.append(Box((slx,thy+1,rlz),(tlx-1,shy,rhz)))

        if thx > shx and tly < sly:
            result.inside_test.append(Box((shx+1,tly,rlz),(thx,sly-1,rhz)))

        if thx < shx and tly > sly:
            result.inside_self.append(Box((thx+1,sly,rlz),(shx,tly-1,rhz)))

        if thx > shx and thy > shy:
            result.inside_test.append(Box((shx+1,shy+1,rlz),(thx,thy,rhz)))

        if thx < shx and thy < shy:
            result.inside_self.append(Box((thx+1,thy+1,rlz),(shx,shy,rhz)))

        if tlz < slz and tly < sly:
            result.inside_test.append(Box((rlx,tly,tlz),(rhx,sly-1,slz-1)))

        if tlz > slz and tly > sly:
            result.inside_self.append(Box((rlx,sly,slz),(rhx,tly-1,tlz-1)))

        if tlz < slz and thy > shy:
            result.inside_test.append(Box((rlx,shy+1,tlz),(rhx,thy,slz-1)))

        if tlz> slz and thy < shy:
            result.inside_self.append(Box((rlx,thy+1,slz),(rhx,shy,tlz-1)))

        if thz > shz and tly < sly:
            result.inside_test.append(Box((rlx,tly,shz+1),(rhx,sly-1,thz)))

        if thz < shz and tly > sly:
            result.inside_self.append(Box((rlx,sly,thz+1),(rhx,tly-1,shz)))

        if thz > shz and thy > shy:
            result.inside_test.append(Box((rlx,shy+1,shz+1),(rhx,thy,thz)))

        if thz < shz and thy < shy:
            result.inside_self.append(Box((rlx,thy+1,thz+1),(rhx,shy,shz)))

        if tlx < slx and tlz < slz:
            result.inside_test.append(Box((tlx,rly,tlz),(slx-1,rhy,slz-1)))

        if tlx > slx and tlz > slz:
            result.inside_self.append(Box((slx,rly,slz),(tlx-1,rhy,tlz-1)))

        if tlx < slx and thz > shz:
            result.inside_test.append(Box((tlx,rly,shz+1),(slx-1,rhy,thz)))

        if tlx > slx and thz < shz:
            result.inside_self.append(Box((slx,rly,thz+1),(tlx-1,rhy,shz)))

        if thx > shx and tlz < slz:
            result.inside_test.append(Box((shx+1,rly,tlz),(thx,rhy,slz-1)))

        if thx < shx and tlz > slz:
            result.inside_self.append(Box((thx+1,rly,slz),(shx,rhy,tlz-1)))

        if thx > shx and thz > shz:
            result.inside_test.append(Box((shx+1,rly,shz+1),(thx,rhy,thz)))

        if thx < shx and thz < shz:
            result.inside_self.append(Box((thx+1,rly,thz+1),(shx,rhy,shz)))

#Sides

        if tlx < slx:
            result.inside_test.append(Box((tlx,rly,rlz),(slx-1,rhy,rhz)))
        if tlx > slx:
            result.inside_self.append(Box((slx,rly,rlz),(tlx-1,rhy,rhz)))

        if thx > shx:
            result.inside_test.append(Box((shx+1,rly,rlz),(thx,rhy,rhz)))
        if thx < shx:
            result.inside_self.append(Box((thx+1,rly,rlz),(shx,rhy,rhz)))

        if tly < sly:
            result.inside_test.append(Box((rlx,tly,rlz),(rhx,sly-1,rhz)))
        if tly > sly:
            result.inside_self.append(Box((rlx,sly,rlz),(rhx,tly-1,rhz)))

        if thy > shy:
            result.inside_test.append(Box((rlx,shy+1,rlz),(rhx,thy,rhz)))
        if thy < shy:
            result.inside_self.append(Box((rlx,thy+1,rlz),(rhx,shy,rhz)))

        if tlz < slz:
            result.inside_test.append(Box((rlx,rly,tlz),(rhx,rhy,slz-1)))
        if tlz > slz:
            result.inside_self.append(Box((rlx,rly,slz),(rhx,rhy,tlz-1)))

        if thz > shz:
            result.inside_test.append(Box((rlx,rly,shz+1),(rhx,rhy,thz)))
        if thz < shz:
            result.inside_self.append(Box((rlx,rly,thz+1),(rhx,rhy,shz)))

        return result

def parse_line(line:str)->Tuple[str,Box]:
    line = line.strip()
    [on_off,rest] = line.split(" ")
    entries = rest.split(",")
    line_item = dict()
    for entry in entries:
        [axis,coords] = entry.split("=")
        [low,high] = [int(x) for x in coords.split("..")]
        line_item[axis] = (low,high)
    curr_box = Box((line_item["x"][0],line_item["y"][0],line_item["z"][0]),(line_item["x"][1],line_item["y"][1],line_item["z"][1]))
    return (on_off,curr_box)

def main():
    with open(f"{os.path.dirname(__file__)}/input.txt","r") as in_file:
        
        active_boxes: List[Box] = list()

        for line in in_file:
            next_active_boxes: List[Box] = list()
            on_off, line_box = parse_line(line)
            if on_off == "on":
                to_be_added = [line_box]
                intersecting_boxes = [a for a in active_boxes if a.intersects(line_box)]
                for i_box in intersecting_boxes:
                    next_to_be_added:List["Box"] = list()
                    for t_box in to_be_added:
                        result = t_box.intersection(i_box)
                        next_to_be_added += result.inside_self
                    to_be_added = next_to_be_added
                next_active_boxes = to_be_added + active_boxes
            else:
                intersecting_boxes = [a for a in active_boxes if a.intersects(line_box)]
                safe_boxes = [a for a in active_boxes if not a.intersects(line_box)]
                to_be_added:List["Box"] = list()
                for i_box in intersecting_boxes:
                    result = line_box.intersection(i_box)
                    to_be_added += result.inside_test
                next_active_boxes = safe_boxes + to_be_added
            active_boxes = next_active_boxes
        
        total = sum([x.size() for x in active_boxes])
        print(f"solution: {total}")
            


if __name__ == "__main__":
    main()