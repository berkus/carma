//
// Part of Roadkill Project. Check http://<urlhere> for latest version.
//
// Copyright 2010, Stanislav Karchebnyy <berkus@exquance.com>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#include <GLFW/glfw3.h>
#include <cstdlib>
#include <climits>
#include <cstdio>
#include <algorithm>
#include <ctype.h>
#include <thread>
#include "math/matrix.h"
#include "blocks.h"
#include "texturizer.h"
#include "viewport.h"
#include "animated_parameter.h"
#include "loader.h"

#define WIDTH 800
#define HEIGHT 600

// The time in milliseconds between timer ticks
#define TIMERMSECS 33

// rotation rate in degrees per second
#define YROTRATE 45.0f
#define XROTRATE 15.0f

using namespace std;
using namespace raii_wrapper;

static GLfloat LightPos[4]={-5.0f,5.0f,10.0f,0.0f};
static GLfloat Ambient[4]={0.5f,0.5f,0.5f,1.0f};

static viewport_t viewport;
// Pesky globals
model_t model;
texture_renderer_t texturizer;

// Global variables for measuring time (in milli-seconds)
static double startTime;
static double prevTime;

static animated_parameter_t<GLfloat>
    xrot(15.0f, XROTRATE, -15.0f, 15.0f, animated_parameter_t<GLfloat>::PingPongLoop),
    yrot(0.0, YROTRATE),
    fov(45.0f, 30.0f, 45.f, 90.f, animated_parameter_t<GLfloat>::PingPongLoop);

static void error_callback(int error, const char* description)
{
    std::cerr << "Error " << error << ":" << description << std::endl;
}

static void animate(int /*value*/)
{
    std::this_thread::sleep_for(std::chrono::milliseconds(TIMERMSECS));

    // Set up the next timer tick (do this first)
    // glutTimerFunc(TIMERMSECS, animate, 0);

    // Measure the elapsed time
    double currTime = glfwGetTime();
    double timeSincePrevFrame = currTime - prevTime;

    std::cout << "Elapsed time " << (timeSincePrevFrame*1000) << std::endl;

    // Rotate the model
    yrot.animate(timeSincePrevFrame*1000);
    xrot.animate(timeSincePrevFrame*1000);
    fov.animate(timeSincePrevFrame*1000);

    // Force a redisplay to render the new image
    // glutPostRedisplay();

    prevTime = currTime;
}

// Respond to a window resize event
static void reshape(GLsizei w, GLsizei h)
{
    viewport.reshape(w, h);
}

static void init(GLsizei w, GLsizei h)
{
    // Set up the OpenGL state
    glClearColor(0.4, 0.4, 0.4, 0.0);    /* set background */

    glEnable(GL_LIGHTING);
    glEnable(GL_LIGHT0);

    glLightfv(GL_LIGHT0, GL_POSITION, LightPos);
    glLightfv(GL_LIGHT0, GL_AMBIENT,  Ambient);

    glShadeModel(GL_SMOOTH); // enable Gouraud

    glEnable(GL_CULL_FACE);
    glEnable(GL_DEPTH_TEST);
    glDepthMask(GL_TRUE);

    reshape(w, h);

    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        printf("\n\n ERROR!!! File name required\n\n");
        return 1;
    }

    glfwSetErrorCallback(error_callback);

    if (!glfwInit())
        return 1;

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 2);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 0);

    GLFWwindow* window = glfwCreateWindow(WIDTH, HEIGHT, "Glook", NULL, NULL);
    if (!window)
    {
        glfwTerminate();
        return 1;
    }

    // glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);

    glfwMakeContextCurrent(window);

    int width, height;
    glfwGetFramebufferSize(window, &width, &height);

    init(width, height);
    glfwSwapInterval(1);

    try {
        if (!load_actor(argv[1]))
        {
            printf("Actor load failed!\n");
            return 1;
        }
    }
    catch(file_error& e)
    {
        printf("File error: %s, aborting.\n", e.message());
        return 1;
    }

    printf("Files loaded.\n");

    // Initialize the time variables
    startTime = glfwGetTime();
    prevTime = startTime;

    while (!glfwWindowShouldClose(window))
    {
        /* Render here */
        animate(0);

        viewport.set_fov(fov.value());

        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);     /* clear the display */

        glLoadIdentity();
        glTranslatef(0.0f, 0.0f, -3.0f);
        glRotatef(xrot.value(), 1.0f, 0.0f, 0.0f);
        glRotatef(yrot.value(), 0.0f, 1.0f, 0.0f);

        for(std::map<std::string, actor_t*>::iterator it = model.parts.begin(); it != model.parts.end(); ++it)
        {
            actor_t* actor = (*it).second;
            if (actor->visible)
            {
                glPushMatrix();
                glTranslatef(actor->translate.x, actor->translate.y, actor->translate.z);
                printf("Rendering actor %s.\n", (*it).first.c_str());
                model.meshes[actor->mesh_name]->render();
                glPopMatrix();
            }
        }

        /* Swap front and back buffers */
        glfwSwapBuffers(window);

        /* Poll for and process events */
        glfwPollEvents();
    }

    glfwDestroyWindow(window);

    glfwTerminate();
}
