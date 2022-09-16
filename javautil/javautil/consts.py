from abc import abstractmethod
import struct


class Const_Type:
    const_type: int

    def set_index(self, index: int):
        self.index = index

    @abstractmethod
    def to_bytes(self)->bytes:
        pass

    def __lt__(self, other: "Const_Type"):
        if self.const_type == other.const_type:
            return self.index < other.index
        return self.const_type < other.const_type


class Class_Info(Const_Type):
    def __init__(self, name_index: "Utf8"):
        self.name_index = name_index
        self.const_type = 7

    def __str__(self):
        leftside = f"{'#' + str(self.index):>4} - Class: #{self.name_index.index}"
        return f"{leftside:<40.40}// {self.resolve()}"

    def resolve(self):
        return self.name_index.resolve()

    def to_bytes(self):
        return struct.pack(">BH", self.const_type, self.name_index.index)


class Fieldref_Info(Const_Type):
    def __init__(self, class_index: Class_Info,
                 name_and_type_index: "Name_And_Type_Info"):
        self.class_index = class_index
        self.name_and_type_index = name_and_type_index
        self.const_type = 9

    def __str__(self):
        leftside = f"{'#' + str(self.index):>4} - Fieldref: #{self.class_index.index}, #{self.name_and_type_index.index}"
        return f"{leftside:<40.40}// {self.resolve()}"

    def resolve(self):
        return f'({self.class_index.resolve()}) {self.name_and_type_index.resolve()}'

    def to_bytes(self):
        return struct.pack(">BHH", self.const_type, self.class_index.index,
                           self.name_and_type_index.index)


class Methodref_Info(Const_Type):
    def __init__(self, class_index: Class_Info,
                 name_and_type_index: "Name_And_Type_Info"):
        self.class_index = class_index
        self.name_and_type_index = name_and_type_index
        self.const_type = 10

    def __str__(self):
        leftside = f"{'#' + str(self.index):>4} - Methodref: #{self.class_index.index}, #{self.name_and_type_index.index}"
        return f"{leftside:<40.40}// {self.resolve()}"

    def resolve(self):
        return f'({self.class_index.resolve()}) {self.name_and_type_index.resolve()}'

    def to_bytes(self):
        return struct.pack(">BHH", self.const_type, self.class_index.index,
                           self.name_and_type_index.index)


class Interface_Methodref_Info(Const_Type):
    def __init__(self, class_index: Class_Info,
                 name_and_type_index: "Name_And_Type_Info"):
        self.class_index = class_index
        self.name_and_type_index = name_and_type_index
        self.const_type = 11

    def __str__(self):
        leftside = f"{'#' + str(self.index):>4} - Interface Methodref: #{self.class_index.index}, #{self.name_and_type_index.index}"
        return f"{leftside:<40.40}// {self.resolve()}"

    def resolve(self):
        return f'({self.class_index.resolve()}) {self.name_and_type_index.resolve()}'

    def to_bytes(self):
        return struct.pack(">BHH", self.const_type, self.class_index.index,
                           self.name_and_type_index.index)


class String_Info(Const_Type):
    def __init__(self, string_index: "Utf8"):
        self.string_index = string_index
        self.const_type = 8

    def __str__(self):
        leftside = f"{'#' + str(self.index):>4} - String: #{self.string_index.index}"
        return f"{leftside:<40.40}// {self.resolve()}"

    def resolve(self):
        return self.string_index.resolve()

    def to_bytes(self):
        return struct.pack(">BH", self.const_type, self.string_index.index)


class Integer_Info(Const_Type):
    def __init__(self, int_bytes: int):
        self.int_bytes = int_bytes
        self.const_type = 3

    def __str__(self):
        return f"{'#' + str(self.index):>4} - Integer: {self.int_bytes}"

    def to_bytes(self):
        return struct.pack(">BI", self.const_type, self.int_bytes)


