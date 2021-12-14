mod generator;
mod server;
fn main() {
    let seed = 64;
    let width = 16;
    let height = 16;
    let chunk_size = 16;
    let sealevel = 0.1;
    let name = "Land of Green".to_string();
    let world = generator::generate(seed, width, height, chunk_size, sealevel, name);
    server.serve(&world);
}
