mod graphics;

fn main() {
    pollster::block_on(graphics::run());
}
