struct Params {
    order_of_pass: u32,
    direction: u32,
    order_of_stage: u32,

}

//the size of the sort for each pass is 2 4 2 8 4 2 16 8 4 2 but we can just use order_of_pass and force cpu to deal with it
//Only issue is finding what other # to use -- we could use 1024 threads but 2048 is easier
//We will just use bitshifts & a little subtraction -- thankfully, we dont need to use modulo
//order_of_pass is 1 2 1 3 2 1 4 3 2 1 -- this way it works with bitshifts
fn sign(n: u32) -> u32 {
    return n - ((n >> 1) << 1);
}
//keep only shift bits.
fn keep_only(n: u32, shift: u32) -> u32 {
    return (n << (32 - shift)) >> (32 - shift);
}
@group(0) @binding(0) var<storage, read_write> b1 : array<vec2<u32>, 2048>;
@group(0) @binding(1) var<storage, read_write> b2 : array<vec2<u32>, 2048>;
@group(0) @binding(2) var<uniform> params: Params;
//I like how 64 sounds so that will be the workgroup size
@compute @workgroup_size(64,1,1)
fn start(@builtin(global_invocation_id) id: vec3<u32>) {
    let block = (id.x >> params.order_of_pass) << params.order_of_pass; // equivalent to x - (x % (2 ^ order_of_pass)) -- block below x
    let pair = keep_only(id.x - block + (1u << (params.order_of_pass - 1)), params.order_of_pass) + block;
    let block_sign = u32(sign(block >> params.order_of_stage) == 0 && params.order_of_stage != 11);
    let a = select(b1[id.x], b2[id.x], params.direction != 0);
    let b = select(b1[pair], b2[pair], params.direction != 0);
    if bool(u32(a.x < b.x) ^ u32(pair > id.x) ^ block_sign) {
        if params.direction == 0 {
            b2[id.x] = b;
        } else {
            b1[id.x] = b;
        }
    } else {
        if params.direction == 0 {
            b2[id.x] = a;
        } else {
            b1[id.x] = a;
        }
    }
}
