#include "blocks.h"
#include <map>

class texture_t
{
public:
    GLuint bound_id;
    pixelmap_t pixelmap;

    texture_t() : bound_id(0), pixelmap() {}
};

class texture_renderer_t
{
    std::map<std::string, texture_t*> cache;

    bool read(raii_wrapper::file& f);
    bool bind();
};
