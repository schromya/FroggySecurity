/*  Example 2: Ownership

    Compile ->  g++ ex_2.cpp
    Run ->      ./a.out
*/

#include <iostream>
#include <string>

int main()
{
    std::string string_1 = "Stay Froggy!";
    std::string string_2 = string_1;

    std::cout << string_1 << std::endl;
    std::cout << string_2 << std::endl;

}