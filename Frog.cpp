#include <iostream>
#include <string>
#include <stdlib.h>

class Frog
{

public:
    Frog();
        void init();
    void meet();
    void feed();
    void chat();
};

Frog::Frog()
{
}

void Frog::init()
{
        std::cout << "Welcome to Frog World!" << std::endl;
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

int checkAuth(char uname[], char pwd[]){
        if(strcmp(uname, "admin") == 0 && strcmp(pwd, "password")){
                return 1;
        }
        return 0;
}

int frogInit(Frog& ourFrog){
        ourFrog.init();
        return 1;
}

int main()
{
    Frog myFrog;

    int authenticated = 0;
    char username[10];
    char password[64];

    //std::cout << "Please Enter Your Username : ";
    //std::cin >> username;

    std::cout << "Please Enter Your Password : ";
    std::cin >> password;

    if (checkAuth(username, password)) {
        frogInit(myFrog);
    } else {
        std::cout << "Wrong Username or Password\n";
    }
    return 0;
}

