#include <iostream>

extern "C" int kev();

int main() {
    std::cout << "Hello, World!" << kev() << std::endl;
    return 0;
}
