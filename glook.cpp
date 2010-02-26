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
#include <sstream>
#include <math.h>
#include "blocks.h"
#include "texturizer.h"
#include <algorithm>
#include <ctype.h>
#include "math/matrix.h"
#include <sys/stat.h>
#include <errno.h>

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
static model_t model;
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
    for (size_t n = 0; n < vertices.size(); n++)
    {
        normals.push_back(vector_t<float>());
    }

    for (size_t n = 0; n < faces.size(); n++)
    {
        vector_t<float> normal = calc_normal(vertices[faces[n].v1], vertices[faces[n].v2], vertices[faces[n].v3]);
        normals[faces[n].v1] = normals[faces[n].v2] = normals[faces[n].v3] = normal;
    }
}

static void render_vertex(vector_t<float> vertex, vector_t<float> normal, uvcoord_t uv)
{
    glNormal3f(normal.x, normal.y, normal.z);
    glTexCoord2f(uv.u, uv.v);
    glVertex3f(vertex.x, vertex.y, vertex.z);
}

void mesh_t::render()
{
    glBegin(GL_TRIANGLES);
    // Draw all faces of the mesh
    int previous_texture = -1;
    for (size_t n = 0; n < faces.size(); n++)
    {
        if (material_names.size() > 0) // what happens to the meshes without materials? are they drawn?
        {
            if (previous_texture != faces[n].material_id)
            {
                string matname = material_names[faces[n].material_id - 1];
                string pixelmap = model.materials[matname].pixelmap_name; //FIXME: global model ref
                printf("Setting face material %s, texture %s.\n", matname.c_str(), pixelmap.c_str());
                glEnd();
                if (!texturizer.set_texture(pixelmap))
                {
                    printf("Ooops!");
                    return;
                }
                glBegin(GL_TRIANGLES);
                previous_texture = faces[n].material_id;
            }
        }

        render_vertex(vertices[faces[n].v1], normals[faces[n].v1], uvcoords[faces[n].v1]);
        render_vertex(vertices[faces[n].v2], normals[faces[n].v2], uvcoords[faces[n].v2]);
        render_vertex(vertices[faces[n].v3], normals[faces[n].v3], uvcoords[faces[n].v3]);
    }
    glEnd();
}

static void render()        /* function called whenever redisplay needed */
{
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);     /* clear the display */
    glEnable(GL_TEXTURE_2D);

    glLoadIdentity();
    glTranslatef(0.0f, 0.0f, -3.0f);
    glRotatef(xrot, 1.0f, 0.0f, 0.0f);
    glRotatef(yrot, 0.0f, 1.0f, 0.0f);

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

    // Strip stray newlines (they appear whilst reading TXT file).
    int end = strlen(pathbuf) - 1;
    while (end > 0 && (pathbuf[end] == '\r' || pathbuf[end] == '\n' || pathbuf[end] == '\t'))
    {
        pathbuf[end] = 0;
        end--;
    }

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

// from GameDev forum
template<typename RT, typename T, typename Trait, typename Alloc>
RT ss_atoi(const std::basic_string<T, Trait, Alloc>& the_string)
{
    std::basic_istringstream< T, Trait, Alloc> temp_ss(the_string);
    RT num;
    temp_ss >> num;
    return num;
}

static std::vector<std::string> read_txt_lines(file& f)
{
    std::vector<std::string> out;

    std::string count;
    if (!f.getline(count))
        return out;
//     printf("Got count string: %s\n", count.c_str());

    size_t num = ss_atoi<size_t>(count.substr(0, count.find_first_of(' ')));
//     printf("%d entries to read.\n", num);
    for (size_t i = 0; i < num; ++i)
    {
        if (!f.getline(count))
            return out;
//         printf("Got value string: %s\n", count.c_str());
        out.push_back(count);
    }
//     printf("Returning complete vector.\n");
    return out;
}

