#include <iostream>
#include <string>

class Frog
{

public:
    Frog();
    void meet();
    void feed();
    void chat();
};

Frog::Frog()
{
    std::cout << "Welcome to Frog World!" << std::endl;

    // TODO give options
}

void Frog::meet()
{
}

void Frog::feed()
{
}

void Frog::chat()
{
}

int main()
{
    Frog myFrog;
    
    int authenticated = 0;
    char username[10];
    char password[10];

    std::cout << "Please Enter Your Username : ";
    std::cin >> username;

    std::cout << "Please Enter Your Password : ";
    std::cin >> password;

    if (std::strcmp(username, "admin") == 0 && std::strcmp(password, "password") == 0) {
        authenticated = 1;
    }
    if (authenticated) {
        std::cout << "You're In.\n";
    } else {
        std::cout << "Wrong Username or Password\n";
    }
    return 0;
}
