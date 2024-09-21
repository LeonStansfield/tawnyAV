#version 120
precision lowp float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

float random(float seed, float time) {
    return fract(sin(dot(vec2(seed, time), vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    vec2 texelSize = 1.0 / TextureSize;
    
    vec4 color = texture2D(Texture, uv);
    vec4 colorRight = texture2D(Texture, uv + vec2(texelSize.x, 0.0));
    vec4 colorUp = texture2D(Texture, uv + vec2(0.0, texelSize.y));
    
    vec4 edge = abs(color - colorRight) + abs(color - colorUp);
    
    float edgeThreshold = 0.1 + random(Seed, Time) * 0.03;
    if (edge.r > edgeThreshold || edge.g > edgeThreshold || edge.b > edgeThreshold) {
        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); // Red color for lines
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0); // Black color for everything else
    }
}