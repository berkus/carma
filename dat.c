#include <stdio.h>
#include <arpa/inet.h>
#include <stdint.h>

//TODO: use city map in RACES/ to find out actual scale?
float conv_fixed_16_16(int32_t fx)
{
    double fp = fx;
    fp = fp / ((double)(1<<31)); // multiplication by a constant
    return fp;
}

struct vertex {
    int32_t x;
    int32_t y;
    int32_t z;
};

struct uvmap {
    int32_t u;
    int32_t v;
};

struct polygon {
    int16_t v1, v2, v3; // vertex indices
    int16_t flags; // looks like flags, always only one bit set -- not always, see CITYA81.DAT!!
    int8_t unknown; // something, no idea yet, might be related to flags
};

// struct mat_poly {
//     int32_t something; // always 0x2 so far
//     int16_t mat_idx[M]; // array of indices of material from material list for i-th polygon
//     int32_t something_else, and_more; // dummy pad? always 0x0, 0x0 so far
// };

// Typical layout:
// Filename
// N vertices
// N uvmaps (per vertex)
// M polygons (triangles, actually)
// K materials
// M mat_polys assigning material to poly
// ..repeat

void print_fixed_16_16(int32_t v)
{
    printf("%f", conv_fixed_16_16(ntohl(v)));
}

unsigned char file_header[16];   //      The file header

struct chunk
{
    uint32_t type;     //chunk type
    uint32_t size;     //size of chunk -4
    uint32_t entries;  //number of entires
};

// for material list disregard size, use entries
#define MATERIAL_LIST 0x16
#define VERTEX_LIST 0x17
#define UVMAP_LIST 0x18
#define POLYGON_LIST 0x35
#define FILE_NAME 0x36
// for MAT_POLY_LIST add 8 to size
#define MAT_POLY_LIST 0x1a

// PIX files
#define PIX_ENTRY 0x03
#define PIX_DATA 0x21


char *name_chunk(unsigned char c) //returns the name of a chunk (to display)
{
    switch (c)
    {
        case MATERIAL_LIST:return("Material list"); // text
        case VERTEX_LIST:return("Vertex list");   // 3 components * 4 bytes
        case UVMAP_LIST:return("U&V list");      // 2 components * 4 bytes
        case POLYGON_LIST:return("Polygon list");  // 9 bytes
        case FILE_NAME:return("File Name");     // text
        case MAT_POLY_LIST:return("Material/polygon list"); // ~2.5 bytes
        default: return("Unrecognised **PROBABLE ERROR**");
    }
}


int main(int argc,char *argv[])
{
    FILE *FP;
    int loop;
    unsigned chunk_count=0;
    unsigned long count,chunk_size,number_entries;
    struct chunk chunk_header;
    unsigned char buffer;

    if(argc<2)       //Was a file name specified?
    {
        printf("\n\n ERROR!!!   ");
        puts("File name required\n");
        return(1);    //if not, exit
    }

    if ((FP = fopen(argv[1], "rb"))== NULL)   //open the specified file
    {
        puts("\n\n ERROR!!!  Cannot open input file.\n");
        return(1);
    }

    count=fread(file_header,1,16,FP);        //Read 16 byte file header

    if(count<16)
    {
        puts("\n\n ERROR!!!  File header truncated.\n");     //exit if file header short
        return(1);
    }

    printf("File header Data: ");      //Print file header to the screen
    for(loop=0;loop<16;loop++)
    {
        printf("%02hX ",(file_header[loop]));
    }
    puts("\nReading Chunks:");

    //************** CHUNK PARSER
    count=fread(&chunk_header,(sizeof(struct chunk)-2),1,FP);
    //read header for next chunk  (first chunk starts 2 bytes early?)

    while (count > 0)                     //while we're not at the end-of-file
    {
        chunk_count++;                  //add one to chunk count

        chunk_header.type = ntohl(chunk_header.type);

        // Convert chunk size to little endian format
        chunk_size = ntohl(chunk_header.size);

        chunk_size -= 4;  //Total Chunk size is usually -4
        if (chunk_header.type == MAT_POLY_LIST) chunk_size+=8; //1A chunks aren't :)
        if (chunk_count==1) chunk_size+=2;             //The first name chunk starts 2 bytes early

        // Convert number of entries to little endian format
        number_entries=ntohl(chunk_header.entries);

        printf("\nChunk #%d, Type: %02hXh [",chunk_count,chunk_header.type);
        printf(name_chunk(chunk_header.type));
        printf("]\n");

        printf("Chunk size = %lu bytes, Number of entries = %lu\n",chunk_size,number_entries);

        if (chunk_header.type == MATERIAL_LIST)
        {     //If it's a MAT-list type chunk, chunk size may not be good
            for(count=0;count<number_entries;count++)
            {
                buffer=1;     //use number of items to read file names.
                while(buffer!=0)
                {
                    if(fread(&buffer,1,1,FP)==0)
                    {
                        puts("\n\n\n ERROR!!!  Unexpected end of file!\n");
                        return(1);
                    }
                    if (!buffer)
                        break;
                    putchar(buffer);
                }
                putchar('\n');
            }
        }
        else if (chunk_header.type == VERTEX_LIST)
        {
            struct vertex v;
            for (count=0;count<number_entries;count++)
            {
                if(fread(&v,1,12,FP)==0)
                {
                    puts("\n\n\n ERROR!!!  Unexpected end of file!\n");
                    return(1);
                }
                printf("Vertex{");
                print_fixed_16_16(v.x);
                printf(", ");
                print_fixed_16_16(v.y);
                printf(", ");
                print_fixed_16_16(v.z);
                printf("}\n");
            }
        }
        else if (chunk_header.type == POLYGON_LIST)
        {
            for (count=0;count<chunk_size;count++)
            {
                if(fread(&buffer,1,1,FP)==0)
                {
                    puts("\n\n\n ERROR!!!  Unexpected end of file!\n");
                    return(1);
                }
                printf("%02hX ",buffer);
                if(count%9==8) putchar('\n');
            }
        }
        else if (chunk_header.type == UVMAP_LIST)
        {
            struct uvmap uv;
            for(count=0;count<number_entries;count++)
            {
                if(fread(&uv,1,8,FP)==0)
                {
                    puts("\n\n\n ERROR!!!  Unexpected end of file!\n");
                    return(1);
                }
                printf("UV{");
                print_fixed_16_16(uv.u);
                printf(", ");
                print_fixed_16_16(uv.v);
                printf("}\n");
            }
        }
        else for(count=0;count<chunk_size;count++)
        {        //data type chunk- use chunk size
            if(fread(&buffer,1,1,FP)==0)
            {
                puts("\n\n\n ERROR!!!  Unexpected end of file!\n");
                return(1);
            }
            if(chunk_header.type == FILE_NAME) {
                if (buffer)
                    putchar(buffer);
            }
            else printf("%02hX ",buffer);
            if(count%12==11) putchar('\n');
        }
        putchar('\n');

        count=fread(&chunk_header,12,1,FP);//Read header for next chunk
    }

    fclose(FP);                        //Close input file

    printf("\nChunk count: %u\n",chunk_count);

    return(0);                         //Exit program
}
