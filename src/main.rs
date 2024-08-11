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

    let texture = get_wall_texture();

    for x in 0..block_size {
        for y in 0..block_size {
            let mut color = texture[y % texture.len()][x % texture[0].len()];
            
            // Aplica la sombra con un factor de intensidad de 0.7
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
    
    // Calcula el tamaño de los bloques en función del tamaño del framebuffer
    let block_size = std::cmp::min(framebuffer.width / maze[0].len(), framebuffer.height / maze.len());

    // Dibuja el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col])
        }
    }

    // Dibuja el jugador
    framebuffer.set_current_color(0xFFDDD);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    // Raycasting
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_size);
    }
}

fn get_wall_texture() -> Vec<Vec<u32>> {
    vec![
        vec![0xFF5733, 0xFF5733, 0xC70039, 0xC70039],
        vec![0xFF5733, 0xFF5733, 0xC70039, 0xC70039],
        vec![0x900C3F, 0x900C3F, 0x581845, 0x581845],
        vec![0x900C3F, 0x900C3F, 0x581845, 0x581845],
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

    framebuffer.set_background_color(0x1E1E1E); 
    framebuffer.set_current_color(0xFF5733);

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

