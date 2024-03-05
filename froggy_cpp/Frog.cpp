#include <iostream>
#include <string>
#include <chrono>
#include <thread>
#include <Windows.h>
#include <random>


const int screenWidth = 120;
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
            if (image.values.size() > x + y * image.width && image.values[x + y * image.width] != ' ' && !(yPos + y < 0 || yPos + y >= screenHeight || xPos + x < 0 || xPos + x >= screenWidth)) {
                screen[x + xPos + (y+yPos) * screenWidth] = image.values[x + y * image.width];
            }
        }
    }
}

void resetScreen() {
    for (int y = 0; y < screenHeight; y++) {
        for (int x = 0; x < screenWidth; x++) {
            screen[x + y * screenWidth] = ' ';
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

void updateScore(sprite * image, int score) {
    image->values = R"(Score : )" + std::to_string(score);
}

int main()
{
    // Hide the Cursor
    HANDLE consoleHandle = GetStdHandle(STD_OUTPUT_HANDLE);
    CONSOLE_CURSOR_INFO info;
    info.dwSize = 100; // The size of the cursor
    info.bVisible = FALSE; // Set the cursor visibility to FALSE
    SetConsoleCursorInfo(consoleHandle, &info);

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
    std::getline(std::cin, name);

    /*
    if (name.size() < 10) {
        for (int i = 0; i < 10 - name.size(); i++) {
            name += " ";
        }
    } else if (name.size() > 10) {

    }
    */
    sprite nametag;
    nametag.width = 12;
    nametag.height = 3;
    nametag.values = R"(============|John Cena |============)";
    
    sprite score;
    int currentScore = 0;
    score.width = 12;
    score.height = 1;
    //score.values = R"(Score : 0)";

    int mushroomPos = 0;
    int frogPos = 0;
    bool frogHasWings = true;
    bool isTall = std::rand() % 2;
    while (true) {

        updateScore(&score, currentScore);
        if (mushroomPos > screenWidth + mushroomShort.width) {
            mushroomPos = 0;
            isTall = std::rand() % 2;
        }
        if (frogHasWings && frogPos > -10) {
            frogPos--;
        } else {
            if (frogPos < 0) {
                frogHasWings = false;
                frogPos++;
            } else {
                frogHasWings = true;
            }
        }

        resetScreen();
        if (frogHasWings) {
            copySprite(frogjump, 20, 16 + frogPos);
        } else {
            copySprite(frogsit, 20, 16 + frogPos);
        }
        if (isTall) {
            copySprite(mushroomTall , screenWidth - mushroomShort.width - mushroomPos, 15);
        } else {
            copySprite(mushroomShort, screenWidth - mushroomShort.width - mushroomPos, 16);
        }
        copySprite(nametag, 3, 1);
        copySprite(score, screenWidth - score.width - 5, 2);
        drawScreen();
        mushroomPos++;
        mushroomPos++;
        currentScore++;
        //std::this_thread::sleep_for(std::chrono::milliseconds(15));
    }
}
