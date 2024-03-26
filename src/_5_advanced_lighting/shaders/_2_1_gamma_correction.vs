#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

// WebGl2 does not support interface blocks
//// declare an interface block; see 'Advanced GLSL' for what these are.
//out VS_OUT {
//    vec3 FragPos;
//    vec3 Normal;
//    vec2 TexCoords;
//} vs_out;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoords;

uniform mat4 projection;
uniform mat4 view;

void main()
{
    FragPos = aPos;
    Normal = aNormal;
    TexCoords = aTexCoords;
    gl_Position = projection * view * vec4(aPos, 1.0);
}