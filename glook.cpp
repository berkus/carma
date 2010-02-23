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
#include <cstring>
#include <climits>
#include <cstdio>
#include <math.h>
#include "blocks.h"
#include "texturizer.h"
#include <algorithm>
#include <ctype.h>
#include "math/matrix.h"

#define WIDTH 800
#define HEIGHT 600

#define BASE_DIR "DecodedData/DATA/"

// The time in milliseconds between timer ticks
#define TIMERMSECS 33

// rotation rate in degrees per second
#define ROTRATE 45.0f

using namespace std;
using namespace raii_wrapper;

static GLfloat LightPos[4]={-5.0f,5.0f,10.0f,0.0f};
static GLfloat Ambient[4]={0.5f,0.5f,0.5f,1.0f};

static mesh_t mesh;
static texture_renderer_t texturizer;

// Global variables for measuring time (in milli-seconds)
static int startTime;
static int prevTime;

static GLfloat rot = 0.0f;

// Calculate normal from vertices in counter-clockwise order.
template <typename type_t>
static vector_t<type_t> calc_normal(vector_t<type_t> v1, vector_t<type_t> v2, vector_t<type_t> v3)
{
    return normalize((v1 - v2) & (v2 - v3));
}

void mesh_t::calc_normals()
{
    normals.clear();
    for (size_t n = 0; n < mesh.vertices.size(); n++)
    {
        normals.push_back(vector_t<float>());
    }

    for (size_t n = 0; n < mesh.faces.size(); n++)
    {
        vector_t<float> normal = calc_normal(mesh.vertices[mesh.faces[n].v1], mesh.vertices[mesh.faces[n].v2], mesh.vertices[mesh.faces[n].v3]);
        normals[mesh.faces[n].v1] = normals[mesh.faces[n].v2] = normals[mesh.faces[n].v3] = normal;
    }
}

static void render()        /* function called whenever redisplay needed */
{
    glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);     /* clear the display */
//     glEnable(GL_TEXTURE_2D);

    glLoadIdentity();
    glTranslatef(0.0f, 0.0f, -3.0f);
    glRotatef(45.0f, 1.0f, 0.0f, 0.0f);
    glRotatef(rot, 0.0f, 1.0f, 0.0f);

//     texturizer.draw_texture("SCWHEEL.PIX");
    glBegin(GL_TRIANGLES);
    // Draw all faces of the mesh
    for (size_t n = 0; n < mesh.faces.size(); n++)
    {
//         texturizer.set_texture(mesh.materials[mesh.faces[n].material_id - 1]);

        glNormal3f(mesh.normals[mesh.faces[n].v1].x, mesh.normals[mesh.faces[n].v1].y, mesh.normals[mesh.faces[n].v1].z);
        glTexCoord2f(mesh.uvcoords[mesh.faces[n].v1].u, mesh.uvcoords[mesh.faces[n].v1].v);
        glVertex3f(mesh.vertices[mesh.faces[n].v1].x, mesh.vertices[mesh.faces[n].v1].y, mesh.vertices[mesh.faces[n].v1].z);

        glNormal3f(mesh.normals[mesh.faces[n].v2].x, mesh.normals[mesh.faces[n].v2].y, mesh.normals[mesh.faces[n].v2].z);
        glTexCoord2f(mesh.uvcoords[mesh.faces[n].v2].u, mesh.uvcoords[mesh.faces[n].v2].v);
        glVertex3f(mesh.vertices[mesh.faces[n].v2].x, mesh.vertices[mesh.faces[n].v2].y, mesh.vertices[mesh.faces[n].v2].z);

        glNormal3f(mesh.normals[mesh.faces[n].v3].x, mesh.normals[mesh.faces[n].v3].y, mesh.normals[mesh.faces[n].v3].z);
        glTexCoord2f(mesh.uvcoords[mesh.faces[n].v3].u, mesh.uvcoords[mesh.faces[n].v3].v);
        glVertex3f(mesh.vertices[mesh.faces[n].v3].x, mesh.vertices[mesh.faces[n].v3].y, mesh.vertices[mesh.faces[n].v3].z);
    }
    glEnd();

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

