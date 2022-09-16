from typing import Callable, Dict, List
from .interfaces import Const_Type


class Local:
    def __init__(self, is_double=False):
        self.index = 0
        self.is_double = is_double


class Op():
    def __init__(self, bytecode: bytes, args: tuple):
        self._args = args
        self._bytecode = bytecode
        self.index = 0

    def size(self):
        if self._bytecode == b'\xaa' or self._bytecode == b'\xab':
            pad = 3 - (self.index % 4)
            return op_code_lookup[self._bytecode].size(self._args + (pad, ))
        else:
            return op_code_lookup[self._bytecode].size(self._args)

    def to_bytes(self):
        if self._bytecode == b'\xaa' or self._bytecode == b'\xab':
            pad = 3 - (self.index % 4)
            return op_code_lookup[self._bytecode].to_bytes(
                self._bytecode, self._args + (pad, ))
        else:
            return op_code_lookup[self._bytecode].to_bytes(
                self._bytecode, self._args)


class Code:
    def __init__(self):
        self._ops: list[Op] = list()

    def __add__(self, other: 'Code') -> 'Code':
        out = Code()
        out._ops = self._ops + other._ops
        self._fix_alignment(out._ops)
        return out

    def append(self, item: Op) -> None:
        self._ops.append(item)
        Code._fix_alignment(self._ops)

    def __getitem__(self, index: int):
        return self._ops[index]

    def to_bytes(self) -> bytes:
        out = b''
        for op in self._ops:
            out += op.to_bytes()
        return out

    def _fix_alignment(self, ops: List[Op]) -> None:
        offset = 0
        for op in ops:
            op.index = offset
            offset += op.size()


class Op_Code_Builder():
    def __init__(self, bytecode: bytes):
        self._bytecode = bytecode

    def __call__(self, *args):
        op_code_lookup[self._bytecode].check(args)
        new_op = Op(self._bytecode, args)
        out = Code()
        out._ops.append(new_op)
        return out


class Op_Codes:
    def __dir__(self):
        out = super().__dir__()
        return out + mnemonic_lookup.keys()

    def __getattr__(self, name):
        return Op_Code_Builder(mnemonic_lookup[name])


