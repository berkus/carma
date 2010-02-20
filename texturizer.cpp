#include "texturizer.h"

bool texture_renderer_t::read(file& f)
{
    texture_t* tex = new texture_t;
    CHECK_READ(tex->pixelmap.read(f));
    cache[tex->pixelmap.name] = tex;
    return true;
}

bool texture_renderer_t::bind(std::string name)
{
    if (cache[name]->bound_id)
        return true;

    glGenTextures(1, &bound_id);            // Allocate space for texture
    glBindTexture(GL_TEXTURE_2D, bound_id); // Set our texture handle as current

    // Create the texture
    glTexImage2D(GL_TEXTURE_2D, 0, 3, Img.GetWidth(), Img.GetHeight(), 0, GL_RGB, GL_UNSIGNED_BYTE, Img.GetImg());
/*  glTexImage2D(GL_TEXTURE_2D,0,4,Img.GetWidth(),Img.GetHeight(),0,GL_RGBA,GL_UNSIGNED_BYTE,Img.GetImg());*/

    // Specify filtering and edge actions
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_S,GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_T,GL_REPEAT);

    return true;
}
