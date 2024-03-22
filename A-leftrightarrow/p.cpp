#include <vector>
#include <cassert>
#include <iostream>

void sayNo() {
    std::cout << "No" << std::endl;
    exit(0);
}

int main(){
    std::string S; std::cin >> S;
    if (S[0] != '<' or S.back() != '>') {
        sayNo();
    }
    for (size_t i = 1; i < S.size() - 1; i++) {
        if (S[i] != '=') { sayNo(); }
    }
    std::cout << "Yes" << std::endl;
    return 0;
}