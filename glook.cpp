#include <GL/glut.h>
#include <cstdlib>
#include <cstdio>
#include <math.h>
#include "blocks.h"

using namespace std;
using raii_wrapper::file;

#define NCOLORS 7
GLfloat colors[][3] = { {1.0f, 1.0f, 1.0f}, {1.0f, 1.0f, 0.0f}, {1.0f, 0.0f, 1.0f}, {0.0f, 1.0f, 1.0f},
                        {1.0f, 0.0f, 0.0f}, {0.0f, 1.0f, 0.0f}, {0.0f, 0.0f, 1.0f} };

mesh_t mesh;

void displayCB(void)        /* function called whenever redisplay needed */
{
    glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);     /* clear the display */

    // Draw all faces of the mesh
    for (size_t n = 0; n < mesh.faces.size(); n++)
    {
        glBegin(GL_TRIANGLES);
            glColor3fv(colors[(mesh.faces[n].material_id - 1) % NCOLORS]);
            glVertex3f(mesh.vertices[mesh.faces[n].v1].x, mesh.vertices[mesh.faces[n].v1].y, mesh.vertices[mesh.faces[n].v1].z);
            glVertex3f(mesh.vertices[mesh.faces[n].v2].x, mesh.vertices[mesh.faces[n].v2].y, mesh.vertices[mesh.faces[n].v2].z);
            glVertex3f(mesh.vertices[mesh.faces[n].v3].x, mesh.vertices[mesh.faces[n].v3].y, mesh.vertices[mesh.faces[n].v3].z);
        glEnd();
    }
    glFlush();
}

void keyCB(unsigned char key, int /*x*/, int /*y*/) /* called on key press */
{
    if( key == 'q' ) exit(0);
}

bool load_mesh(const char* fname)
{
    size_t count;
    unsigned char file_header[16];   //      The file header
    file f(fname, ios::in|ios::binary);

    count = f.read(file_header, 16);        //Read 16 byte file header

    if(count < 16)
    {
        puts("\n\n ERROR!!!  File header truncated.\n");     //exit if file header short
        return false;
    }

    printf("File header Data: ");      //Print file header to the screen
    for(int loop=0;loop<16;loop++)
    {
        printf("%02hX ",(file_header[loop]));
    }
    puts("\nReading Chunks:");

    return mesh.read(f);
}

int main(int argc, char *argv[])
{
    GLfloat fovy = 45.0f;
    GLfloat znear = 1.0f;
    GLfloat zfar = 500.0f;
    int win;

    glutInit(&argc, argv);        /* initialize GLUT system */

    if (argc < 2)       //Was a file name specified?
    {
        printf("\n\n ERROR!!!   ");
        puts("File name required\n");
        return 1;
    }

    if (!load_mesh(argv[1]))
    {
        printf("Mesh load failed!\n");
        return 1;
    }

    glutInitDisplayMode(GLUT_RGB);
    glutInitWindowSize(800,600);      /* width, height */
    GLfloat aspect = 800.0f / 600.0f;
    win = glutCreateWindow("Glook");   /* create window */

    /* from this point on the current window is win */

    glClearColor(0.0,0.0,0.0,0.0);    /* set background to black */

    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    gluPerspective(fovy, aspect, znear, zfar);   /* how object is mapped to window */

    GLfloat xmin, xmax, ymin, ymax;
    ymax = znear * tan(fovy * M_PI / 360.0);
    ymin = -ymax;
    xmin = ymin * aspect;
    xmax = ymax * aspect;

    printf("Viewport: (%f,%f)-(%f,%f)\n", xmin,ymin, xmax,ymax);

    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
    glTranslatef(0.0f, 0.0f, -5.0f);

    glutDisplayFunc(displayCB);       /* set window's display callback */
    glutKeyboardFunc(keyCB);      /* set window's key callback */

    glutMainLoop();           /* start processing events... */

    /* execution never reaches this point */

    return 0;
}
