#include <OpenGL/gl.h>
#include <string>
#include "blocks.h"
#include "texturizer.h"

using namespace std;

extern model_t model;
extern texture_renderer_t texturizer;

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
    if (material_names.size() > 0)
        glEnable(GL_TEXTURE_2D);
    else
        texturizer.reset_texture();

    glBegin(GL_TRIANGLES);
    // Draw all faces of the mesh
    int previous_texture = -1;
    for (size_t n = 0; n < faces.size(); n++)
    {
        if (material_names.size() > 0) // what happens to the meshes without materials? are they drawn?
        {
            if (previous_texture != faces[n].material_id)
            {
                glEnd();
                string matname = material_names[faces[n].material_id - 1];
                string pixelmap = model.materials[matname].pixelmap_name; //FIXME: global model ref
                printf("Setting face material %s, texture %s.\n", matname.c_str(), pixelmap.c_str());
                if (!texturizer.set_texture(pixelmap))
                {
                    printf("Ooops!");
                    return;
                }
                previous_texture = faces[n].material_id;
                glBegin(GL_TRIANGLES);
            }
        }

        render_vertex(vertices[faces[n].v1], normals[faces[n].v1], uvcoords[faces[n].v1]);
        render_vertex(vertices[faces[n].v2], normals[faces[n].v2], uvcoords[faces[n].v2]);
        render_vertex(vertices[faces[n].v3], normals[faces[n].v3], uvcoords[faces[n].v3]);
    }
    glEnd();
}
