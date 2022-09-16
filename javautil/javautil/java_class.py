from typing import List, Union

from javautil.attributes import Unknown_Attribute
from javautil.consts import *
from javautil.sections import *
import struct


class Java_Class:

    def __init__(self):
        self.magic = b'\xca\xfe\xba\xbe'
        self.minor_version = 0
        self.major_version = 55
        self.access_flags = 0
        self.this_class = None
        self.super_class = None
        self.const_pool: List[Union[None,Const_Type]] = list()
        self.interfaces: List[Class_Info] = list()
        self.fields: List[Field_Info] = list()
        self.methods: List[Method_Info] = list()
        self.attributes: List[Unknown_Attribute] = list()

    def add_constant_raw(self, data: Const_Type):
        data.set_index(len(self.const_pool) + 1)
        self.const_pool.append(data)
        if type(data) == Double_Info or type(data) == Long_Info:
            self.const_pool.append(None)

    def add_interface_raw(self, class_ref: Class_Info):
        self.interfaces.append(class_ref)

    def add_field_raw(self, field_info: Field_Info):
        self.fields.append(field_info)

    def add_method_raw(self, method_info: Method_Info):
        self.methods.append(method_info)

    def add_attribute_raw(self, attribute_info: Unknown_Attribute):
        self.attributes.append(attribute_info)

    def to_bytes(self):
        out = b""
        out += struct.pack(">4sHHH", self.magic, self.minor_version,
                           self.major_version,
                           len(self.const_pool) + 1)
        for const in self.const_pool:
            if const == None:
                continue
            else:
                out += const.to_bytes()
        out += struct.pack(">HHHH", self.access_flags, self.this_class,
                           self.super_class, len(self.interfaces))
        for iface in self.interfaces:
            out += struct.pack(">H", iface)
        out += struct.pack(">H", len(self.fields))
        for field in self.fields:
            out += field.to_bytes()
        out += struct.pack(">H", len(self.methods))
        for method in self.methods:
            out += method.to_bytes()
        out += struct.pack(">H", len(self.attributes))
        for attr in self.attributes:
            out += attr.to_bytes()
        return out

    def __str__(self):
        out = ""
        out += f"Version: {self.major_version}:{self.minor_version}\n"
        out += f"\n"
        out += f"Interfaces: {len(self.interfaces)} Fields: {len(self.fields)} Methods: {len(self.methods)} Attributes: {len(self.attributes)}\n"
        out += "-----Constant_Pool-----\n"
        for c in sorted([x for x in self.const_pool if x != None]):
            out += f'{c}\n'
        for f in self.fields:
            out += f'\n{f}\n'
        for m in self.methods:
            out += f'\n{m}\n'
        return out

    @staticmethod
    def from_bytes(input_bytes: bytes):
        cursor = 0

        def read_struct(fmt:"str"):
            nonlocal cursor
            chunk_size = struct.calcsize(fmt)
            values = struct.unpack(fmt,
                                   input_bytes[cursor:cursor + chunk_size])
            cursor += chunk_size
            return values

        (magic, minor_version, major_version,
         contant_pool_count) = read_struct(">4sHHH")
        out_java = Java_Class()
        out_java.magic = magic
        out_java.minor_version = minor_version
        out_java.major_version = major_version


        const_index = 1
        while const_index < contant_pool_count:
            (tag, ) = read_struct("B")
            if tag == 7:
                (name_index, ) = read_struct(">H")
                out_java.add_constant_raw(Class_Info(name_index))
            elif tag == 9:
                (class_index, name_and_type_index) = read_struct(">HH")
                out_java.add_constant_raw(
                    Fieldref_Info(class_index, name_and_type_index))
            elif tag == 10:
                (class_index, name_and_type_index) = read_struct(">HH")
                out_java.add_constant_raw(
                    Methodref_Info(class_index, name_and_type_index))
            elif tag == 11:
                (class_index, name_and_type_index) = read_struct(">HH")
                out_java.add_constant_raw(
                    Interface_Methodref_Info(class_index, name_and_type_index))
            elif tag == 8:
                (string_index, ) = read_struct(">H")
                out_java.add_constant_raw(String_Info(string_index))
            elif tag == 3:
                (int_bytes, ) = read_struct(">I")
                out_java.add_constant_raw(Integer_Info(int_bytes))
            elif tag == 4:
                (float_bytes, ) = read_struct(">f")
                out_java.add_constant_raw(Float_Info(float_bytes))
            elif tag == 5:
                (long_bytes, ) = read_struct(">Q")
                out_java.add_constant_raw(Long_Info(long_bytes))
                const_index += 1
            elif tag == 6:
                (double_bytes, ) = read_struct("d")
                out_java.add_constant_raw(Double_Info(double_bytes))
                const_index += 1
            elif tag == 12:
                (name_index, descriptor_index) = read_struct(">HH")
                out_java.add_constant_raw(
                    Name_And_Type_Info(name_index, descriptor_index))
            elif tag == 1:
                (length, )= read_struct(">H")
                (utf8_bytes, ) = read_struct(f"{length}s")
                out_java.add_constant_raw(Utf8(utf8_bytes))
            elif tag == 15:
                (reference_kind, reference_index) = read_struct(">HH")
                out_java.add_constant_raw(
                    Method_Handle_Info(reference_kind, reference_index))
            elif tag == 16:
                (descriptor_index, ) = read_struct(">H")
                out_java.add_constant_raw(Method_Type_Info(descriptor_index))
            elif tag == 18:
                (bootstrap_method_attr_index,
                 name_and_type_index) = read_struct(">HH")
                out_java.add_constant_raw(
                    Invoke_Dynamic_Info(bootstrap_method_attr_index,
                                        name_and_type_index))
            const_index += 1
        (access_flags, this_class, super_class,
         interfaces_count) = read_struct(">HHHH")
        out_java.access_flags = access_flags
        out_java.this_class = this_class
        out_java.super_class = super_class

        for _ in range(interfaces_count):
            (interface, ) = read_struct(">H")
            out_java.add_interface_raw(interface)

        (field_count, ) = read_struct(">H")
        for _ in range(field_count):
            (access_flags, name_index, descriptor_index,
             attributes_count) = read_struct(">HHHH")
            field = Field_Info(access_flags, name_index, descriptor_index)
            for _ in range(attributes_count):
                (attribute_name_index, attribute_length) = read_struct(">HL")
                (info, ) = read_struct(f"{attribute_length}s")
                field.add_attribute_raw(
                    Attribute_Info(attribute_name_index, info))
            out_java.add_field_raw(field)

        (methods_count, ) = read_struct(">H")
        for _ in range(methods_count):
            (access_flags, name_index, descriptor_index,
             attributes_count) = read_struct(">HHHH")
            method = Method_Info(access_flags, name_index, descriptor_index)
            for _ in range(attributes_count):
                (attribute_name_index, attribute_length) = read_struct(">HL")
                (info, ) = read_struct(f"{attribute_length}s")
                method.add_attribute_raw(
                    Attribute_Info(attribute_name_index, info))
            out_java.add_method_raw(method)

        (attributes_count, ) = read_struct(">H")
        for _ in range(attributes_count):
            (attribute_name_index, attribute_length) = read_struct(">HL")
            (info, ) = read_struct(f"{attribute_length}s")
            out_java.add_attribute_raw(
                Attribute_Info(attribute_name_index, info))
        return out_java
