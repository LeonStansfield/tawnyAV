#version 100
precision lowp float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
uniform float Seed;
uniform sampler2D Texture;

float cosRange(float amt, float range, float minimum) {
    return (((1.0 + cos(amt * 3.14159 / 180.0)) * 0.5) * range) + minimum;
}

void main() {
    vec2 uv = gl_FragCoord.xy / TextureSize;
    vec2 p = (2.0 * gl_FragCoord.xy - TextureSize) / max(TextureSize.x, TextureSize.y);
    
    float ct = cosRange(Time * 5.0, 3.0, 1.1);
    float xBoost = cosRange(Time * 0.2, 5.0, 5.0);
    float yBoost = cosRange(Time * 0.1, 10.0, 5.0);
    float fScale = cosRange(Time * 15.5, 1.25, 0.5);

    for (int i = 1; i < 40; i++) {
        float _i = float(i);
        vec2 newp = p;
        newp.x += 0.2 / _i * sin(_i * p.y + Time * cos(ct) * 0.5 / 20.0 + 0.005 * _i) * fScale + xBoost; 
        newp.y += 0.2 / _i * sin(_i * p.x + Time * ct * 0.3 / 40.0 + 0.03 * float(i + 15)) * fScale + yBoost;
        p = newp;
    }

    vec3 col = vec3(
        0.5 * sin(3.0 * p.x) + 0.5,
        0.5 * sin(3.0 * p.y) + 0.5,
        sin(p.x + p.y)
    );
    col *= 0.975;

    // Add border
    float extrusion = (col.x + col.y + col.z) / 4.0;
    extrusion *= 1.5;
    
    vec4 textureColor = texture2D(Texture, uv);
    gl_FragColor = vec4(col, extrusion) + textureColor * 0.5;
}