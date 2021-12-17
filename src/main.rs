mod generator;
mod server;
mod client;
mod world_structs;
fn main() {
    let seed = 64;
    let width = 16;
    let height = 16;
    let chunk_size = 16;
    let sealevel = 256.0;
    let name = "Land of Green".to_string();
    let world = generator::generate(seed, width, height, chunk_size, sealevel, name);
    server::serve(world, 5000);
    client::run();
}
