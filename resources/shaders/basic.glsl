#version 100
precision lowp float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
}
