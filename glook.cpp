#include <GL/glut.h>
#include <cstdlib>
#include <cstring>
#include <cstdio>
#include <math.h>
#include "blocks.h"
#include "texturizer.h"

#define WIDTH 800
#define HEIGHT 600

// The time in milliseconds between timer ticks
#define TIMERMSECS 33

// rotation rate in degrees per second
#define ROTRATE 45.0f

using namespace std;
using namespace raii_wrapper;

#define NCOLORS 7
static GLfloat colors[][3] = { {1.0f, 1.0f, 1.0f}, {1.0f, 1.0f, 0.0f}, {1.0f, 0.0f, 1.0f}, {0.0f, 1.0f, 1.0f},
                        {1.0f, 0.0f, 0.0f}, {0.0f, 1.0f, 0.0f}, {0.0f, 0.0f, 1.0f} };

static GLfloat LightPos[4]={-5.0f,5.0f,10.0f,0.0f};
static GLfloat Ambient[4]={0.5f,0.5f,0.5f,1.0f};

static mesh_t mesh;
static texture_renderer_t texturizer;

// Global variables for measuring time (in milli-seconds)
static int startTime;
static int prevTime;

static GLfloat rot = 0.0f;

// Calculate normal from vertices in counter-clockwise order.
static vertex_t calc_normal(vertex_t v1, vertex_t v2, vertex_t v3)
{
    double v1x,v1y,v1z,v2x,v2y,v2z;
    double nx,ny,nz;
    double vLen;

    vertex_t Result;

    // Calculate vectors
    v1x = v1.x - v2.x;
    v1y = v1.y - v2.y;
    v1z = v1.z - v2.z;

    v2x = v2.x - v3.x;
    v2y = v2.y - v3.y;
    v2z = v2.z - v3.z;

    // Get cross product of vectors
    nx = (v1y * v2z) - (v1z * v2y);
    ny = (v1z * v2x) - (v1x * v2z);
    nz = (v1x * v2y) - (v1y * v2x);

    // Normalise final vector
    vLen = sqrt( (nx * nx) + (ny * ny) + (nz * nz) );

    Result.x = (float)(nx / vLen);
    Result.y = (float)(ny / vLen);
    Result.z = (float)(nz / vLen);

    return Result;
}

static void render()        /* function called whenever redisplay needed */
{
    glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);     /* clear the display */

    glLoadIdentity();
    glTranslatef(0.0f, 0.0f, -3.0f);
    glRotatef(45.0f, 1.0f, 0.0f, 0.0f);
    glRotatef(rot, 0.0f, 1.0f, 0.0f);

    glBegin(GL_TRIANGLES);
    // Draw all faces of the mesh
    for (size_t n = 0; n < mesh.faces.size(); n++)
    {
        vertex_t normal = calc_normal(mesh.vertices[mesh.faces[n].v1], mesh.vertices[mesh.faces[n].v2], mesh.vertices[mesh.faces[n].v3]);
        glNormal3f(normal.x, normal.y, normal.z);

        glColor3fv(colors[(mesh.faces[n].material_id - 1) % NCOLORS]);// to be replaced with textures
//         glTexCoord2f(uv);
        glVertex3f(mesh.vertices[mesh.faces[n].v1].x, mesh.vertices[mesh.faces[n].v1].y, mesh.vertices[mesh.faces[n].v1].z);
//         glTexCoord2f(uv);
        glVertex3f(mesh.vertices[mesh.faces[n].v2].x, mesh.vertices[mesh.faces[n].v2].y, mesh.vertices[mesh.faces[n].v2].z);
//         glTexCoord2f(uv);
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

static bool load_textures(const char* fname, mesh_t& mesh)
{
    // Pathname fidgeting...
    char pathbuf[256];
    strncpy(pathbuf, "DecodedData/DATA/MATERIAL/", 256);
    if (strchr(fname, '/'))
        strncat(pathbuf, strrchr(fname, '/') + 1, 256 - 1 - strlen(pathbuf));
    else
        strncat(pathbuf, fname, 256 - 1 - strlen(pathbuf));
    pathbuf[strlen(pathbuf) - 3] = 'M'; // change DAT to MAT (FIXME a hack)

    printf("Opening %s, from %s\n", pathbuf, get_current_dir_name()); //FIXME: leaks

    // Load materials from MAT file.
    file f(pathbuf, ios::in|ios::binary);
    CHECK_READ(resource_file_t::read_file_header(f));

    material_t mat;

    while (mat.read(f))
        mat.dump();

    for (size_t i = 0; i < mesh.materials.size(); i++)
    {
//         printf("Loading material %s...\n", mesh.materials[i].c_str());
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

    glutInit(&argc, argv);        /* initialize GLUT system */

    if (argc < 2)       //Was a file name specified?
    {
        printf("\n\n ERROR!!!   ");
        puts("File name required\n");
        return 1;
    }

    try {
        if (!load_mesh(argv[1]))
        {
            printf("Mesh load failed!\n");
            return 1;
        }
        mesh.dump();

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

    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);
    glutInitWindowSize(WIDTH, HEIGHT);
    win = glutCreateWindow("Glook");   /* create window */
    /* from this point on the current window is win */

    init(WIDTH, HEIGHT);

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
