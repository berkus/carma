#include <GL/glut.h>
#include <cmath>
#include <cstdio>
#include "viewport.h"

#define EPSILON 0.0001

void viewport_t::reshape(GLsizei new_w, GLsizei new_h)
{
    if (new_w == w && new_h == h)
        return;

    w = new_w;
    h = new_h;

    set_viewport();
}

void viewport_t::set_fov(GLfloat new_fovy)
{
    if (fabs(fovy - new_fovy) < EPSILON)
        return;

    fovy = new_fovy;

    set_viewport();
}

void viewport_t::set_viewport()
{
    glViewport(0, 0, w, h);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();

    GLfloat aspect = (GLfloat)w/(GLfloat)h;

    gluPerspective(fovy, aspect, znear, zfar);   /* how object is mapped to window */

    GLfloat xmin, xmax, ymin, ymax;
    ymax = znear * tan(fovy * M_PI / 360.0);
    ymin = -ymax;
    xmin = ymin * aspect;
    xmax = ymax * aspect;

    printf("Viewport: (%f,%f)-(%f,%f), fovy %f, aspect %f\n", xmin,ymin, xmax,ymax, fovy, aspect);

    glMatrixMode(GL_MODELVIEW);
}
