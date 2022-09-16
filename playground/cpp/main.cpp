#include <iostream>
#include <vector>

class MyClass
{
private:
    int foo;
    int bar;
    int baz;

public:
    MyClass();
    ~MyClass();
    void print();
};

MyClass::MyClass()
{
    this->foo = 7;
}

MyClass::~MyClass()
{
}

void MyClass::print()
{
    std::cout << "pointer size is: " << sizeof(this) << std::endl;
    std::cout << "deref size is: " << sizeof(*this) << std::endl;
}

int main(int argc, char **argv)
{
    MyClass foo = MyClass();
    foo.print();
}
