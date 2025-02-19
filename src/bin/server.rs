use mechants::generator;
use mechants::server;
fn main() {
    let seed = 64;
    let width = 8;
    let height = 8;
    let chunk_size = 12;
    let sealevel = 400.0;
    let name = "Land of Ants".to_string();
    let world = generator::generate(seed, width, height, chunk_size, sealevel, name);
    server::serve(world);
}
