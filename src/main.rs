mod framebuffer;
mod game;

use framebuffer::Framebuffer;
use game::{initialize, update}; // Importa `initialize` y `update` del módulo `game`

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELL_SIZE: usize = 5; // Tamaño de la célula

fn main() {
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let grid_width = WIDTH / CELL_SIZE;
    let grid_height = HEIGHT / CELL_SIZE;

    // Inicializa la cuadrícula con un patrón de pulsar y figuras aleatorias
    let mut cells = vec![vec![false; grid_width]; grid_height];
    initialize(&mut cells);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(&mut cells); 

        framebuffer.draw_cells(&cells, CELL_SIZE);

        window.update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT).unwrap();
        std::thread::sleep(Duration::from_millis(100)); // Controla la velocidad de actualización
    }
}
