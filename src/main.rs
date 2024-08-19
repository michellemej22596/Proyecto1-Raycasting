mod framebuffer;
mod maze;
mod player;
mod caster;

use minifb::{Window, WindowOptions, Key, MouseMode};
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{Player};
use crate::caster::{cast_ray};
use std::time::{Duration, Instant};


enum ViewMode {
    View2D,
    View3D,
}

const MINIMAP_SCALE: usize = 5;  // Escala del minimapa (reducido en tamaño)
const MINIMAP_MARGIN: usize = 10;  // Margen desde la esquina de la pantalla
const MINIMAP_BORDER_COLOR: u32 = 0xFFFFFF; // Color del borde del minimapa

fn draw_minimap(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player, block_size: usize) {
    let minimap_width = maze[0].len() * MINIMAP_SCALE;
    let minimap_height = maze.len() * MINIMAP_SCALE;

    let minimap_x = MINIMAP_MARGIN;
    let minimap_y = MINIMAP_MARGIN;

    // Dibujar el borde del minimapa
    for x in 0..minimap_width + 2 {
        framebuffer.point(minimap_x + x, minimap_y);
        framebuffer.point(minimap_x + x, minimap_y + minimap_height + 1);
    }
    for y in 0..minimap_height + 2 {
        framebuffer.point(minimap_x, minimap_y + y);
        framebuffer.point(minimap_x + minimap_width + 1, minimap_y + y);
    }

    // Dibujar el laberinto en el minimapa
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            let cell = maze[y][x];
            let color = match cell {
                '+' => 0xFF5733,  // Color similar al mapa principal
                '#' => 0x7A7A7A,
                '*' => 0x9A9A9A,
                _ => 0x333333,    // Fondo del minimapa
            };

            framebuffer.set_current_color(color);

            for i in 0..MINIMAP_SCALE {
                for j in 0..MINIMAP_SCALE {
                    framebuffer.point(
                        minimap_x + x * MINIMAP_SCALE + i + 1,  // +1 para el borde
                        minimap_y + y * MINIMAP_SCALE + j + 1,
                    );
                }
            }
        }
    }

    // Dibujar la posición del jugador en el minimapa
    let player_minimap_x = minimap_x + (player.pos.x / block_size as f32 * MINIMAP_SCALE as f32) as usize + 1;
    let player_minimap_y = minimap_y + (player.pos.y / block_size as f32 * MINIMAP_SCALE as f32) as usize + 1;

    framebuffer.set_current_color(0xFFFF00);  // Color amarillo para el jugador

    for i in 0..MINIMAP_SCALE {
        for j in 0..MINIMAP_SCALE {
            framebuffer.point(player_minimap_x + i, player_minimap_y + j);
        }
    }
}





fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    let texture = match cell {
        '+' => get_wall_texture1(),  // Textura para paredes de tipo '+'
        '#' => get_wall_texture2(),  // Textura para paredes de tipo '#'
        '*' => get_wall_texture3(),  // Textura para paredes de tipo '*'
        _ => get_wall_texture1(),    // Textura por defecto
    };

    for x in 0..block_size {
        for y in 0..block_size {
            let mut color = texture[y % texture.len()][x % texture[0].len()];
            let color_with_shadow = apply_shadow(color, 0.7);  
            framebuffer.set_current_color(color_with_shadow);
            framebuffer.point(xo + x, yo + y);
        }
    }
}



fn apply_shadow(color: u32, intensity: f32) -> u32 {
    let r = ((color >> 16) & 0xFF) as f32 * intensity;
    let g = ((color >> 8) & 0xFF) as f32 * intensity;
    let b = (color & 0xFF) as f32 * intensity;

    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn render(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = std::cmp::min(framebuffer.width / maze[0].len(), framebuffer.height / maze.len());

    // Dibuja el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col])
        }
    }

    // Dibuja el sprite del jugador
    let sprite = get_player_sprite();
    let sprite_size = sprite.len();

    for x in 0..sprite_size {
        for y in 0..sprite_size {
            let color = sprite[y][x];
            framebuffer.set_current_color(color);
            framebuffer.point((player.pos.x as usize) + x - sprite_size / 2, (player.pos.y as usize) + y - sprite_size / 2);
        }
    }

    // Raycasting
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_size);
    }
}

