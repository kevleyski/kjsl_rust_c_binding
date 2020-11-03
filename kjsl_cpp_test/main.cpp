#include <iostream>

extern "C" int kev();

int main() {
    std::cout << "kev=" << kev() << std::endl;
    return 0;
}
