
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player, a: f32, block_size: usize) -> f32 {
    let mut d = 0.0;

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' {
            return d;
        }

        framebuffer.point(x, y);

        d += 10.0;  // Incremento de la distancia en 10 unidades por iteración
    }
}