class zero_arg:
    op_codes = [
        b'\x32', b'\x53', b'\x01', b'\x2a', b'\x2b', b'\x2c', b'\x2d', b'\xb0',
        b'\xbe', b'\x4b', b'\x4c', b'\x4d', b'\x4e', b'\xbf', b'\x33', b'\x54',
        b'\xca', b'\x34', b'\x55', b'\x90', b'\x8e', b'\x8f', b'\x63', b'\x31',
        b'\x52', b'\x98', b'\x97', b'\x0e', b'\x0f', b'\x6f', b'\x26', b'\x27',
        b'\x28', b'\x29', b'\x6b', b'\x77', b'\x73', b'\xaf', b'\x47', b'\x48',
        b'\x49', b'\x4a', b'\x67', b'\x59', b'\x5a', b'\x5b', b'\x5c', b'\x5d',
        b'\x5e', b'\x8d', b'\x8b', b'\x8c', b'\x62', b'\x30', b'\x51', b'\x96',
        b'\x95', b'\x0b', b'\x0c', b'\x0d', b'\x6e', b'\x22', b'\x23', b'\x24',
        b'\x25', b'\x6a', b'\x76', b'\x72', b'\xae', b'\x43', b'\x44', b'\x45',
        b'\x46', b'\x66', b'\x91', b'\x92', b'\x87', b'\x86', b'\x85', b'\x93',
        b'\x60', b'\x2e', b'\x7e', b'\x4f', b'\x02', b'\x03', b'\x04', b'\x05',
        b'\x06', b'\x07', b'\x08', b'\x6c', b'\x1a', b'\x1b', b'\x1c', b'\x1d',
        b'\xfe', b'\xff', b'\x68', b'\x74', b'\x80', b'\x70', b'\xac', b'\x78',
        b'\x7a', b'\x3b', b'\x3c', b'\x3d', b'\x3e', b'\x64', b'\x7c', b'\x82',
        b'\x8a', b'\x89', b'\x88', b'\x61', b'\x2f', b'\x7f', b'\x50', b'\x94',
        b'\x09', b'\x0a', b'\x6d', b'\x1e', b'\x1f', b'\x20', b'\x21', b'\x69',
        b'\x75', b'\x81', b'\x71', b'\xad', b'\x79', b'\x7b', b'\x3f', b'\x40',
        b'\x41', b'\x42', b'\x65', b'\x7d', b'\x83', b'\xc2', b'\xc3', b'\x00',
        b'\x57', b'\x58', b'\xb1', b'\x35', b'\x56', b'\x5f'
    ]
    mnemonics = [
        'aaload', 'aastore', 'aconst_null', 'aload_0', 'aload_1', 'aload_2',
        'aload_3', 'areturn', 'arraylength', 'astore_0', 'astore_1',
        'astore_2', 'astore_3', 'athrow', 'baload', 'bastore', 'breakpoint',
        'caload', 'castore', 'd2f', 'd2i', 'd2l', 'dadd', 'daload', 'dastore',
        'dcmpg', 'dcmpl', 'dconst_0', 'dconst_1', 'ddiv', 'dload_0', 'dload_1',
        'dload_2', 'dload_3', 'dmul', 'dneg', 'drem', 'dreturn', 'dstore_0',
        'dstore_1', 'dstore_2', 'dstore_3', 'dsub', 'dup', 'dup_x1', 'dup_x2',
        'dup2', 'dup2_x1', 'dup2_x2', 'f2d', 'f2i', 'f2l', 'fadd', 'faload',
        'fastore', 'fcmpg', 'fcmpl', 'fconst_0', 'fconst_1', 'fconst_2',
        'fdiv', 'fload_0', 'fload_1', 'fload_2', 'fload_3', 'fmul', 'fneg',
        'frem', 'freturn', 'fstore_0', 'fstore_1', 'fstore_2', 'fstore_3',
        'fsub', 'i2b', 'i2c', 'i2d', 'i2f', 'i2l', 'i2s', 'iadd', 'iaload',
        'iand', 'iastore', 'iconst_m1', 'iconst_0', 'iconst_1', 'iconst_2',
        'iconst_3', 'iconst_4', 'iconst_5', 'idiv', 'iload_0', 'iload_1',
        'iload_2', 'iload_3', 'impdep1', 'impdep2', 'imul', 'ineg', 'ior',
        'irem', 'ireturn', 'ishl', 'ishr', 'istore_0', 'istore_1', 'istore_2',
        'istore_3', 'isub', 'iushr', 'ixor', 'l2d', 'l2f', 'l2i', 'ladd',
        'laload', 'land', 'lastore', 'lcmp', 'lconst_0', 'lconst_1', 'ldiv',
        'lload_0', 'lload_1', 'lload_2', 'lload_3', 'lmul', 'lneg', 'lor',
        'lrem', 'lreturn', 'lshl', 'lshr', 'lstore_0', 'lstore_1', 'lstore_2',
        'lstore_3', 'lsub', 'lushr', 'lxor', 'monitorenter', 'monitorexit',
        'nop', 'pop', 'pop2', 'return', 'saload', 'sastore', 'swap'
    ]

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 0:
            raise ValueError("Required arguments: ()")

    @staticmethod
    def size(args: tuple) -> int:
        return 1

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code


class one_arg_1literal:
    op_codes = [b'\xbc', b'\x10']  #newarray primitive, bipush
    mnemonics = ['newarray', "bipush"]

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != int:
            raise ValueError("Required arguments: (int)")

    @staticmethod
    def size(args: tuple) -> int:
        return 2

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].to_bytes(1, 'big')


class one_arg_2literal:
    op_codes = [b'\x11']  #sipush
    mnemonics = ['sipush']

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != int:
            raise ValueError("Required arguments: (int)")

    @staticmethod
    def size(args: tuple) -> int:
        return 3

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].to_bytes(2, 'big')


