#version 330 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    sampler2D emission;
    float shininess;
};

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform vec3 viewPos;
uniform Material material;
uniform Light light;

void main()
{
    vec3 diffuse_color = texture(material.diffuse, TexCoords).rgb;
    vec3 specular_color = texture(material.specular, TexCoords).rgb;
    vec3 emission = texture(material.emission, TexCoords).rgb;

    // ambient
    vec3 ambient = light.ambient * diffuse_color;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * diffuse_color;

    // specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // here we inverse the sampled specular color. Black becomes white and white becomes black.
    vec3 specular = light.specular * spec * (vec3(1.0) - specular_color);

    vec3 result = ambient + diffuse + specular + emission;
    FragColor = vec4(result, 1.0);
}
