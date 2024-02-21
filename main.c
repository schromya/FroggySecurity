#include <stdio.h>
#include <string.h>


typedef struct frog{

const char frogStanding[50];
const char frogJumping[50];
} Frog;



typedef struct obstacle{

} Obstacle;

typedef struct background{

} Background;

typedef struct world{
Frog* frog;
Obstacle* obstacles;
Background background;
} World;