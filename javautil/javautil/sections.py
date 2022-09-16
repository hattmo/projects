from typing import List
from javautil.attributes import Unknown_Attribute
from javautil.consts import Utf8
import struct


class Class_Access:

    ACC_PUBLIC = 0x0001
    ACC_FINAL = 0x0010
    ACC_SUPER = 0x0020
    ACC_INTERFACE = 0x0200
    ACC_ABSTRACT = 0x0400
    ACC_SYNTHETIC = 0x1000
    ACC_ANNOTATION = 0x2000
    ACC_ENUM = 0x4000


class Method_access:
    ACC_PUBLIC = 0x0001
    ACC_PRIVATE = 0x0002
    ACC_PROTECTED = 0x0004
    ACC_STATIC = 0x0008
    ACC_FINAL = 0x0010
    ACC_SYNCHRONIZED = 0x0020
    ACC_BRIDGE = 0x0040
    ACC_VARARGS = 0x0080
    ACC_NATIVE = 0x0100
    ACC_ABSTRACT = 0x0400
    ACC_STRICT = 0x0800
    ACC_SYNTHETIC = 0x1000


class Field_Access:
    ACC_PUBLIC = 0x0001
    ACC_PRIVATE = 0x0002
    ACC_PROTECTED = 0x0004
    ACC_STATIC = 0x0008
    ACC_FINAL = 0x0010
    ACC_VOLATILE = 0x0040
    ACC_TRANSIENT = 0x0080
    ACC_SYNTHETIC = 0x1000
    ACC_ENUM = 0x4000


class Nested_Class_Access:
    ACC_PUBLIC = 0x0001
    ACC_PRIVATE = 0x0002
    ACC_PROTECTED = 0x0004
    ACC_STATIC = 0x0008
    ACC_FINAL = 0x0010
    ACC_INTERFACE = 0x0200
    ACC_ABSTRACT = 0x0400
    ACC_SYNTHETIC = 0x1000
    ACC_ANNOTATION = 0x2000
    ACC_ENUM = 0x4000


class Field_Info:
    def __init__(self, access_flags: int, name_index: Utf8,
                 descriptor_index: Utf8):
        self.access_flags = access_flags
        self.name_index = name_index
        self.descriptor_index = descriptor_index
        self.attributes: List[Unknown_Attribute] = list()

    def add_attribute_raw(self, attribute_info:Unknown_Attribute):
        self.attributes.append(attribute_info)

    def __resolve_flags(self):
        out:List[str] = list()
        if self.access_flags & Field_Access.ACC_PUBLIC:
            out.append("public")
        if self.access_flags & Field_Access.ACC_PRIVATE:
            out.append("private")
        if self.access_flags & Field_Access.ACC_PROTECTED:
            out.append("protected")
        if self.access_flags & Field_Access.ACC_STATIC:
            out.append("static")
        if self.access_flags & Field_Access.ACC_FINAL:
            out.append("final")
        if self.access_flags & Field_Access.ACC_VOLATILE:
            out.append("volatile")
        if self.access_flags & Field_Access.ACC_TRANSIENT:
            out.append("transient")
        if self.access_flags & Field_Access.ACC_SYNTHETIC:
            out.append("synthetic")
        if self.access_flags & Field_Access.ACC_ENUM:
            out.append("enum")
        return ', '.join(out)

    def __str__(self)->str:
        out = f"access flags: {self.__resolve_flags()}\n"
        out += f"name: {self.name_index.resolve()}\n"
        out += f"descriptor: {self.descriptor_index.resolve()}\n"
        out += f"attributes: {len(self.attributes)}"
        return out

    def to_bytes(self)->bytes:
        out = struct.pack(">HHHH", self.access_flags, self.name_index.index,
                          self.descriptor_index.index, len(self.attributes))
        for attr in self.attributes:
            out += attr.to_bytes()
        return out


class Method_Info:
    def __init__(self, access_flags: int, name_index: Utf8,
                 descriptor_index: Utf8):
        self.access_flags = access_flags
        self.name_index = name_index
        self.descriptor_index = descriptor_index
        self.attributes: List[Unknown_Attribute] = list()

    def add_attribute_raw(self, attribute_info):
        self.attributes.append(attribute_info)

    def __str__(self):
        out = f"access flags: {self.access_flags}\n"
        out += f"name: {self.name_index.resolve()}\n"
        out += f"descriptor: {self.descriptor_index.resolve()}\n"
        out += f"attributes: {len(self.attributes)}"
        return out

    def to_bytes(self):
        out = struct.pack(">HHHH", self.access_flags, self.name_index.index,
                          self.descriptor_index.index, len(self.attributes))
        for attr in self.attributes:
            out += attr.to_bytes()
        return out
