// physics_sim.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <filesystem>
#include <cmath>
#include <sstream>
#include "const.h"
#include <GLFW/glfw3.h>
#include <nlohmann/json.hpp>

int main(int argc, char* argv[])
{
    std::filesystem::path filepath = std::filesystem::current_path() / "C:WorldData";
    //std::array<char, 256> file;
    char file[256] = " ";
    std::string defaultfile = "Sol.json";
    /*
    if(!glfwInit())
    {
        return -1;
    }
    */

    std::cout << argc << "\n";
    std::cout << argv[0] << "\n";

    if (argc > 1) {
        
        strcpy_s(argv[1], 256, file);
       
    }
    else
    {

        // load the Sol system by default if no arguments are provided.
        strcpy_s(defaultfile.data(), 256, file);
        //file = "Sol.json";
        
    }
    std::cout << filepath;
    // Create the Master Subsystem
    //master_s


    /*
    GLFWwindow* window = glfwCreateWindow(640, 480, "My Title", NULL, NULL);
    if (!window)
    {
        glfwTerminate();
        return -1;
    }
    glfwMakeContextCurrent(window);

    while (!glfwWindowShouldClose(window))
    {
                
        glClear(GL_COLOR_BUFFER_BIT);

        
        glfwSwapBuffers(window);

        
        glfwPollEvents();
    }
    
    std::cout << window;
    glfwTerminate();
    */
    return 0;

}

// Run program: Ctrl + F5 or Debug > Start Without Debugging menu
// Debug program: F5 or Debug > Start Debugging menu


// Tips for Getting Started: 
//   1. Use the Solution Explorer window to add/manage files
//   2. Use the Team Explorer window to connect to source control
//   3. Use the Output window to see build output and other messages
//   4. Use the Error List window to view errors
//   5. Go to Project > Add New Item to create new code files, or Project > Add Existing Item to add existing code files to the project
//   6. In the future, to open this project again, go to File > Open > Project and select the .sln file
