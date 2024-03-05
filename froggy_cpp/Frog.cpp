#include <iostream>
#include <string>
#include <chrono>
#include <thread>
#include <Windows.h>

const int screenWidth = 100;
const int screenHeight = 20;
unsigned int screenSize = screenWidth * screenHeight;
std::string screen(screenSize, '*');

struct sprite {
    unsigned int width;
    unsigned int height;
    std::string values;
};

void copySprite(sprite image, unsigned int xPos, unsigned int yPos) {
    for (int y = 0; y < image.height; y++) {
        for (int x = 0; x < image.width; x++) {
            if (!(yPos + y < 0 || yPos + y >= screenHeight || xPos + x < 0 || xPos + x >= screenWidth)) {
                screen[x + xPos + (y+yPos) * screenWidth] = image.values[x + y * image.width];
            }
        }
    }
}

void resetScreen() {
    for (int y = 0; y < screenHeight; y++) {
        for (int x = 0; x < screenWidth; x++) {
            screen[x + y * screenWidth] = '*';
        }
    }
}

void setCursorPosition(int x, int y) {
    static const HANDLE hOut = GetStdHandle(STD_OUTPUT_HANDLE);
    std::cout.flush();
    COORD coord = { (SHORT)x, (SHORT)y };
    SetConsoleCursorPosition(hOut, coord);
}

void drawScreen() {

    for (int y = 0; y < screenHeight; y++) {
        for (int x = 0; x < screenWidth; x++) {
            setCursorPosition(x, y);
            std::cout << screen[x + y * screenWidth];
        }
        //std::cout << "\n";
    }
    std::cout << std::flush;
}


int main()
{
    sprite frogsit;
    frogsit.width = 15;
    frogsit.height = 4;
    frogsit.values = R"(   (•)___(•)    /\ /     \ /\  | \       / | _| --------- |_)";

    sprite frogjump;
    frogjump.width = 15;
    frogjump.height = 6;
    frogjump.values = R"(   (•)___(•)     /         \   |\         /\  |  -------  |  |           |  /           \ )";

    sprite mushroomTall;
    mushroomTall.width = 13;
    mushroomTall.height = 5;
    mushroomTall.values = R"(   _______     /  o    \   |___ o ___|     |   |        |___|             )";

    sprite mushroomShort;
    mushroomShort.width = 13;
    mushroomShort.height = 4;
    mushroomShort.values = R"(   _______     /  o    \   |___ o ___|     |___|             )";

    std::string name;
    std::cout << "What is your name? Max 10 letters" << std::endl;
    //std::getline(std::cin, name);

    sprite nametag;
    nametag.width = 12;
    nametag.height = 3;
    nametag.values = R"(============|John Cena |============)";
    
    int mushroomPos = 0;
    while (true) {
        // Hide the Cursor
        HANDLE consoleHandle = GetStdHandle(STD_OUTPUT_HANDLE);
        CONSOLE_CURSOR_INFO info;
        info.dwSize = 100; // The size of the cursor
        info.bVisible = FALSE; // Set the cursor visibility to FALSE
        SetConsoleCursorInfo(consoleHandle, &info);

        resetScreen();
        copySprite(frogsit, 15, 16);
        copySprite(mushroomShort, 100 - mushroomShort.width - mushroomPos, 16);
        copySprite(nametag, 0, 0);
        drawScreen();
        mushroomPos++;
        mushroomPos++;
        //std::this_thread::sleep_for(std::chrono::milliseconds(15));
    }
}
