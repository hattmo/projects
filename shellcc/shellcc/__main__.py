import argparse
from typing import Dict
import random
import string
from shellcc.preprocess import preprocess


class Module:
    pass


def main():
    parser = argparse.ArgumentParser("shellcc")
    parser.add_argument("files", nargs="+")
    parser.add_argument("-o", dest="output")
    args = parser.parse_args()
    for module in args.files:
        print(preprocess(module, {}, 10))


if __name__ == "__main__":
    main()
