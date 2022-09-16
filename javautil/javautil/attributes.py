import struct
from javautil.interfaces import Linkable
from .sections import Attribute_Info


def parse_attribute(attr: Attribute_Info):
    if attr.const_pool == None:
        return None


class Unknown_Attribute(Linkable):
    def __init__(self, attribute_name_index: int, info: bytes):
        super().__init__()
        self.attribute_name_index = attribute_name_index
        self.info = info

    def __str__(self):
        return ""

    def link():
        pass

    def to_bytes(self):
        return struct.pack(f">HL{len(self.info)}s", self.attribute_name_index,
                           len(self.info), self.info)


class Code_Attribute:
    def __init__(self):
        self.attribute_name_index = 0
        self.maxstack = 0
        self.max_locals = 0
        self.code = b''
