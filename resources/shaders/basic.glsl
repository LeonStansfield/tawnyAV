#version 100
precision lowp float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

void main() {
    // Make background black by setting the alpha to 1 and RGB to 0
    vec4 texColor = texture2D(Texture, uv);
    if (texColor.a == 0.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        gl_FragColor = texColor;
    }
}