class Float_Info(Const_Type):
    def __init__(self, float_bytes: float):
        self.float_bytes = float_bytes
        self.const_type = 4

    def __str__(self):
        return f"{'#' + str(self.index):>4} - Float: {self.float_bytes}"

    def to_bytes(self):
        return struct.pack(">Bf", self.const_type, self.float_bytes)


class Long_Info(Const_Type):
    def __init__(self, long_bytes: int):
        super().__init__()
        self.long_bytes = long_bytes
        self.const_type = 5

    def __str__(self):
        return f"{'#' + str(self.index):>4} - Long: {self.long_bytes}"

    def to_bytes(self):
        return struct.pack(">BQ", self.const_type, self.long_bytes)


class Double_Info(Const_Type):
    def __init__(self, double_bytes: float):
        super().__init__()
        self.double_bytes = double_bytes
        self.const_type = 6

    def __str__(self):
        return f"{'#' + str(self.index):>4} - Double: {self.double_bytes}"

    def to_bytes(self):
        return struct.pack(">Bd", self.const_type, self.double_bytes)


class Name_And_Type_Info(Const_Type):
    def __init__(self, name_index: "Utf8", descriptor_index: "Utf8"):
        super().__init__()
        self.name_index = name_index
        self.descriptor_index = descriptor_index
        self.const_type = 12

    def __str__(self):
        leftside = f"{'#' + str(self.index):>4} - Name and Type: #{self.name_index.index}, #{self.descriptor_index.index}"
        return f"{leftside:<40.40}// {self.resolve()}"

    def resolve(self):
        return f"{self.name_index.resolve()} {self.descriptor_index.resolve()}"

    def to_bytes(self):
        return struct.pack(">BHH", self.const_type, self.name_index.index,
                           self.descriptor_index.index)


class Utf8(Const_Type):
    def __init__(self, data: bytes):
        super().__init__()
        self.data = data
        self.const_type = 1

    def __str__(self):
        return f"{'#' + str(self.index):>4} - Utf8: {self.resolve()}"


# TODO: String encoding is not actually UTF-8 but should be ok for most english code

    def resolve(self):
        try:
            return self.data.decode()
        except:
            return "Could not decode"

    def to_bytes(self):
        return struct.pack(f">BH{len(self.data)}s", self.const_type,
                           len(self.data), self.data)


class Method_Handle_Info(Const_Type):
    def __init__(self, reference_kind: int, reference_index: Const_Type):
        super().__init__()
        self.reference_kind = reference_kind
        self.reference_index = reference_index
        self.const_type = 15

    def __str__(self):
        return f"{'#' + str(self.index):>4} - Method Handle: {self.reference_kind}, #{self.reference_index.index}"

    def to_bytes(self):
        return struct.pack(">BHH", self.const_type, self.reference_kind,
                           self.reference_index.index)


class Method_Type_Info(Const_Type):
    def __init__(self, descriptor_index: "Utf8"):
        super().__init__()
        self.descriptor_index = descriptor_index
        self.const_type = 16

    def __str__(self):
        leftside =  f"{'#' + str(self.index):>4} - Method Type: #{self.descriptor_index.index}"
        return f"{leftside:<40.40}// {self.resolve()}"

    def resolve(self):
        return f"{self.descriptor_index.resolve()}"

    def to_bytes(self):
        return struct.pack(">BH", self.const_type, self.descriptor_index.index)


class Invoke_Dynamic_Info(Const_Type):
    def __init__(self, bootstrap_method_attr_index: int,
                 name_and_type_index: Name_And_Type_Info):
        super().__init__()
        self.bootstrap_method_attr_index = bootstrap_method_attr_index
        self.name_and_type_index = name_and_type_index
        self.const_type = 18

    def __str__(self):
        return f"{'#' + str(self.index):>4} - Invoke Dynamic: #{self.bootstrap_method_attr_index}, #{self.name_and_type_index}"

    def to_bytes(self):
        return struct.pack(">BHH", 18, self.bootstrap_method_attr_index,
                           self.name_and_type_index)
