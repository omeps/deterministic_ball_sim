struct Params {
  damping: f32,
}
@group(0) @binding(0) var<storage> ball_read_a : array<vec2<f32>, 2048>;
@group(0) @binding(1) var<storage> ball_read_b : array<vec2<f32>, 2048>;
@group(0) @binding(2) var<storage, read_write> ball_write : array<vec2<f32>, 2048>;
@group(0) @binding(3) var<uniform> params : Params;

@compute @workgroup_size(1)
fn draw(@builtin(global_workgroup_id) id: vec3<u32>) {
}
