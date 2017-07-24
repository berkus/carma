#include <cstring>
#include <climits>
#include <sstream>
#include <string>
#include <vector>
#include "raiifile.h"
#include "blocks.h"
#include "loader.h"
#include "texturizer.h"

using namespace std;
using raii_wrapper::file;

extern model_t model;
extern texture_renderer_t texturizer;

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
static RT ss_atoi(const std::basic_string<T, Trait, Alloc>& the_string)
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

vector_t<float> parse_vector(std::string line)
{
    vector_t<float> v;

    std::stringstream ss(line);
    std::string str;

    getline(ss, str, ',');
    std::cerr << "got " << str << std::endl;
    v.x = std::stof(str);
    getline(ss, str, ',');
    std::cerr << "got " << str << std::endl;
    v.y = std::stof(str);
    getline(ss, str, '/');
    std::cerr << "got " << str << std::endl;
    v.z = std::stof(str);

    return v;
}

bool load_actor(const char* fname)
{
    // Load description file.
    char* txtfile = pathsubst(fname, BASE_DIR"CARS/", ".ENC");
    printf("Opening car %s\n", txtfile);
    file txt(txtfile, ios::in|ios::binary);
    free(txtfile);
    // Parse only material/mesh mumbo-jumbo for now.
    std::vector<std::string> txt_lines, load_meshes, load_pixmaps, load_materials;
    vector_t<float> leftRearWheelPos, rightRearWheelPos, leftFrontWheelPos, rightFrontWheelPos;
    std::string line;
    bool mechanics = false;
    size_t mechanicsCount = 0;
    while (txt.getline(line))
    {
        if (line.find(std::string(".PIX,G")) != line.npos)
        {
            // 3x pixelmap
            txt_lines = read_txt_lines(txt);
            load_pixmaps.insert(load_pixmaps.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
            load_pixmaps.insert(load_pixmaps.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
            load_pixmaps.insert(load_pixmaps.end(), txt_lines.begin(), txt_lines.end());
            // shadetable
            txt_lines = read_txt_lines(txt);
            // 3x material
            txt_lines = read_txt_lines(txt);
            load_materials.insert(load_materials.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
            load_materials.insert(load_materials.end(), txt_lines.begin(), txt_lines.end());
            txt_lines = read_txt_lines(txt);
            load_materials.insert(load_materials.end(), txt_lines.begin(), txt_lines.end());
            // models
            txt_lines = read_txt_lines(txt);
            load_meshes.insert(load_meshes.end(), txt_lines.begin(), txt_lines.end());
            // actors
            txt_lines = read_txt_lines(txt);
        }

        if (line.find("START OF MECHANICS STUFF version") != line.npos) {
            mechanics = true;
            mechanicsCount = 0;
        }

        if (line.find("END OF MECHANICS STUFF") != line.npos) {
            mechanics = false;
        }

        // 1. need to find wheel indices
        // 2. need to get wheel positions from mechanics stuff and assign to wheels

        if (mechanics) {
            if (mechanicsCount == 1) { // left rear wheel pos `x,y,z ///`
                leftRearWheelPos = parse_vector(line);
            }
            if (mechanicsCount == 2) { // right rear wheel pos `x,y,z ///`
                rightRearWheelPos = parse_vector(line);
            }
            if (mechanicsCount == 3) { // left front wheel pos `x,y,z ///`
                leftRearWheelPos = parse_vector(line);
            }
            if (mechanicsCount == 4) { // right front wheel pos `x,y,z ///`
                rightRearWheelPos = parse_vector(line);
            }

            ++mechanicsCount;
        }
    }
    txt.close();

    std::cout << load_meshes.size() << " meshes, "
              << load_pixmaps.size() << " pixmaps, "
              << load_materials.size() << " materials to load." << std::endl;

    std::cout << "Car wheel positions: LF " << leftFrontWheelPos
              << " ,RF " << rightFrontWheelPos
              << " ,LR " << leftRearWheelPos
              << " ,RR " << rightRearWheelPos
              << std::endl;

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
//     texturizer.dump_cache_textures();

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

