pub trait Renderable {
    fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {}
}