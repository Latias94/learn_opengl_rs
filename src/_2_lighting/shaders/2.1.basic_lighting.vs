#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 FragPos;
out vec3 Normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
	// world space position of the vertices
	FragPos = vec3(model * vec4(aPos, 1.0));
	// when we apply a non-uniform scale to our model matrix, we should use a normal matrix to transform the normals
	// Normal = mat3(transpose(inverse(model))) * aNormal;
    Normal = aNormal;
	gl_Position = projection * view * vec4(FragPos, 1.0);
}