fn render_3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = std::cmp::min(framebuffer.width / maze[0].len(), framebuffer.height / maze.len());

    let num_rays = framebuffer.width; // Un rayo por columna de píxeles en la pantalla

    for x in 0..num_rays {
        // Calcula el ángulo del rayo
        let ray_angle = (player.a - player.fov / 2.0) + (x as f32 / num_rays as f32) * player.fov;

        // Cast del rayo
        let distance_to_wall = cast_ray(framebuffer, &maze, player, ray_angle, block_size);

        // Corrige la distorsión de la perspectiva
        let corrected_distance = distance_to_wall * (player.a - ray_angle).cos();
        let corrected_distance = if corrected_distance < 1.0 { 1.0 } else { corrected_distance };

        // Calcula la altura de la pared en la pantalla
        let wall_height = (framebuffer.height as f32 / corrected_distance) as usize;

        // Calcula los límites superior e inferior de la pared
        let wall_top = framebuffer.height / 2 - wall_height / 2;
        let wall_bottom = framebuffer.height / 2 + wall_height / 2;

        let wall_top = if wall_top < 0 { 0 } else { wall_top };
        let wall_bottom = if wall_bottom >= framebuffer.height { framebuffer.height - 1 } else { wall_bottom };

        // Determinar el tipo de pared y seleccionar el color
        let color = match maze[(player.pos.y / block_size as f32) as usize][(player.pos.x / block_size as f32) as usize] {
            '+' => 0xFF5733,  // Color similar a get_wall_texture1
            '#' => 0x7A7A7A,  // Color similar a get_wall_texture2
            '*' => 0x9A9A9A,  // Color similar a get_wall_texture3
            _ => 0xFFFFFF,    // Color por defecto (blanco)
        };

        let color_with_shadow = apply_shadow(color, 1.0 / corrected_distance);

        // Dibuja la pared en 3D
        for y in 0..framebuffer.height {
            if y < wall_top {
                framebuffer.set_current_color(0x1A1A1A);  // Cielo
            } else if y > wall_bottom {
                framebuffer.set_current_color(0x333333);  // Suelo
            } else {
                framebuffer.set_current_color(color_with_shadow);
            }
            framebuffer.point(x, y);
        }
    }
}



fn get_wall_texture1() -> Vec<Vec<u32>> {
    vec![
        vec![0xFF5733, 0xFF5733, 0xC70039, 0xC70039],
        vec![0xFF5733, 0xFF5733, 0xC70039, 0xC70039],
        vec![0x900C3F, 0x900C3F, 0x581845, 0x581845],
        vec![0x900C3F, 0x900C3F, 0x581845, 0x581845],
    ]

}

fn get_wall_texture2() -> Vec<Vec<u32>> {
    vec![
        vec![0x7A7A7A, 0x7A7A7A, 0xFF5733, 0xFF5733],  // Rojo vibrante
        vec![0x7A7A7A, 0x7A7A7A, 0xFF5733, 0xFF5733],
        vec![0x5A5A5A, 0x5A5A5A, 0x4A4A4A, 0x4A4A4A],
        vec![0x5A5A5A, 0x5A5A5A, 0x4A4A4A, 0x4A4A4A],
    ]
}

fn get_wall_texture3() -> Vec<Vec<u32>> {
    vec![
        vec![0x9A9A9A, 0x9A9A9A, 0x7A1A1A, 0x7A1A1A],  // Un rojo oscuro
        vec![0x9A9A9A, 0x9A9A9A, 0x7A1A1A, 0x7A1A1A],
        vec![0x7A7A7A, 0x7A7A7A, 0x6A6A6A, 0x6A6A6A],
        vec![0x7A7A7A, 0x7A7A7A, 0x6A6A6A, 0x6A6A6A],
    ]
}

fn get_player_sprite() -> Vec<Vec<u32>> {
    vec![
        vec![0xFFFFFF, 0x000000, 0xFFFFFF],
        vec![0x000000, 0xFFFFFF, 0x000000],
        vec![0xFFFFFF, 0x000000, 0xFFFFFF],
    ]
}

fn draw_char(framebuffer: &mut Framebuffer, x: usize, y: usize, char_matrix: &[[u8; 5]; 7], color: u32) {
    framebuffer.set_current_color(color);
    for (i, row) in char_matrix.iter().enumerate() {
        for (j, &pixel) in row.iter().enumerate() {
            if pixel == 1 {
                framebuffer.point(x + j, y + i);
            }
        }
    }
}

fn draw_text(framebuffer: &mut Framebuffer, x: usize, y: usize, text: &str, color: u32) {
    let char_map = [
        ('F', [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ]),
        ('P', [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ]),
        ('S', [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ]),
        ('0', [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ]),        
       
    ];

    let mut offset = 0;
    for c in text.chars() {
        if let Some(char_matrix) = char_map.iter().find(|&&(ch, _)| ch == c).map(|&(_, matrix)| matrix) {
            draw_char(framebuffer, x + offset, y, &char_matrix, color);
            offset += 6; // Espaciado entre caracteres
        }
    }
}

