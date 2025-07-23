use macroquad::prelude::*;

#[macroquad::main("Insane Livestock")]
#[tokio::main]
pub async fn main() {
    loop {
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_cube(vec3(0., 0., 0.), vec3(10., 10., 10.), None, WHITE);

        set_default_camera();
        draw_text("INSANE LIVESTOCK", 20., 100., 100., WHITE);

        next_frame().await
    }
}
