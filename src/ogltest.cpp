//
// Test OpenGL 4.1 with glm and glfw
// Use this to try out new shaders.
//
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <OpenGL/gl.h>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <iostream>
#include <fstream>
#include <sstream>
#include "shader.hpp"

using namespace std;

std::string read_content(std::istream& is)
{
    std::stringstream s;
    s << is.rdbuf();
    return s.str();
}

static void error_callback(int error, const char* description)
{
    std::cerr << "Error " << error << ":" << description << std::endl;
}

int main()
{
    glfwSetErrorCallback(error_callback);

    if (!glfwInit())
        return 1;

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
#if __APPLE__
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
#endif

    GLFWwindow* window = glfwCreateWindow(800, 600, "oglTest", NULL, NULL);
    if (!window)
    {
        glfwTerminate();
        return 1;
    }
    glfwMakeContextCurrent(window);

    GLenum err = glewInit();
    if (GLEW_OK != err)
    {
        std::cerr << "Error " << glewGetErrorString(err) << std::endl;
    }
    std::cout << "Status: Using GLEW " << glewGetString(GLEW_VERSION);

    int width, height;
    glfwGetFramebufferSize(window, &width, &height);
    glfwSwapInterval(1);

    ShaderProgram rendering_program;
    rendering_program.source(GL_VERTEX_SHADER, "../shaders/first.vert");
    rendering_program.source(GL_FRAGMENT_SHADER, "../shaders/first.frag");
    rendering_program.link();

    GLuint vertex_array_object;
    glCreateVertexArrays(1, &vertex_array_object);
    glBindVertexArray(vertex_array_object);

    while (!glfwWindowShouldClose(window))
    {
        /* Render here */
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);     /* clear the display */

        glLoadIdentity();
        glTranslatef(0.0f, 0.0f, -3.0f);

        /* Swap front and back buffers */
        glfwSwapBuffers(window);

        /* Poll for and process events */
        glfwPollEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
}
