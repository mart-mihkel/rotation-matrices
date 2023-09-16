#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 roll;
uniform mat4 pitch;
uniform mat4 yaw;

void main() {
    mat4 matrix = roll * pitch * yaw;
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = matrix * vec4(position, 100.0);
}