fn draw_fps_bar(framebuffer: &mut Framebuffer, fps: usize) {
    let max_fps = 30;  // Ahora la barra se llenará con 30 FPS
    let bar_length = (fps * 60 / max_fps).min(60);  // Ajustar la escala

    let x = framebuffer.width - 120;  // Posición en la esquina superior derecha
    let y = 30;  // Un poco más abajo que el texto de "FPS"

    framebuffer.set_current_color(0xFFFFFF);  // Color blanco para la barra

    for i in 0..bar_length {
        for j in 0..5 {  // Ancho de la barra
            framebuffer.point(x + i, y + j);
        }
    }
}


fn draw_fps(framebuffer: &mut Framebuffer, fps: usize) {
    let x = framebuffer.width - 60;  // Posición en la esquina superior derecha
    let y = 10;

    // Dibujar el texto "FPS"
    draw_text(framebuffer, x, y, "FPS", 0xFFFFFF);

    // Dibujar la barra de FPS
    draw_fps_bar(framebuffer, fps);
}

fn main() {

    // // Iniciar el sistema de audio de Rodio
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // // Cargar el archivo de audio (asegúrate de que el archivo exista)
    // let file = BufReader::new(File::open("haunted.mp3").unwrap());

    // // Decodificar el archivo de audio
    // let source = Decoder::new(file).unwrap();

    // // Reproducir el audio (sin repetir)
    // let sink = Sink::try_new(&stream_handle).unwrap();
    // sink.append(source);

    // // Mantener el programa corriendo mientras se reproduce el audio
    // sink.play();
    
    // Configuraciones del juego
    let window_width = 1200;
    let window_height = 600;
    let framebuffer_width = 1200;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(66); // Aproximadamente 15 FPS

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x1A1A1A);

    let mut player = Player {
        pos: Vec2::new(100.0, 100.0),
        a: std::f32::consts::PI / 3.0,
        fov: std::f32::consts::PI / 3.0,
    };

    let move_speed = 5.0;
    let rotate_speed = 0.1;

    let maze = load_maze("./maze.txt"); // Cargar el laberinto
    let block_size = std::cmp::min(framebuffer.width / maze[0].len(), framebuffer.height / maze.len());

    let mut view_mode = ViewMode::View2D;

    let mut last_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps_display = 0;

    let mut last_mouse_x = None; // Para almacenar la posición anterior del mouse

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let elapsed_time = current_time - last_time;

        // Contador de FPS
        fps_counter += 1;
        if elapsed_time >= Duration::from_secs(1) {
            fps_display = fps_counter;
            fps_counter = 0;
            last_time = current_time;
        }

        // Rotación con el mouse (solo horizontal)
        if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Discard) {
            if let Some(last_x) = last_mouse_x {
                let delta_x = mouse_x - last_x;
                player.a += delta_x * 0.005; // Ajusta el factor de sensibilidad
            }
            last_mouse_x = Some(mouse_x); // Actualiza la posición del mouse
        }

        // Renderizado del juego como antes...
        framebuffer.clear();

        // Movimiento del jugador con detección de colisiones
        if window.is_key_down(Key::W) {
            player.move_forward(move_speed, &maze, block_size);
        }
        if window.is_key_down(Key::S) {
            player.move_backward(move_speed, &maze, block_size);
        }
        if window.is_key_down(Key::A) {
            player.strafe_left(move_speed, &maze, block_size);
        }
        if window.is_key_down(Key::D) {
            player.strafe_right(move_speed, &maze, block_size);
        }
        if window.is_key_down(Key::Left) {
            player.rotate_left(rotate_speed);
        }
        if window.is_key_down(Key::Right) {
            player.rotate_right(rotate_speed);
        }
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            view_mode = match view_mode {
                ViewMode::View2D => ViewMode::View3D,
                ViewMode::View3D => ViewMode::View2D,
            };
        }

        // Renderizar el mundo principal
        match view_mode {
            ViewMode::View2D => render(&mut framebuffer, &player),
            ViewMode::View3D => {
                render_3d(&mut framebuffer, &player);
                draw_minimap(&mut framebuffer, &maze, &player, block_size);
            },
        }

        // Desplegar los FPS en la esquina de la pantalla
        draw_fps(&mut framebuffer, fps_display);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}