// (c) anaya 2021
// ==============
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

// constants
// =========
#define ASSERTS_ENABLED false // disable this if
                              // you wanna disable
                              // the file check
                              // asserts.
#define WIDTH 640
#define HEIGHT 480
#define TITLE "something useless..."

// all vars are global...
// ======================
struct 
{
    struct
    {
        SDL_Window* window;
    } system;
    
    struct
    {
        SDL_Surface* atlas;
        TTF_Font* internal_font;
    } resources; // CPU RESOURCES ONLY!
    
    bool running;
} core;

// forward declarations
// ====================
void load_resources();
void unload_resources();
void start_program();
bool update_program();
void reset_program();
void exit_program(int code);

// main
// ====
int main(int argc, char* argv[])
{
    load_resources();
    start_program();
    
    while (update_program())
    {
        Uint8* keyboard = SDL_GetKeyboardState(NULL);
        if (keyboard[SDL_SCANCODE_ESCAPE]) core.running = false;

        
    }

    unload_resources();
    exit_program(0);
    return 0;
}

// IMPLEMENT FUNCTIONS
// ===================
void load_resources() // init some libraries, etc.
                      // you can't load gpu resources here
                      // bc there is no gpu context.
{
    core.running = false;

    SDL_Init(SDL_INIT_EVERYTHING);
    atexit(SDL_Quit);

    TTF_Init();
    atexit(TTF_Quit);

    #ifdef _WIN32 // load resources from .EXE :3
    // TBD
    #else // Unix
    core.resources.atlas = SDL_LoadBMP("atlas.bmp");
    core.resources.internal_font = TTF_OpenFont("dos.ttf");
    #endif

    // assert that mandatory resources have been loaded
    #if ASSERTS_ENABLED
    if (!core.resources.atlas)
    {
        printf("missing texture atlas!\n");
        exit_program(-1);
    }

    if (!core.resources.internal_font)
    {
        printf("missing font!\n");
        exit_program(-2);
    }
    #endif
}

void unload_resources() // you should be able to unload gpu resources
                        // (OS resources are not unloaded until the 
                        // program exits)
{
    TTF_CloseFont(core.resources.internal_font);
    SDL_FreeSurface(core.resources.atlas);
}

void start_program()
{
    // create window, etc.
    core.system.window = SDL_CreateWindow
    (
        TITLE,
        SDL_WINDOWPOS_UNDEFINED,
        SDL_WINDOWPOS_UNDEFINED,
        WIDTH,
        HEIGHT,
        SDL_WINDOW_SHOWN
    );

    core.running = true;
}

bool update_program() // polls events, etc.
{
    SDL_Event ev;
    while (SDL_PollEvent(&ev))
    {
        switch (ev.type)
        {
        case SDL_QUIT:
            core.running = false;
            break;
        }
    }

    return core.running;
}

void reset_program() // resets game state
{

}

void exit_program(int code)
{
    unload_resources();
    SDL_DestroyWindow(core.system.window);

    exit(code);
}

// EOF - anaya :3