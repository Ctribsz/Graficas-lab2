pub struct Framebuffer {
    pub buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            buffer: vec![0xFFFFFFFF; width * height], // Inicializa el buffer con color blanco
            width,
            height,
        }
    }

    pub fn draw_cells(&mut self, cells: &[Vec<bool>], cell_size: usize) {
        // Limpia el buffer con color blanco
        self.buffer.iter_mut().for_each(|p| *p = 0xFFFFFFFF);

        let cell_color: u32 = 0xFF66FF66; // Color de las células (RGB 102, 255, 102)

        for (y, row) in cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let start_x = x * cell_size;
                    let start_y = y * cell_size;
                    for dy in 0..cell_size {
                        for dx in 0..cell_size {
                            let px = start_x + dx;
                            let py = start_y + dy;
                            if px < self.width && py < self.height {
                                self.buffer[py * self.width + px] = cell_color;
                            }
                        }
                    }
                }
            }
        }
    }
}