static bool load_actor(const char* fname)
{
    // Load description file.
    char* txtfile = pathsubst(fname, BASE_DIR"CARS/", ".ENC");
    printf("Opening car %s\n", txtfile);
    file txt(txtfile, ios::in|ios::binary);
    free(txtfile);
    // Parse only material/mesh mumbo-jumbo for now.
    std::vector<std::string> txt_lines, load_meshes, load_pixmaps, load_materials;
    std::string line;
//     bool parse_lines = false;
    while (txt.getline(line))
    {
        if (line.find(std::string(".PIX,G")) != line.npos)
        {
//             printf("After %s starting parsing lines.\n", line.c_str());
//             parse_lines = true;
//         }
//         if (parse_lines)
//         {
            // 3x pixelmap
            txt_lines = read_txt_lines(txt);
//             load_pixmaps.insert(load_pixmaps.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
            load_pixmaps.insert(load_pixmaps.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
//             load_pixmaps.insert(load_pixmaps.end(), txt_lines.begin(), txt_lines.end());
            // shadetable
            txt_lines = read_txt_lines(txt);
            // 3x material
            txt_lines = read_txt_lines(txt);
//             load_materials.insert(load_materials.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
            load_materials.insert(load_materials.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
//             load_materials.insert(load_materials.end(), txt_lines.begin(), txt_lines.end());
            // models
            txt_lines = read_txt_lines(txt);
            load_meshes.insert(load_meshes.end(), txt_lines.begin(), txt_lines.end());
            // actors
            txt_lines = read_txt_lines(txt);
//             parse_lines = false;
        }
//     "GROOVE" specifies actor placements and functions
    }
    txt.close();

    printf("%d meshes, %d pixmaps, %d materials to load.\n", load_meshes.size(), load_pixmaps.size(), load_materials.size());

    // Load actor file.
    char* actfile = pathsubst(fname, BASE_DIR"ACTORS/", ".ACT");
    printf("Opening actor %s\n", actfile);
    file act(actfile, ios::in|ios::binary);
    free(actfile);
    #define f act
    CHECK_READ(resource_file_t::read_file_header(act));
    #undef f

    if (!model.read(act))
        return false;
    act.close();

    // Now iterate all meshes and load them.
//     for (std::vector<std::string>::iterator it = load_meshes.begin(); it != load_meshes.end(); ++it)
//     {
//         if (model.meshes.find((*it)) != model.meshes.end())
//         {
//             printf("Model %s already loaded, skipping.\n", (*it).c_str());
//             continue;
//         }

        char* meshfile = pathsubst(fname/*(*it).c_str()*/, BASE_DIR"MODELS/", ".DAT");
        printf("Opening model '%s'\n", meshfile);
        file f(meshfile, ios::in|ios::binary);
        free(meshfile);
        CHECK_READ(resource_file_t::read_file_header(f));

        while (1)
        {
            mesh_t* mesh = new mesh_t;
            if (!mesh->read(f))
            {
                delete mesh;
                break;
            }
            if (model.meshes.find(mesh->name) != model.meshes.end())
            {
                printf("Model %s already loaded, skipping.\n", mesh->name.c_str());
                delete mesh;
                continue;
            }

            mesh->calc_normals();
            printf("Adding model %s to meshes.\n", mesh->name.c_str());
            model.meshes[mesh->name] = mesh;
        }
        f.close();
//     }

    // Load materials from MAT files.
    model.materials.clear();

    for (std::vector<std::string>::iterator it = load_materials.begin(); it != load_materials.end(); ++it)
    {
        char* matfile = pathsubst((*it).c_str(), BASE_DIR"MATERIAL/", NULL);
        printf("Opening material %s\n", matfile);
        file f(matfile, ios::in|ios::binary);
        free(matfile);
        CHECK_READ(resource_file_t::read_file_header(f));

        material_t mat;

        while (mat.read(f))
        {
            printf("Adding material %s to the list.\n", mat.name.c_str());
            model.materials[mat.name] = mat;
        }
        f.close();
    }

    // Load palette from PIX file.
    pixelmap_t palette;
    char* palfile = pathsubst("DRRENDER.PAL", BASE_DIR"REG/PALETTES/", NULL);
    printf("Opening palette %s\n", palfile);
    file pal(palfile, ios::in|ios::binary);
    free(palfile);
    #define f pal
    CHECK_READ(resource_file_t::read_file_header(pal));
    CHECK_READ(palette.read(pal));
    #undef f
    pal.close();

    texturizer.set_palette(palette);

    // Load textures from PIX files.
    for (std::vector<std::string>::iterator it = load_pixmaps.begin(); it != load_pixmaps.end(); ++it)
    {
        char* pixfile = pathsubst((*it).c_str(), BASE_DIR"PIXELMAP/", NULL);
        printf("Opening pixelmap %s\n", pixfile);
        file pix(pixfile, ios::in|ios::binary);
        free(pixfile);
        #define f pix
        CHECK_READ(texturizer.read(pix));
        #undef f
        pix.close();
    }

    // Bind textures for materials in model.
    texturizer.dump_cache_textures();

    for(std::map<std::string, mesh_t*>::iterator it = model.meshes.begin(); it != model.meshes.end(); ++it)
    {
        mesh_t* mesh = (*it).second;
        for (size_t i = 0; i < mesh->material_names.size(); ++i)
        {
            printf("Loading material %s...", mesh->material_names[i].c_str());
            if (model.materials.find(mesh->material_names[i]) != model.materials.end())
            {
                std::string pixmap = model.materials[mesh->material_names[i]].pixelmap_name;
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
