@group(0) @binding(0) var<uniform> cameraTransform: mat4x4<f32>;

struct PolygonInput {
    @location(0) position: vec2<f32>;
    @location(1) color: vec4<f32>;
};

struct PolygonOutput {
    @builtin(position) clip_position: vec4<f32>;
    @location(0) color: vec4f,
};

@vertex
fn vertext_main(input: PolygonInput) -> PolygonOutput {
    var out: PolygonOutput;
    out.color = input.color;
    out.clip_position = cameraTransform * vec4f(input.position, 0.0, 1.0);
    return out;
}

@fragment
fn fragment_main(input: PolygonOutput) -> @location(0) vec4<f32> {
    return input.color;
}