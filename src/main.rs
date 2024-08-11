mod framebuffer;
mod maze;
mod player;
mod caster;

use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use std::time::Duration;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{Player};
use crate::caster::{cast_ray};

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




fn main() {
    let window_width = 1200;  // Tamaño de la ventana ajustado
    let window_height = 600;  // Tamaño de la ventana ajustado
    let framebuffer_width = 1200;  // Tamaño del framebuffer ajustado
    let framebuffer_height = 600;  // Tamaño del framebuffer ajustado
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x1A1A1A);

    let player = Player {
        pos: Vec2::new(100.0, 100.0),  // Ajusta la posición inicial si es necesario
        a: PI / 3.0,
        fov: PI / 3.0
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        
        render(&mut framebuffer, &player);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

