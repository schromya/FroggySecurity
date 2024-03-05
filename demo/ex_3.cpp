/*  Example 3: Memory Management

    Compile ->  g++ ex_3.cpp
    Run ->      ./a.out
*/

#include <iostream>
#include <string>

int main()
{
    {
        std::string* string_1 = new std::string("Stay Froggy");  // In scope
    }

    // Out of scope but not deleted!
}