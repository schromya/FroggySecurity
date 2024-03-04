#include <iostream>
#include <string>

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
                screen[x + y * screenWidth] = image.values[x + y * image.width];
            }
        }
    }
}

void drawScreen() {

    for (int y = 0; y < screenHeight; y++) {
        for (int x = 0; x < screenWidth; x++) {
            std::cout << screen[x + y * screenHeight];
        }
        std::cout << "\n";
    }
    std::cout << std::flush;
}


int main()
{
    sprite frogsit;
    frogsit.width = 20;
    frogsit.height = 4;
    frogsit.values = R"(    (•)___(•)        / \ / \ / \        | \         / |        _ | --------- | _)";

    copySprite(frogsit, 10, 10);
    drawScreen();
}
