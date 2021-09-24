#include <iostream>
#include <string>
#include <filesystem>
#include <cmath>
#include <sstream>
#include <GLFW/glfw3.h>
#include <nlohmann/json.hpp>
#include "master.h"

int main(int argc, char* argv[])
{
    std::filesystem::path filepath = std::filesystem::current_path() / "C:WorldData";

    std::string file;

    std::string defaultfile = "Sol.json";
    /*
    if(!glfwInit())
    {
        return -1;
    }
    */

    //std::cout << argc << "\n";
    //std::cout << argv[0] << "\n";


    if (argc > 1) {

        file = argv[1];

    }
    else
    {

        // load the Sol system by default if no arguments are provided.
        file = defaultfile;

    }
    Master_sys Master;
    Master.Init(filepath.string().data(), file);



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