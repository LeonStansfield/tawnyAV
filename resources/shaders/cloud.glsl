#version 120
precision mediump float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

const float cloudscale = 1.1;
const float speed = 0.03;
const float clouddark = 0.5;
const float cloudlight = 0.3;
const float cloudcover = 0.2;
const float cloudalpha = 8.0;
const float skytint = 0.5;
const vec3 skycolour1 = vec3(0.2, 0.4, 0.6);
const vec3 skycolour2 = vec3(0.4, 0.7, 1.0);

// Simplex noise function or a simpler noise function for this example
float noise(vec2 p) {
    return fract(sin(dot(p, vec2(2.9898, 3.233))) * 1.0);
}

void main() {
    vec4 fragColor;
    vec2 p = uv * TextureSize / TextureSize;
    vec2 uv = p * cloudscale - Time * speed;
    uv = mod(uv, 1.0);  // Wrap the UV coordinates
    
    float cloudPattern = noise(uv * 10.0); // Adjust scale of noise
    
    vec3 skycolour = mix(skycolour2, skycolour1, p.y);
    vec3 cloudcolour = vec3(1.1, 1.1, 0.9) * clamp(clouddark + cloudlight * cloudPattern, 0.0, 1.0);
    
    float cloudAlpha = cloudcover + cloudalpha * cloudPattern;
    vec3 result = mix(skycolour, clamp(skytint * skycolour + cloudcolour, 0.0, 1.0), clamp(cloudAlpha, 0.0, 1.0));
    
    fragColor = vec4(result, 1.0);
    gl_FragColor = fragColor;
}