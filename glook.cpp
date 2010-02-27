//
// Part of Roadkill Project. Check http://<urlhere> for latest version.
//
// Copyright 2010, Stanislav Karchebnyy <berkus@exquance.com>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#include <GL/glut.h>
#include <cstdlib>
#include <climits>
#include <cstdio>
#include <algorithm>
#include <ctype.h>
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
static int startTime;
static int prevTime;

static animated_parameter_t<GLfloat>
    xrot(15.0f, XROTRATE, -15.0f, 15.0f, animated_parameter_t<GLfloat>::PingPongLoop),
    yrot(0.0, YROTRATE);

static void render()        /* function called whenever redisplay needed */
{
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

    glutSwapBuffers();
    glFlush();
}

static void key(unsigned char key, int /*x*/, int /*y*/) /* called on key press */
{
    if (key == 'q') exit(0);
    if (key == 27) exit(0);
    // Force a redraw of the screen in order to update the display
    glutPostRedisplay();
}

static void animate(int /*value*/)
{
    // Set up the next timer tick (do this first)
    glutTimerFunc(TIMERMSECS, animate, 0);

    // Measure the elapsed time
    int currTime = glutGet(GLUT_ELAPSED_TIME);
    int timeSincePrevFrame = currTime - prevTime;
//     int elapsedTime = currTime - startTime;

    // Rotate the model
    yrot.animate(timeSincePrevFrame);
    xrot.animate(timeSincePrevFrame);

    // Force a redisplay to render the new image
    glutPostRedisplay();

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
    int win;

    if (argc < 2)
    {
        printf("\n\n ERROR!!! File name required\n\n");
        return 1;
    }

    glutInit(&argc, argv);        /* initialize GLUT system */
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);
    glutInitWindowSize(WIDTH, HEIGHT);
    win = glutCreateWindow("Glook");   /* create window */
    /* from this point on the current window is win */

    init(WIDTH, HEIGHT);

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

    glutDisplayFunc(render);
    glutKeyboardFunc(key);
    glutReshapeFunc(reshape);
    glutPostRedisplay();

    // Start the timer
    glutTimerFunc(TIMERMSECS, animate, 0);

    // Initialize the time variables
    startTime = glutGet(GLUT_ELAPSED_TIME);
    prevTime = startTime;

    glutMainLoop();           /* start processing events... */

    /* execution never reaches this point */
    return 0;
}
