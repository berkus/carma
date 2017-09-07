#version 140

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D diffuse_tex;

void main() {
    color = vec4(texture(diffuse_tex, v_tex_coords).rgb, 1.0);
}
