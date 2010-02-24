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
#define YROTRATE 45.0f
#define XROTRATE 15.0f

using namespace std;
using namespace raii_wrapper;

class viewport_t
{
public:
    GLfloat fovy;
    GLfloat znear;
    GLfloat zfar;
    GLsizei w, h;

    viewport_t() : fovy(45.0f), znear(0.1f), zfar(100.0f), w(0), h(0) {}

    void reshape(GLsizei new_w, GLsizei new_h)
    {
        if (new_w == w && new_h == h)
            return;

        w = new_w;
        h = new_h;

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
};

static GLfloat LightPos[4]={-5.0f,5.0f,10.0f,0.0f};
static GLfloat Ambient[4]={0.5f,0.5f,0.5f,1.0f};

static viewport_t viewport;
static mesh_t mesh;
static texture_renderer_t texturizer;

// Global variables for measuring time (in milli-seconds)
static int startTime;
static int prevTime;

static GLfloat xrot = 45.0f, xdir = -1.0f, yrot = 0.0f;

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

static void render_vertex(vector_t<float> vertex, vector_t<float> normal, uvcoord_t uv)
{
    glNormal3f(normal.x, normal.y, normal.z);
    glTexCoord2f(uv.u, uv.v);
    glVertex3f(vertex.x, vertex.y, vertex.z);
}

static void render()        /* function called whenever redisplay needed */
{
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);     /* clear the display */
    glEnable(GL_TEXTURE_2D);

    glLoadIdentity();
    glTranslatef(0.0f, 0.0f, -3.0f);
    glRotatef(xrot, 1.0f, 0.0f, 0.0f);
    glRotatef(yrot, 0.0f, 1.0f, 0.0f);

    glBegin(GL_TRIANGLES);
    // Draw all faces of the mesh
    for (size_t n = 0; n < mesh.faces.size(); n++)
    {
        texturizer.set_texture(mesh.materials[mesh.material_names[mesh.faces[n].material_id - 1]].pixelmap_name);

        render_vertex(mesh.vertices[mesh.faces[n].v1], mesh.normals[mesh.faces[n].v1], mesh.uvcoords[mesh.faces[n].v1]);
        render_vertex(mesh.vertices[mesh.faces[n].v2], mesh.normals[mesh.faces[n].v2], mesh.uvcoords[mesh.faces[n].v2]);
        render_vertex(mesh.vertices[mesh.faces[n].v3], mesh.normals[mesh.faces[n].v3], mesh.uvcoords[mesh.faces[n].v3]);
    }
    glEnd();
    glDisable(GL_TEXTURE_2D);

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
    // Load actor file.
    char* actfile = pathsubst(fname, BASE_DIR"ACTORS/", ".ACT");
    printf("Opening %s\n", actfile);
    file act(actfile, ios::in|ios::binary);
    free(actfile);
    CHECK_READ(resource_file_t::read_file_header(act));

    model_t model;
    if (!model.read(act))
        return false;
    model.dump();
    act.close();

    // Load materials from MAT file.
    char* matfile = pathsubst(fname, BASE_DIR"MATERIAL/", ".MAT");
    printf("Opening %s\n", matfile);
    file f(matfile, ios::in|ios::binary);
    free(matfile);
    CHECK_READ(resource_file_t::read_file_header(f));

    material_t mat;

    mesh.materials.clear();
    while (mat.read(f))
        mesh.materials[mat.name] = mat;
    f.close();

    // Load palette from PIX file.
    pixelmap_t palette;
    char* palfile = pathsubst("DRRENDER.PAL", BASE_DIR"REG/PALETTES/", NULL);
    printf("Opening %s\n", palfile);
    file pal(palfile, ios::in|ios::binary);
    free(palfile);
    CHECK_READ(resource_file_t::read_file_header(pal));
    CHECK_READ(palette.read(pal));
    pal.close();

    texturizer.set_palette(palette);

    // Load pixmaps from PIX file.
    char* pixfile = pathsubst(fname, BASE_DIR"PIXELMAP/", ".PIX");
    printf("Opening %s\n", pixfile);
    file pix(pixfile, ios::in|ios::binary);
    free(pixfile);
    CHECK_READ(texturizer.read(pix));
    pix.close();

    for (size_t i = 0; i < mesh.material_names.size(); i++)
    {
        printf("Loading material %s...", mesh.material_names[i].c_str());
        if (mesh.materials.find(mesh.material_names[i]) != mesh.materials.end())
        {
            std::string& pixmap = mesh.materials[mesh.material_names[i]].pixelmap_name;
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
    int timeSincePrevFrame = currTime - prevTime;
    int elapsedTime = currTime - startTime;

    // Rotate the model
    yrot = (YROTRATE / 1000) * elapsedTime;
    xrot += (XROTRATE / 1000) * timeSincePrevFrame * xdir;

    if (xrot > 45.0f)
    {
        xrot = 45.0;
        xdir = -1.0f;
    }
    if (xrot < -30.0f)
    {
        xrot = -30.0f;
        xdir = 1.0f;
    }

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
