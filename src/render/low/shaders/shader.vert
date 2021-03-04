// shader.vert
#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

layout(set=0, binding=0) 
uniform Uniforms {
    mat4 u_view_proj; 
};

layout(set=1, binding=3) 
uniform ChunkUniform {
    vec3 chunkPosition; 
};

void main() {
    v_tex_coords = a_tex_coords;

    float x = a_position.x;
    float y = a_position.y;
    float z = a_position.z;

    x += chunkPosition.x;
    y += chunkPosition.y;
    z += chunkPosition.z;

    gl_Position = u_view_proj * vec4(vec3(x, y, z), 1.0);
}
 