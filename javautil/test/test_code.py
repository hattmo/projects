from javautil import OP

def test_code():
    code = OP.iconst_1() + OP.iconst_1() + OP.iadd() + OP.bipush(15)
    print()
    for i in code.to_bytes(): print(f"0x{i:02X}")