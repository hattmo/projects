from dataclasses import dataclass
import re
from typing import Optional


class Ip_Item:
    upper: int
    lower: int

    def __init__(self, upper: int, lower: int):
        self.upper = upper
        self.lower = lower

    def __str__(self) -> str:
        out = ""
        for i in range(4):
            octet = (self.lower >> 8 * (3 - i)) & 0xFF
            out += str(octet)
            if i < 3:
                out += "."
        out += "-"
        for i in range(4):
            octet = (self.upper >> 8 * (3 - i)) & 0xFF
            out += str(octet)
            if i < 3:
                out += "."
        return out

    def __repr__(self) -> str:
        return self.__str__()


class Ips:
    _ips: list[Ip_Item]

    def __init__(self, ips: list[Ip_Item]):
        self._ips = ips
        self._current = None

    def __iter__(self):
        return Ip_Iter(self._ips)


class Ip_Iter:

    def __init__(self, ips: list[Ip_Item]) -> None:
        self._ips: list[Ip_Item] = ips
        self._index: int = 0
        self._current = ips[0].lower
        self._upper = ips[0].upper


    def __next__(self):
        if self._index >= len(self._ips):
            raise StopIteration
        out = self._current
        self._current += 1
        if self._current >= self._upper:
            self._index += 1
            if self._index < len(self._ips):
                self._current = self._ips[self._index].lower
                self._upper = self._ips[self._index].upper
        return int_to_ip(out)
    
def int_to_ip(ip_int: int) -> str:
    out = ""
    for i in range(4):
        octet = (ip_int >> 8 * (3 - i)) & 0xFF
        out += str(octet)
        if i < 3:
            out += "."
    return out

def parse_ips(ip_string: str):
    regex = re.compile(
        r"(\d{1,3}|\d{1,3}-\d{1,3})\.(\d{1,3}|\d{1,3}-\d{1,3})\.(\d{1,3}|\d{1,3}-\d{1,3})\.(\d{1,3}|\d{1,3}-\d{1,3})(\/(\d{1,2}))?"
    )
    ips = ip_string.split(",")
    ips = list(map(lambda x: x.strip(), ips))
    ip_items: list[Ip_Item] = []
    for ip in ips:
        match = regex.fullmatch(ip)
        if not match:
            raise ValueError("Invalid IP address")
        if ip.count("-") > 1:
            raise ValueError("Invalid IP address")
        if ip.count("/") > 0 and ip.count("-") > 0:
            raise ValueError("Invalid IP address")
        lower = 0
        upper = 0
        for i in range(4):
            octet = match.group(i + 1)
            if octet.count("-") > 0:
                octet_parts = octet.split("-")
                lower_oct_num = int(octet_parts[0])
                upper_oct_num = int(octet_parts[1])
                if lower_oct_num > 255 or upper_oct_num > 255:
                    raise ValueError("Invalid IP address")
                lower += lower_oct_num << 8 * (3 - i)
                upper += upper_oct_num << 8 * (3 - i)
                mask = (1 << (8 * (3 - i))) - 1
                upper |= mask
                break
            else:
                oct_num = int(octet)
                if oct_num > 255:
                    raise ValueError("Invalid IP address")
                lower += oct_num << 8 * (3 - i)
                upper += oct_num << 8 * (3 - i)
        if match.group(6):
            cidr = int(match.group(6))
            if cidr > 32:
                raise ValueError("Invalid IP address")
            lower = lower & (0xFFFFFFFF << (32 - cidr))
            upper = upper | (0xFFFFFFFF >> cidr)
        upper += 1
        ip_items.append(Ip_Item(upper, lower))
    return Ips(ip_items)
