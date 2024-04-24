struct Params {
  damping: f32,
  size: f32,
  diameter: f32,
}

fn compare_with_cell(cell: u32, pos: vec2<f32>) -> vec2<f32> {
    var delta: vec2<f32> = vec2(0);
    for (var i = initial_positions[cell]; i < initial_positions[cell + 1]; i++) {
        let other = ball_read_b[sorted_indices[i].y];
        let d = distance(pos, other);
        if d != 0 && d < params.diameter {
            delta += (pos - other) / 2;
        }
    }
    return delta;
}
@group(0) @binding(0) var<storage> ball_read_a : array<vec2<f32>, 2048>;
@group(0) @binding(1) var<storage> ball_read_b : array<vec2<f32>, 2048>;
@group(0) @binding(2) var<storage> sorted_indices : array<vec2<u32>, 2048>;
@group(0) @binding(3) var<storage> initial_positions : array<u32, 100>;
@group(0) @binding(4) var<storage, read_write> ball_write : array<vec2<f32>, 2048>;
@group(0) @binding(5) var<storage, read_write> write_indices : array<vec2<u32>, 2048>;
@group(0) @binding(6) var<uniform> params : Params;
@compute @workgroup_size(1)
fn update(@builtin(global_workgroup_id) id: vec3<u32>) {
    let indice = sorted_indices[id.x];
    var delta: vec2<u32> = vec2(0f);
    let ball_pos = ball_read_b[indice.y];
    
    //let next_pos = 2 * ball_pos - ball_read_a[indice.y] + 
}
