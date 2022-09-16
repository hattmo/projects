#!/bin/env python
import subprocess
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("base")
parser.add_argument("start")
parser.add_argument("end")
args = parser.parse_args()
start = int(args.start)
end = int(args.end)
base = args.base

for i in range(start, end + 1):
    ip = f"{base}.{i}"
    res = subprocess.run(
        ["sudo", "ping", "-q", "-n", "-c", "1", "-w", "1", ip],
        capture_output=True,
        encoding="utf8",
    )
    for line in res.stdout.splitlines():
        if "packets transmitted" in line and line.split()[3] == "1":
            print(ip)