class one_arg_1local:
    op_codes = [
        b'\x19', b'\x3a', b'\x18', b'\x39', b'\x17', b'\x38', b'\x15', b'\x36',
        b'\x16', b'\x37', b'\xa9'
    ]  # local load store ret
    mnemonics = [
        'aload', 'astore', 'dload', 'dstore', 'fload', 'fstore', 'iload',
        'istore', 'lload', 'lstore', 'ret'
    ]

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != Local:
            raise ValueError("Required arguments: (Local)")

    @staticmethod
    def size(args: tuple) -> int:
        return 2

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(1, 'big')


class one_arg_2local:
    op_codes = [
        b'\xc4\x19', b'\xc4\x3a', b'\xc4\x18', b'\xc4\x39', b'\xc4\x17',
        b'\xc4\x38', b'\xc4\x15', b'\xc4\x36', b'\xc4\x16', b'\xc4\x37',
        b'\xc4\xa9'
    ]  #local load stor ret wide
    mnemonics = [
        'aload_w', 'astore_w', 'dload_w', 'dstore_w', 'fload_w', 'fstore_w',
        'iload_w', 'istore_w', 'lload_w', 'lstore_w', 'ret_w'
    ]

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != Local:
            raise ValueError("Required arguments: (Local)")

    @staticmethod
    def size(args: tuple) -> int:
        return 3

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(2, 'big')


class one_arg_1const:
    op_codes = [b'\x12']  # ldc
    mnemonics = ["ldc"]

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != Const_Type:
            raise ValueError("Required arguments: (Const_Type)")

    @staticmethod
    def size(args: tuple) -> int:
        return 2

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(1, 'big')


class one_arg_2const:
    op_codes = [
        b'\xbd', b'\xc0', b'\xb4', b'\xb2', b'\xc1', b'\xb7', b'\xb8', b'\xb6',
        b'\x13', b'\x14', b'\xbb', b'\xb5', b'\xb3'
    ]  #constant operations (invoke ldc_w load store field static)
    mnemonics = [
        'anewarray', 'checkcast', 'getfield', 'getstatic', 'instanceof',
        'invokespecial', 'invokestatic', 'invokevirtual', 'ldc_w', 'ldc2_w',
        'new', 'putfield', 'putstatic'
    ]

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != Const_Type:
            raise ValueError("Required arguments: (Const_Type)")

    @staticmethod
    def size(args: tuple) -> int:
        return 3

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(2, 'big')


class one_arg_2branch:
    op_codes = [
        b'\xa7', b'\xa5', b'\xa6', b'\x9f', b'\xa2', b'\xa3', b'\xa4', b'\xa1',
        b'\xa0', b'\x99', b'\x9c', b'\x9d', b'\x9e', b'\x9b', b'\x9a', b'\xc7',
        b'\xc6', b'\xa8'
    ]  #goto jsr if<cond>
    mnemonics = [
        'goto', 'if_acmpeq', 'if_acmpne', 'if_icmpeq', 'if_icmpge',
        'if_icmpgt', 'if_icmple', 'if_icmplt', 'if_icmpne', 'ifeq', 'ifge',
        'ifgt', 'ifle', 'iflt', 'ifne', 'innonnull', 'ifnull', 'jsr'
    ]

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != Op:
            raise ValueError("Required arguments: (Op)")

    @staticmethod
    def size(args: tuple) -> int:
        return 3

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(2, 'big')


class one_arg_4branch:
    op_codes = [b'\xc8', b'\xc9']  # goto_w jsr_w
    mnemonics = ['goto_w', 'jsr_w']

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != Op:
            raise ValueError("Required arguments: (Op)")

    @staticmethod
    def size(args: tuple) -> int:
        return 5

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(4, 'big')


class two_arg_1local_1literal:
    op_codes = [b'\x84']  # iinc
    mnemonics = ['iinc']

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 2 or type(args[0]) != Local or type(args[1]) != int:
            raise ValueError("Required arguments: (Local, int)")

    @staticmethod
    def size(args: tuple) -> int:
        return 3

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(1, 'big') + args[1].to_bytes(
            1, 'big')


