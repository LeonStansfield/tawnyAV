use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::scene::Scene;
use crate::globals;

pub struct CloudScene {
    image: Texture2D,
    material: Material,
    time: f32,
    render_target: RenderTarget,
    render_width: f32,
    render_height: f32,
}

impl CloudScene {
    pub async fn new() -> Self {
        let image = load_texture(*globals::LOGO_FILEPATH).await.unwrap();

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: CLOUD_SCENE_FRAGMENT_SHADER,
            },
            MaterialParams {
                pipeline_params,
                uniforms: vec![
                    UniformDesc::new("TextureSize", UniformType::Float2),
                    UniformDesc::new("Time", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .unwrap();

        let render_width = *globals::RENDER_WIDTH.lock().unwrap();
        let render_height = *globals::RENDER_HEIGHT.lock().unwrap();
        let render_target = render_target(render_width as u32, render_height as u32);

        Self { 
            image,
            material, 
            time: 0.0,
            render_target,
            render_width,
            render_height,
        }
    }
}

impl Scene for CloudScene {
    fn update(&mut self) {
        self.time += get_frame_time();

        if *globals::BEAT_DETECTED.lock().unwrap() == true {
            self.time = gen_range(0.0, 100.0);

            let mut beat_detected = globals::BEAT_DETECTED.lock().unwrap();
            *beat_detected = false;
        }
    }

    fn draw(&mut self) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / self.render_width, 2.0 / self.render_height),
            target: vec2(self.render_width / 2.0, self.render_height / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);

        gl_use_material(&self.material);

        let texture_size = vec2(self.render_width, self.render_height);
        self.material.set_uniform("TextureSize", texture_size);
        self.material.set_uniform("Time", self.time);

        draw_texture_ex(
            &self.image,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.render_width, self.render_height)),
                ..Default::default()
            },
        );

        gl_use_default_material();

        set_default_camera();
        clear_background(WHITE);

        draw_texture_ex(
            &self.render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
}

const CLOUD_SCENE_FRAGMENT_SHADER: &'static str = "#version 100
precision mediump float;

varying vec2 uv;

uniform vec2 TextureSize;
uniform float Time;
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

const mat2 m = mat2( 1.6,  1.2, -1.2,  1.6 );

vec2 hash( vec2 p ) {
    p = vec2(dot(p,vec2(127.1,311.7)), dot(p,vec2(269.5,183.3)));
    return -1.0 + 2.0*fract(sin(p)*43758.5453123);
}

float noise( in vec2 p ) {
    const float K1 = 0.366025404; // (sqrt(3)-1)/2;
    const float K2 = 0.211324865; // (3-sqrt(3))/6;
    vec2 i = floor(p + (p.x+p.y)*K1);    
    vec2 a = p - i + (i.x+i.y)*K2;
    vec2 o = (a.x>a.y) ? vec2(1.0,0.0) : vec2(0.0,1.0); //vec2 of = 0.5 + 0.5*vec2(sign(a.x-a.y), sign(a.y-a.x));
    vec2 b = a - o + K2;
    vec2 c = a - 1.0 + 2.0*K2;
    vec3 h = max(0.5-vec3(dot(a,a), dot(b,b), dot(c,c) ), 0.0 );
    vec3 n = h*h*h*h*vec3( dot(a,hash(i+0.0)), dot(b,hash(i+o)), dot(c,hash(i+1.0)));
    return dot(n, vec3(70.0));    
}

float fbm(vec2 n) {
    float total = 0.0, amplitude = 0.1;
    for (int i = 0; i < 7; i++) {
        total += noise(n) * amplitude;
        n = m * n;
        amplitude *= 0.4;
    }
    return total;
}

void main() {
    vec4 fragColor;
    vec2 fragCoord = uv * TextureSize;
    vec2 p = fragCoord.xy / TextureSize;
    vec2 uv = p * vec2(TextureSize.x / TextureSize.y, 1.0);    
    float time = Time * speed;
    float q = fbm(uv * cloudscale * 0.5);
    
    float r = 0.0;
    uv *= cloudscale;
    uv -= q - time;
    uv = fract(uv); // Wrap coordinates to create infinite scrolling
    float weight = 0.8;
    for (int i = 0; i < 8; i++) {
        r += abs(weight * noise(uv));
        uv = m * uv + time;
        weight *= 0.7;
    }
    
    float f = 0.0;
    uv = p * vec2(TextureSize.x / TextureSize.y, 1.0);
    uv *= cloudscale;
    uv -= q - time;
    uv = fract(uv); // Wrap coordinates to create infinite scrolling
    weight = 0.7;
    for (int i = 0; i < 8; i++) {
        f += weight * noise(uv);
        uv = m * uv + time;
        weight *= 0.6;
    }
    
    f *= r + f;
    
    float c = 0.0;
    time = Time * speed * 2.0;
    uv = p * vec2(TextureSize.x / TextureSize.y, 1.0);
    uv *= cloudscale * 2.0;
    uv -= q - time;
    uv = fract(uv); // Wrap coordinates to create infinite scrolling
    weight = 0.4;
    for (int i = 0; i < 7; i++) {
        c += weight * noise(uv);
        uv = m * uv + time;
        weight *= 0.6;
    }
    
    float c1 = 0.0;
    time = Time * speed * 3.0;
    uv = p * vec2(TextureSize.x / TextureSize.y, 1.0);
    uv *= cloudscale * 3.0;
    uv -= q - time;
    uv = fract(uv); // Wrap coordinates to create infinite scrolling
    weight = 0.4;
    for (int i = 0; i < 7; i++) {
        c1 += abs(weight * noise(uv));
        uv = m * uv + time;
        weight *= 0.6;
    }
    
    c += c1;
    
    vec3 skycolour = mix(skycolour2, skycolour1, p.y);
    vec3 cloudcolour = vec3(1.1, 1.1, 0.9) * clamp((clouddark + cloudlight * c), 0.0, 1.0);
   
    f = cloudcover + cloudalpha * f * r;
    
    vec3 result = mix(skycolour, clamp(skytint * skycolour + cloudcolour, 0.0, 1.0), clamp(f + c, 0.0, 1.0));
    
    fragColor = vec4(result, 1.0);
    gl_FragColor = fragColor;
}
";

const DEFAULT_VERTEX_SHADER: &'static str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
";