#version 140

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

out vec4 color;

uniform vec3 u_light;
uniform vec3 u_specular_color;
uniform sampler2D diffuse_tex;
uniform sampler2D normal_tex;

mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
    vec3 dp1 = dFdx(pos);
    vec3 dp2 = dFdy(pos);
    vec2 duv1 = dFdx(uv);
    vec2 duv2 = dFdy(uv);

    vec3 dp2perp = cross(dp2, normal);
    vec3 dp1perp = cross(normal, dp1);
    vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
    vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;

    float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
    return mat3(T * invmax, B * invmax, normal);
}

void main() {
    vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
    vec3 ambient_color = diffuse_color * 0.2;

    vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;

    // Tangent Binormal Normal
    mat3 tbn = cotangent_frame(v_normal, v_position, v_tex_coords);
    vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));

    float diffuse = max(dot(real_normal, normalize(u_light)), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(real_normal)), 0.0), 16.0);

    color = vec4(ambient_color + diffuse * diffuse_color + specular * u_specular_color, 1.0);
}
