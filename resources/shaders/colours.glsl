#version 120
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 TextureSize;
uniform float Time;

// Palette represents points of a gradient of colours to use
const vec3 Palette[13] = vec3[13](
    vec3(1.0, 1.0, 1.0),         // (255, 255, 255)
    vec3(1.0, 0.894, 0.882),     // (255, 228, 225)
    vec3(1.0, 0.941, 0.961),     // (255, 240, 245)
    vec3(1.0, 0.894, 0.882),     // (255, 228, 225)
    vec3(0.863, 0.706, 0.784),   // (220, 180, 200)
    vec3(0.745, 0.471, 0.627),   // (190, 120, 160)
    vec3(0.671, 0.216, 0.439),   // (171, 55, 112)
    vec3(0.686, 0.004, 0.004),   // (175, 1, 1)
    vec3(0.549, 0.039, 0.039),   // (140, 10, 10)
    vec3(0.392, 0.059, 0.059),   // (100, 15, 15)
    vec3(0.235, 0.039, 0.039),   // (60, 10, 10)
    vec3(0.118, 0.020, 0.020),   // (30, 5, 5)
    vec3(0.0, 0.0, 0.0)          // (0, 0, 0)
);

// Function to interpolate between two colors
vec3 interpolate(vec3 color1, vec3 color2, float t) {
    return mix(color1, color2, t);
}

vec3 getPaletteColor(vec3 col) {
    float minDist = distance(col, Palette[0]);
    vec3 closestColor = Palette[0];
    
    // Iterate over the gradient defined by the palette
    for (int i = 0; i < 12; i++) {
        for (float t = 0.0; t <= 1.0; t += 0.01) {
            vec3 interpolatedColor = interpolate(Palette[i], Palette[i + 1], t);
            float dist = distance(col, interpolatedColor);
            if (dist < minDist) {
                minDist = dist;
                closestColor = interpolatedColor;
            }
        }
    }
    
    return closestColor;
}

void main() {
    vec4 textureColor = texture2D(Texture, uv);
    float r = 0.5 + 0.5 * sin(Time + 0.0);
    float g = 0.5 + 0.5 * sin(Time + 2.0);
    float b = 0.5 + 0.5 * sin(Time + 4.0);
    vec3 colour =  vec3(r, g, b);
    colour = getPaletteColor(colour);
    vec4 backgroundColor = vec4(colour, 1.0);

    gl_FragColor = mix(backgroundColor, textureColor, textureColor.a);
}