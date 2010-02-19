#include "blocks.h"
#include <stdio.h>

void mesh_t::dump()
{
    // Print vertices.
    for (size_t i = 0; i < vertices.size(); i++)
    {
        printf("Vertex{%f,%f,%f}\n", vertices[i].x, vertices[i].y, vertices[i].z);
    }
}
