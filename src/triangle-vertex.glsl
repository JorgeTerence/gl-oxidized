#version 140

in vec2 position;

uniform vec2 c;
uniform float x;

void main() {
    float radius = sqrt(pow(c.x - position.x, 2) + pow(c.y - position.y, 2));
    gl_Position = vec4(c + vec2(position.x, sin(position.x / radius)), 0.0, 1.0);
}