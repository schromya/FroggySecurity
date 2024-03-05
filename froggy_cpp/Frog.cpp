#include <iostream>
#include <string>
#include <thread>
#include <conio.h>
#include <Windows.h>
#include <random>
#include <chrono>

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

    sprite frogdead;
    frogdead.width = 15;
    frogdead.height = 4;
    frogdead.values = R"(   (x)___(x)    /\ /     \ /\  | \       / | _| --------- |_)";

    sprite frogjump;
    frogjump.width = 15;
    frogjump.height = 6;
    frogjump.values = R"(   (•)___(•)     /         \   |\         /\  |  -------  |  |           |  /           \ )";

    sprite frogdeadjump;
    frogdeadjump.width = 15;
    frogdeadjump.height = 6;
    frogdeadjump.values = R"(   (x)___(x)     /         \   |\         /\  |  -------  |  |           |  /           \ )";

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
    system("cls");
    std::cout << "Starting in 3. . ." << std::flush;
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    std::cout << " 2. . ." << std::flush;
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    std::cout << " 1. . ." << std::flush;
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    
    if (name.size() < 10) {
        int temp = name.size();
        for (int i = 0; i < 10 - temp; i++) {
            name += " ";
        }
    }
    
    sprite nametag;
    nametag.width = 12;
    nametag.height = 3;
    nametag.values = "============|"+ name +"|============";
    
    sprite score;
    int currentScore = 0;
    score.width = 12;
    score.height = 1;

    sprite gameOver;
    gameOver.width = 19;
    gameOver.height = 5;
    gameOver.values = "===================|                 ||    GAME OVER    ||                 |===================";

    int mushroomPos = 0;
    int frogPos = 0;
    bool frogHasWings = false;
    bool isTall = std::rand() % 2;
    bool isAlive = true;
    while (true) {

        // Updated Score
        updateScore(&score, currentScore);
        if (mushroomPos > screenWidth + mushroomShort.width) {
            mushroomPos = 0;
            isTall = std::rand() % 2;
        }

        if (isAlive) {
            // Collision
            // X Axis Collision
            if (screenWidth - mushroomShort.width - mushroomPos <= 33 && screenWidth - mushroomPos >= 20) {
                // Y Axis Collision
                if (16 + frogPos + 4 > 16) {
                    isAlive = false;
                }
            }

            // Frog Animation State
            if (frogHasWings && frogPos > -15) {
                frogPos--;
            } else {
                if (frogPos < 0) {
                    frogHasWings = false;
                    frogPos++;
                } else {
                    if (_kbhit()) {
                        frogHasWings = true;
                        _getch();
                    }
                }
            }
        }

        // Clear Screen
        resetScreen();

        // Draw Frog
        if (isAlive) {
            if (frogHasWings) {
                copySprite(frogjump, 20, 16 + frogPos);
            } else {
                copySprite(frogsit, 20, 16 + frogPos);
            }
        } else {
            if (frogHasWings) {
                copySprite(frogdeadjump, 20, 16 + frogPos);
            } else {
                copySprite(frogdead, 20, 16 + frogPos);
            }
        }
        // Draw Mushroom
        if (isTall) {
            copySprite(mushroomTall , screenWidth - mushroomShort.width - mushroomPos, 15);
        } else {
            copySprite(mushroomShort, screenWidth - mushroomShort.width - mushroomPos, 16);
        }
        // Draw UI
        copySprite(nametag, 3, 1);
        copySprite(score, screenWidth - score.width - 5, 2);


        if (isAlive) {
            mushroomPos++;
            mushroomPos++;
            currentScore++;
        } else {
            copySprite(gameOver, (screenWidth - gameOver.width) / 2, (screenHeight - gameOver.height) / 2);
        }

        // Update Screen
        drawScreen();
    }
}
