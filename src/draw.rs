use crate::Image;
use crate::Matrix;
use crate::Color;
use crate::CurveType;

impl Image{
    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Color){
        if x0 > x1{
            let mut tmp = x0;
            x0 = x1;
            x1 = tmp;
            tmp = y0;
            y0 = y1;
            y1 = tmp;
        }
        let slope: f32 = (y1-y0) as f32 / (x1-x0) as f32;
        if slope > 1.0{
            // octant 2
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling y
            while y <= y1{
                self.plot(x, y, color);
                if d < 0{ // as b dominates a, and we need to hit 0
                    x += 1;
                    d += a;
                }
                y += 1;
                d += b;
            }
        }else if slope >= 0.0{
            // octant 1
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling x
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as a dominates b, and we need to hit 0
                    y += 1;
                    d += b;
                }
                x += 1;
                d += a;
            }
        }else if slope < -1.0{
            // octant 7
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling x
            while y >= y1{
                self.plot(x, y, color);
                if d < 0{ // as a dominates b, and we need to hit 0
                    x += 1;
                    d -= a; // basically adding
                }
                y -= 1;
                d -= b; // basically adding
            }
        }else{
            // octant 8
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling y
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as b dominates a, and we need to hit 0
                    y -= 1;
                    d -= b; // basically adding
                }
                x += 1;
                d -= a; // basically adding
            }
        }
    }

    pub fn draw_lines(&mut self, matrix: &Matrix, color: Color){
        for i in (0..matrix.matrix_array[0].len()).step_by(2){
            self.draw_line(matrix.matrix_array[0][i] as i32, matrix.matrix_array[1][i] as i32, matrix.matrix_array[0][i+1] as i32, matrix.matrix_array[1][i+1] as i32, color);
        }
    }
}

impl Matrix{
    pub fn add_edge(&mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32){
        if self.matrix_array.len() < 4{
            *self = Matrix::new(4,0);
        }
        self.add_point(x0,y0,z0);
        self.add_point(x1,y1,z1);
    }

    pub fn add_edge_int(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32){
        if self.matrix_array.len() < 4{
            *self = Matrix::new(4,0);
        }
        self.add_point(x0 as f32,y0 as f32,z0 as f32);
        self.add_point(x1 as f32,y1 as f32,z1 as f32);
    }

    pub fn add_point(&mut self, x: f32, y: f32, z: f32){
        if self.matrix_array.len() < 4{
            *self = Matrix::new(4,0);
        }
        self.matrix_array[0].push(x);
        self.matrix_array[1].push(y);
        self.matrix_array[2].push(z);
        self.matrix_array[3].push(1.0);
    }

    pub fn add_circle( points: Matrix, cx: f32, cy: f32, cz: f32, r: f32, step: f32 ){
        let mut i = 0.0;
        while i < 1.0{
            
            points.add_edge();
            i += step;
        }
    }

    pub fn add_curve( points: &mut Matrix, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, step: f32, t: CurveType ){
        match t{
            Bezier=>{

            }
            Hermite=>{
                let r0 = y2/x2;
                let r1 = y3/x3;
            }
        }
    }
}