class two_arg_2local_2literal:
    op_codes = [b'\xc4\x84']  # iinc_w
    mnemonics = ['iinc_w']

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 2 or type(args[0]) != Local or type(args[1]) != int:
            raise ValueError("Required arguments: (Local, int)")

    @staticmethod
    def size(args: tuple) -> int:
        return 5

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(2, 'big') + args[1].to_bytes(
            2, 'big')


class two_arg_2const_1literal:
    op_codes = [b'\xc5']  # multianewarray
    mnemonics = ['multianewarray']

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 2 or type(args[0]) != Const_Type or type(
                args[1]) != int:
            raise ValueError("Required arguments: (Const_Type, int)")

    @staticmethod
    def size(args: tuple) -> int:
        return 4

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(2, 'big') + args[1].to_bytes(
            1, 'big')


class one_arg_2const_2zero:
    op_codes = [b'\xba']  #invokedynamic
    mnemonics = ['invokedynamic']

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 1 or type(args[0]) != Const_Type:
            raise ValueError("Required arguments: (Const_Type)")

    @staticmethod
    def size(args: tuple) -> int:
        return 5

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return op_code + args[0].index.to_bytes(2, 'big') + b'\x00\x00'


class two_arg_2const_1literal_1zero:
    op_codes = [b'\xb9']  #invokeinterface
    mnemonics = ['invokeinterface']

    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 2 or type(args[0]) != Const_Type or type(
                args[1]) != int:
            raise ValueError("Required arguments: (Const_Type, int)")

    @staticmethod
    def size(args: tuple) -> int:
        return 5

    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return b''


class lookupswitch:
    op_codes = [b'\xab']
    mnemonics = ['lookupswitch']
    # (Op, List[Tuple(int,Op)])
    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 2 or type(args[0]) != Op or type(args[1]) != list:
            raise ValueError("Required arguments: (Op, List[Tuple[int,Op]])")

    # (Op, List[Tuple(int,Op)], int #pad)
    @staticmethod
    def size(args: tuple) -> int:
        return 9 + args[2] + (8 * len(args[1]))

    # (Op, List[Tuple(int,Op)], int #pad)
    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return b''


class tableswitch:
    op_codes = [b'\xaa']
    mnemonics = ['tableswitch']
    # (Op, int, int, List(Op))
    @staticmethod
    def check(args: tuple) -> None:
        if len(args) != 4 or type(args[0]) != Op or type(
                args[1]) != int or type(args[2]) != int or type(
                    args[3]) != list:
            raise ValueError("Required arguments: (Op, int, int, List[Op])")

    # (Op, int, int, List(Op), int #pad)
    @staticmethod
    def size(args: tuple) -> int:
        return 9 + args[4] + (4 * len(args[3]))

    # (Op, int, int, List(Op), int #pad)
    @staticmethod
    def to_bytes(op_code: bytes, args: tuple) -> bytes:
        return b''


class Lookup_Entry:
    check: Callable[[tuple], bool]
    size: Callable[[tuple], int]
    to_bytes: Callable[[bytes, tuple], bytes]
    mnemonic: str


op_code_lookup: Dict[bytes, Lookup_Entry] = dict()
mnemonic_lookup: Dict[str, bytes] = dict()

for op_type in (zero_arg, one_arg_1literal, one_arg_2literal, one_arg_1local,
                one_arg_2local, one_arg_1const, one_arg_2const,
                one_arg_2branch, one_arg_4branch, two_arg_1local_1literal,
                two_arg_2local_2literal, two_arg_2const_1literal,
                one_arg_2const_2zero, two_arg_2const_1literal_1zero,
                lookupswitch, tableswitch):
    for op_code, mnemonic in zip(op_type.op_codes, op_type.mnemonics):
        entry = Lookup_Entry()
        entry.check = op_type.check
        entry.size = op_type.size
        entry.to_bytes = op_type.to_bytes
        entry.mnemonic = mnemonic
        op_code_lookup[op_code] = entry
        mnemonic_lookup[mnemonic] = op_code
