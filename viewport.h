#pragma once

#include <GL/gl.h>

class viewport_t
{
public:
    viewport_t() : fovy(45.0f), znear(0.1f), zfar(100.0f), w(0), h(0) {}

    void reshape(GLsizei new_w, GLsizei new_h);
    void set_fov(GLfloat new_fovy);

private:
    GLfloat fovy;
    GLfloat znear;
    GLfloat zfar;
    GLsizei w, h;

    void set_viewport();
};
