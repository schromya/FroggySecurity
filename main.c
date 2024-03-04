
// Function signature modified to accept 2D array correctly
#include <stdio.h>
#include <stdlib.h>
#include <conio.h> // For kbhit() and getch()
#include <windows.h>

#define WORLD_WIDTH 80
#define WORLD_HEIGHT 24

// Fixed ASCII art initialization with proper commas
char* ascii_frog[] = {
    " /\\  (.)___(.)  /\\ ",
    " | \\/         \\/ | ",
    "_|  \\         /  |_",
    "      -------      "
};

char* ascii_mushroom[] = {
    "   _______   ",
    " /       o \\ ",
    "|___  o  ___|",
    "    |___|    "
};

char world[WORLD_WIDTH][WORLD_HEIGHT];
char def = '.';
int frogX = 10; // Initial X position of the frog
int frogY = WORLD_HEIGHT - 5; // Initial Y position of the frog, placed at the bottom
int isJumping = 0; // Flag to indicate if the frog is jumping

void drawFrog(int x, int y) {
    for (int i = 0; i < 4; i++) { // Assuming your frog art is 4 lines high
        for (int j = 0; ascii_frog[i][j] != '\0'; j++) {
            if ((y + i) < WORLD_HEIGHT && (x + j) < WORLD_WIDTH) { // Bounds check
                world[y + i][x + j] = ascii_frog[i][j];
            }
        }
    }
}

void hideCursor() {
    HANDLE consoleHandle = GetStdHandle(STD_OUTPUT_HANDLE);
    CONSOLE_CURSOR_INFO info;
    info.dwSize = 100; // The size of the cursor
    info.bVisible = FALSE; // Set the cursor visibility to FALSE
    SetConsoleCursorInfo(consoleHandle, &info);
}

void clearWorld() {
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    COORD pos = {0, 0};
    SetConsoleCursorPosition(hConsole, pos);
    for (int y = 0; y < WORLD_HEIGHT; y++) {
        for (int x = 0; x < WORLD_WIDTH; x++) {
            world[y][x] = def;
        }
    }
}

void displayWorld() {
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    COORD pos = {0, 0};
    SetConsoleCursorPosition(hConsole, pos);
    for (int y = 0; y < WORLD_HEIGHT; y++) {
        for (int x = 0; x < WORLD_WIDTH; x++) {
            printf("%c", world[y][x]);
        }
        printf("\n");
    }
}

void updateFrogPosition() {
    if (isJumping) {
        frogY -= 3; // Move the frog up by 3 positions
    } else {
        if (frogY < WORLD_HEIGHT - 5) { // If not on the ground, fall down
            frogY += 3;
        }
    }
    if (frogY < 0) frogY = 0; // Prevent going above the world
    if (frogY > WORLD_HEIGHT - 5) frogY = WORLD_HEIGHT - 5; // Prevent going below the ground
    isJumping = 0; // Reset jumping state
}

int main() {
    hideCursor();
    while (1) {
        if (_kbhit()) {
            char ch = _getch();
            if (ch == ' ') { // Spacebar pressed
                isJumping = 1;
            }
        }
        clearWorld();
        updateFrogPosition();
        drawFrog(frogX, frogY);
        displayWorld();
    }
    return 0;
}
