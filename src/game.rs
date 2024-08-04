pub fn initialize(cells: &mut Vec<Vec<bool>>) {
    let grid_height = cells.len();
    let grid_width = cells[0].len();
    let center_x = grid_width / 2;
    let center_y = grid_height / 2;

    // Coloca una cruz de Pulsars en el centro
    let pulsar_offsets = [
        (-6, -6), (0, -6), (6, -6),
        (-6, 0),          (6, 0),
        (-6, 6), (0, 6), (6, 6),
    ];

    for &(dx, dy) in &pulsar_offsets {
        create_pulsar(cells, center_x as isize + dx as isize, center_y as isize + dy as isize);
    }

    // Llena el resto de la pantalla con Gliders
    fill_with_gliders(cells);
}

// Crea un patrón Pulsar en la posición especificada
fn create_pulsar(cells: &mut Vec<Vec<bool>>, top_left_x: isize, top_left_y: isize) {
    let pattern = [
        (2,0), (3,0), (4,0), (8,0), (9,0), (10,0),
        (0,2), (5,2), (7,2), (12,2),
        (0,3), (5,3), (7,3), (12,3),
        (0,4), (5,4), (7,4), (12,4),
        (2,5), (3,5), (4,5), (8,5), (9,5), (10,5),
        (2,7), (3,7), (4,7), (8,7), (9,7), (10,7),
        (0,8), (5,8), (7,8), (12,8),
        (0,9), (5,9), (7,9), (12,9),
        (0,10), (5,10), (7,10), (12,10),
        (2,12), (3,12), (4,12), (8,12), (9,12), (10,12)
    ];

    for &(x, y) in &pattern {
        let nx = top_left_x + x;
        let ny = top_left_y + y;
        if nx >= 0 && ny >= 0 && (nx as usize) < cells[0].len() && (ny as usize) < cells.len() {
            cells[ny as usize][nx as usize] = true;
        }
    }
}

// Llena la pantalla con Gliders
fn fill_with_gliders(cells: &mut Vec<Vec<bool>>) {
    let glider_pattern = [
        (0,1), (1,0), (1,1), (1,2), (2,1),
    ];

    for y in (0..cells.len()).step_by(10) {
        for x in (0..cells[0].len()).step_by(10) {
            for &(dx, dy) in &glider_pattern {
                let nx = x + dx;
                let ny = y + dy;
                if nx < cells[0].len() && ny < cells.len() {
                    cells[ny][nx] = true;
                }
            }
        }
    }
}


pub fn update(cells: &mut Vec<Vec<bool>>) {
    let mut new_cells = cells.clone();

    for y in 0..cells.len() {
        for x in 0..cells[y].len() {
            let live_neighbors = count_live_neighbors(cells, x, y);
            new_cells[y][x] = match (cells[y][x], live_neighbors) {
                (true, 2 | 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    *cells = new_cells;
}

fn count_live_neighbors(cells: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut count = 0;
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for (dx, dy) in neighbors.iter() {
        let nx = (x as isize + dx + cells[0].len() as isize) % cells[0].len() as isize;
        let ny = (y as isize + dy + cells.len() as isize) % cells.len() as isize;
        if cells[ny as usize][nx as usize] {
            count += 1;
        }
    }
    count
}

