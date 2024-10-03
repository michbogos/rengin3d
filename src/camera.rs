use crate::linalg::{self, Matrix, Vecn};

#[derive(Clone, Copy, Default)]
pub struct Camera{
    pub pos:Vecn<3, f32>,
    pub look_at:Vecn<3, f32>,
    pub up:Vecn<3, f32>,
    pub cam_right:Vecn<3, f32>,
    pub cam_dir:Vecn<3, f32>,
    pub near:f32,
    pub far:f32
}

impl Camera{
    pub fn projection_matrix(self)->Matrix<4,4,f32>{
        let data:[[f32;4];4] = [[1.,0.,0.,0.],
                                [0.,1.,0.,0.],
                                [0.,0.,-(self.far/(self.far-self.near))          ,-1.],
                                [0.,0.,-(self.far*self.near/(self.far-self.near)),1.]];
        return Matrix::<4,4,f32>{ data: data };
    }
}

