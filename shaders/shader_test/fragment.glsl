#version 330

in vec2 fragCoord;
out vec4 fragColor;

uniform vec2 resolution;
uniform float time;

void main()
{
    // Normalize coordinates
    vec2 uv = fragCoord / resolution;
    uv = uv * 2.0 - 1.0;  // From [0,1] to [-1,1]

    // Example shader: Moving colorful gradient
    vec3 color = 0.5 + 0.5 * cos(time + uv.xyx + vec3(0,2,4));
    
    fragColor = vec4(color, 1.0);
}
