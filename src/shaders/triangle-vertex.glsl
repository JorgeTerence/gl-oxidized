#version 140

in vec2 position;

uniform float x;

void main() {
    float radius = sqrt(pow(position.x, 2) + pow(position.y, 2));
    vec2 warped = vec2(position.x, sin(position.x + x / radius));
    gl_Position = vec4(warped, 0.0, 1.0);
}
