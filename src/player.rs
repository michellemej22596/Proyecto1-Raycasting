use nalgebra_glm::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,  // ángulo de vista
    pub fov: f32,  // campo de visión
}

impl Player {
    pub fn move_forward(&mut self, speed: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_x = self.pos.x + self.a.cos() * speed;
        let new_y = self.pos.y + self.a.sin() * speed;

        if !self.collides(new_x, new_y, maze, block_size) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn move_backward(&mut self, speed: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_x = self.pos.x - self.a.cos() * speed;
        let new_y = self.pos.y - self.a.sin() * speed;

        if !self.collides(new_x, new_y, maze, block_size) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn strafe_left(&mut self, speed: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_x = self.pos.x - (self.a + std::f32::consts::PI / 2.0).cos() * speed;
        let new_y = self.pos.y - (self.a + std::f32::consts::PI / 2.0).sin() * speed;

        if !self.collides(new_x, new_y, maze, block_size) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn strafe_right(&mut self, speed: f32, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_x = self.pos.x + (self.a + std::f32::consts::PI / 2.0).cos() * speed;
        let new_y = self.pos.y + (self.a + std::f32::consts::PI / 2.0).sin() * speed;

        if !self.collides(new_x, new_y, maze, block_size) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    pub fn rotate_left(&mut self, speed: f32) {
        self.a -= speed;
    }

    pub fn rotate_right(&mut self, speed: f32) {
        self.a += speed;
    }

    fn collides(&self, x: f32, y: f32, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;

        if i >= maze[0].len() || j >= maze.len() {
            return true;  // Si está fuera de los límites del mapa
        }

        maze[j][i] != ' '  // Devuelve true si la posición es una pared
    }
}

