#version 120
precision lowp float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

#define CHANGE_S -2.0
#define N 10.0

#define COL vec3(143, 57, 143) / 255.0

// Original noise code from https://www.shadertoy.com/view/4sc3z2
#define MOD3 vec3(.1031, .11369, .13787)

vec3 hash33(vec3 p3) {
    p3 = fract(p3 * MOD3);
    p3 += dot(p3, p3.yxz + 19.19);
    return -1.0 + 2.0 * fract(vec3((p3.x + p3.y) * p3.z, (p3.x + p3.z) * p3.y, (p3.y + p3.z) * p3.x));
}

float simplex_noise(vec3 p, float textureAlpha, float Time) {
    const float K1 = 0.333333333;
    const float K2 = 0.166666667;

    vec3 i = floor(p + (p.x + p.y + p.z) * K1);
    vec3 d0 = p - (i - (i.x + i.y + i.z) * K2);

    vec3 e = step(vec3(0.0), d0 - d0.yzx);
    vec3 i1 = e * (1.0 - e.zxy);
    vec3 i2 = 1.0 - e.zxy * (1.0 - e);

    vec3 d1 = d0 - (i1 - 1.0 * K2);
    vec3 d2 = d0 - (i2 - 2.0 * K2);
    vec3 d3 = d0 - (1.0 - 3.0 * K2);

    vec4 h = max(0.6 - vec4(dot(d0, d0), dot(d1, d1), dot(d2, d2), dot(d3, d3)), 0.0);
    vec4 n = h * h * h * h * vec4(dot(d0, hash33(i)), dot(d1, hash33(i + i1)), dot(d2, hash33(i + i2)), dot(d3, hash33(i + 1.0)));

    float noiseValue = dot(vec4(31.316), n);

    // Remap the noise value from [0, 1] to [minValue, 1] based on textureAlpha
    float minValue = 0.0; // Adjust this value as needed
    if (textureAlpha > 0.5) {
        minValue = mix(-0.3, -0.0, (sin(Time) + 1.0) * 0.5);
    }
    float remappedNoiseValue = mix(minValue, 1.0, noiseValue);
    return remappedNoiseValue;
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {    
    vec2 cuv = (fragCoord - TextureSize.xy * .5) / TextureSize.y * 1.25;

    vec3 col = vec3(0.);
    for (float i = 0.; i < N; ++i) {
           
        vec2 uv = cuv * (N - i * .25) * (.25 + sin(Time * .5) * .05);

        uv.y += Time * .25;

        // Sample the texture using fixed coordinates
        vec2 fixedUV = fragCoord / TextureSize;
        vec4 textureColor = texture2D(Texture, fixedUV);

        // Use the texture color to influence the noise
        float s = simplex_noise(vec3(uv * 5.0, Time * .25 * CHANGE_S - 1.0 / N * i * 0.5), textureColor.a, Time) - .5;
        float vs = -i * (1.0 / N);

        if (step(s, vs) > 0.0) {
            col = COL * (i / N * 2.0);
            
            col.r += uv.x * .25;
            col.g -= uv.y * .25;
        }
    }

    fragColor = vec4(col, 1.0);
}

void main() {
    mainImage(gl_FragColor, gl_FragCoord.xy);
}