struct Params {
  damping: f32,
  size: f32,
  diameter: f32,
  gravity: f32
}

fn compare_with_cell(cell: u32, pos: vec2<f32>) -> vec2<f32> {
    if cell >= 100 {
        return vec2(0);
    }
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
    let ball_pos = ball_read_b[indice.y];
    var next_pos = 2 * ball_pos - ball_read_a[indice.y] + compare_with_cell(indice, ball_pos) + compare_with_cell(indice + 1, ball_pos) + compare_with_cell(indice - 1, ball_pos) + compare_with_cell(indice - 11, ball_pos) + compare_with_cell(indice - 10, ball_pos) + compare_with_cell(indice - 9, ball_pos) + compare_with_cell(indice + 11, ball_pos) + compare_with_cell(indice + 10, ball_pos) + compare_with_cell(indice + 9, ball_pos) - vec2(0, params.gravity);
    if next_pos.x < 0 {
        next_pos.x = -next_pos.x * damping;
    } else if next_pos.x > size {
        next_pos.x = size - (next_pos.x - size) * damping;
    }
    if next_pos.y < 0 {
        next_pos.y = -next_pos.y * damping;
    } else if next_pos.y > size {
        next_pos.y = size - (next_pos.y - size) * damping;
    }
    ball_write[indice.y] = next_pos;
    write_indices[indice.y] = vec2(u32(next_pos.x / size * 10) + 10 * u32(next_pos.y * size), indice.y);
}
