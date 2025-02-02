#version 140

in vec2 position;

uniform vec2 cursor;

void main() {
    vec2 pos = position;
    pos.x = pos.x + cursor.x - 1;
    pos.y = pos.y - cursor.y;
    gl_Position = vec4(pos, 1.0, 1.0);
}