// TODO: actual file contains multiple meshes combined using actor spec.
static bool load_mesh(const char* fname)
{
    file f(fname, ios::in|ios::binary);
    CHECK_READ(resource_file_t::read_file_header(f));
    return mesh.read(f);
}

/*!
 * Creates a pathname to file fname at new location newpath and optionally changing extension to newext.
 * New pathname is created using strdup() and must be freed by client.
 */
static char* pathsubst(const char* fname, const char* newpath, const char* newext)
{
    if (!fname || !newpath)
        return 0;

    char pathbuf[PATH_MAX];
    strncpy(pathbuf, newpath, PATH_MAX);
    if (strchr(fname, '/'))
        strncat(pathbuf, strrchr(fname, '/') + 1, PATH_MAX - 1 - strlen(pathbuf));
    else
        strncat(pathbuf, fname, 256 - 1 - strlen(pathbuf));

    if (newext)
    {
        if (strchr(pathbuf, '.'))
        {
            int used = strrchr(pathbuf, '.') - pathbuf;
            strncpy(strrchr(pathbuf, '.'), newext, PATH_MAX - 1 - used);
        }
        else
            strncat(pathbuf, newext, PATH_MAX - 1 - strlen(pathbuf));
    }

    return strdup(pathbuf);
}

static bool load_textures(const char* fname, mesh_t& mesh)
{
    char* matfile = pathsubst(fname, BASE_DIR"MATERIAL/", ".MAT");
    char* curdir = getcwd(NULL, 0);

    printf("Opening %s, from %s\n", matfile, curdir);
    free(curdir);

    // Load materials from MAT file.
    file f(matfile, ios::in|ios::binary);
    free(matfile);
    CHECK_READ(resource_file_t::read_file_header(f));

    material_t mat;
    std::map<std::string, material_t> materials;

    while (mat.read(f))
        materials[mat.name] = mat;
    f.close();

    // Load pixmaps from PIX file.
    char* pixfile = pathsubst(fname, BASE_DIR"PIXELMAP/", ".PIX");
    file pix(pixfile, ios::in|ios::binary);
    free(pixfile);
    CHECK_READ(texturizer.read(pix));
    pix.close();

    for (size_t i = 0; i < mesh.materials.size(); i++)
    {
        printf("Loading material %s...", mesh.materials[i].c_str());
        if (materials.find(mesh.materials[i]) != materials.end())
        {
            std::string& pixmap = materials[mesh.materials[i]].pixelmap_name;
//             std::transform(pixmap.begin(), pixmap.end(), pixmap.begin(), toupper);
            printf("FOUND, binding texture %s\n", pixmap.c_str());
            if (!texturizer.set_texture(pixmap))
            {
                printf("Couldn't generate OpenGL texture!\n");
                return false;
            }
        }
        else
            printf("NOT FOUND\n");
    }

    return true;
}

static void animate(int /*value*/)
{
    // Set up the next timer tick (do this first)
    glutTimerFunc(TIMERMSECS, animate, 0);

    // Measure the elapsed time
    int currTime = glutGet(GLUT_ELAPSED_TIME);
//     int timeSincePrevFrame = currTime - prevTime;
    int elapsedTime = currTime - startTime;

    // Rotate the model
    rot = (ROTRATE / 1000) * elapsedTime;

    // Force a redisplay to render the new image
    glutPostRedisplay();

    prevTime = currTime;
}

// Respond to a window resize event
static void reshape(GLsizei w, GLsizei h)
{
    GLfloat fovy = 45.0f;
    GLfloat znear = 0.1f;
    GLfloat zfar = 100.0f;

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

    printf("Viewport: (%f,%f)-(%f,%f)\n", xmin,ymin, xmax,ymax);

    glMatrixMode(GL_MODELVIEW);
}

static void init(GLsizei w, GLsizei h)
{
    // Set up the OpenGL state
    glClearColor(0.0, 0.0, 0.0, 0.0);    /* set background to black */

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
        if (!load_mesh(argv[1]))
        {
            printf("Mesh load failed!\n");
            return 1;
        }

        if (!load_textures(argv[1], mesh))
        {
            printf("Textures load failed!\n");
            return 1;
        }
    }
    catch(file_error& e)
    {
        printf("File error: %s, aborting.\n", e.message());
        return 1;
    }

    printf("Files loaded.\n");
    mesh.calc_normals();

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
