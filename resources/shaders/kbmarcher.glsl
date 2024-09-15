#version 120
precision lowp float;

#define SS(a,b,c) smoothstep(a-b,a+b,c)
#define gyr(p) dot(sin(p.xyz),cos(p.zxy))
#define T Time
#define R TextureSize

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

// Palette represents points of a gradient of colours to use
const vec3 Palette[7] = vec3[7](
    vec3(1.0, 1.0, 1.0),         // (255, 255, 255)
    vec3(1.0, 0.941, 0.961),     // (255, 240, 245)
    vec3(0.863, 0.706, 0.784),   // (220, 180, 200)
    vec3(0.671, 0.216, 0.439),   // (171, 55, 112)
    vec3(0.549, 0.039, 0.039),   // (140, 10, 10)
    vec3(0.235, 0.039, 0.039),   // (60, 10, 10)
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
    for (int i = 0; i < 6; i++) {
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

float map(in vec3 p) {
    return (1. + .2*sin(p.y*600.)) * 
    gyr(( p*(10.) + .8*gyr(( p*8. )) )) *
    (1.+sin(T+length(p.xy)*10.)) + 
    .3 * sin(T*.15 + p.z * 5. + p.y) *
    (2.+gyr(( p*(sin(T*.2+p.z*3.)*350.+250.) )));
}

vec3 norm(in vec3 p) {
    float m = map(p);
    vec2 d = vec2(.06+.06*sin(p.z),0.);
    return map(p)-vec3(
        map(p-d.xyy),map(p-d.yxy),map(p-d.yyx)
    );
}

void mainImage(out vec4 color, in vec2 coord) {
    vec2 uv = coord / R.xy;
    vec2 uvc = (coord - R.xy / 2.0) / R.y;
    float d = 0.0;
    float dd = 1.0;
    vec3 p = vec3(0.0, 0.0, T / 4.0);
    vec3 rd = normalize(vec3(uvc.xy, 1.0));
    for (float i = 0.0; i < 90.0 && dd > 0.001 && d < 2.0; i++) {
        d += dd;
        p += rd * d;
        dd = map(p) * 0.02;
    }
    vec3 n = norm(p);
    float bw = n.x + n.y;
    bw *= SS(0.9, 0.15, 1.0 / d);
    color = vec4(vec3(bw), 1.0);
}

void main() {
    vec4 color;
    mainImage(color, uv * TextureSize);
    vec3 rgb = color.rgb;
    rgb = getPaletteColor(rgb);
    color.rgb = rgb;

    // Sample the texture
    vec4 textureColor = texture2D(Texture, uv);

    // Invert the effect color where alpha is high
    if (textureColor.a > 0.5) {
        color.rgb = vec3(1.0) - color.rgb;
    }

    gl_FragColor = color;
}
