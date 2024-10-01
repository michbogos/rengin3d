pub trait Algebraic<T>:
std::ops::Mul<Output=T>
+Copy
+std::ops::Add<Output=T>
+std::ops::AddAssign
+std::ops::Neg<Output=T>
+std::default::Default
{}

macro_rules! impl_algebraic{
    ($($numType:ty)*) => ($(
        impl Algebraic<$numType> for $numType{
        }
    )*)
}

impl_algebraic!(isize i8 i16 i32 i64 i128 f32 f64);

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2<T:Algebraic<T>>{
    pub x:T,
    pub y:T
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3<T:Algebraic<T>>{
    pub x:T,
    pub y:T,
    pub z:T
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec4<T:Algebraic<T>>{
    pub x:T,
    pub y:T,
    pub z:T,
    pub w:T
}

#[derive(Debug, Copy, Clone)]
pub struct Vecn<const N:usize, T>{
    pub data:[T; N]
}

impl<const N:usize, T:Algebraic<T>> std::default::Default for Vecn<N, T>{
    fn default() -> Self {
        return Vecn { data: [T::default();N]};
    }
}

//Addition

impl<T:Algebraic<T>> std::ops::Add<Vec2<T>> for Vec2<T>{
    type Output = Vec2<T>;
    fn add(self, _rhs:Vec2<T>)->Vec2<T>{
        let mut res:Vec2<T> = Vec2::<T>{x:T::default(), y:T::default()};
        res.x = self.x+_rhs.x;
        res.y = self.y+_rhs.y;
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Add<Vec3<T>> for Vec3<T>{
    type Output = Vec3<T>;
    fn add(self, _rhs:Vec3<T>)->Vec3<T>{
        let mut res:Vec3<T> = Vec3::<T>{x:T::default(), y:T::default(), z:T::default()};
        res.x = self.x+_rhs.x;
        res.y = self.y+_rhs.y;
        res.z = self.z+_rhs.z;
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Add<Vec4<T>> for Vec4<T>{
    type Output = Vec4<T>;
    fn add(self, _rhs:Vec4<T>)->Vec4<T>{
        let mut res:Vec4<T> = Vec4::<T>{x:T::default(), y:T::default(), z:T::default(), w:T::default()};
        res.x = self.x+_rhs.x;
        res.y = self.y+_rhs.y;
        res.z = self.z+_rhs.z;
        res.w = self.w+_rhs.w;
        return res;
    }
}

impl<const N:usize, T:Algebraic<T>> std::ops::Add<Vecn<N, T>> for Vecn<N, T>{
    type Output = Vecn<N, T>;
    fn add(self, _rhs:Vecn<N, T>)->Vecn<N, T>{
        let mut res:Vecn<N, T>=Vecn::<N,T>::default();
        for i in 0..N{
            res.data[i] = self.data[i]+res.data[i];
        }
        return res;
    }
}

// Negation

impl<T:Algebraic<T>> std::ops::Neg for Vec2<T>{
    type Output = Vec2<T>;
    fn neg(self)->Vec2<T>{
        return Vec2::<T>{x:-self.x, y:-self.y};
    }
}

impl<T:Algebraic<T>> std::ops::Neg for Vec3<T>{
    type Output = Vec3<T>;
    fn neg(self)->Vec3<T>{
        return Vec3::<T>{x:-self.x, y:-self.y, z:-self.z};
    }
}

impl<T:Algebraic<T>> std::ops::Neg for Vec4<T>{
    type Output = Vec4<T>;
    fn neg(self)->Vec4<T>{
        return Vec4::<T>{x:-self.x, y:-self.y, z:-self.z, w:-self.w};
    }
}

impl<const N:usize, T:Algebraic<T>> std::ops::Neg for Vecn<N, T>{
    type Output = Vecn<N, T>;
    fn neg(self)->Vecn<N, T>{
        return Vecn::<N,T> {data:self.data.map(|x:T|-x)};
    }
}

// Subtration
macro_rules! impl_vec_subtract{
    ($($vecType:ty)*)=>($(
        impl<T:Algebraic<T>> std::ops::Sub<$vecType> for $vecType{
            type Output = $vecType;
            fn sub(self, _rhs:$vecType)->$vecType{
                return self+(-_rhs);
            }
        }
    )*)
}

impl<const N:usize, T:Algebraic<T>> std::ops::Sub<Vecn<N,T>> for Vecn<N,T>{
    type Output = Vecn<N,T>;
    fn sub(self, _rhs:Vecn<N,T>)->Vecn<N,T>{
        return self+(-_rhs);
    }
}

impl_vec_subtract!(Vec2<T> Vec3<T> Vec4<T>);

macro_rules! impl_Vec2_scale
{
    ($($numType:ty)*)=>{$(
        impl<T:Algebraic<T>> std::ops::Mul<Vec2<T>> for $numType where $numType:std::ops::Mul<T, Output=T>{
            type Output = Vec2<T>;
            fn mul(self, _rhs:Vec2<T>)->Vec2<T>{
                let mut res:Vec2<T> = Vec2::<T>{x:T::default(), y:T::default()};
                res.x = self*(_rhs.x);
                res.y = self*(_rhs.y);
                return res;
            }
        }
    )*}
}

macro_rules! impl_Vec3_scale
{
    ($($numType:ty)*)=>{$(
        impl<T:Algebraic<T>> std::ops::Mul<Vec3<T>> for $numType where $numType:std::ops::Mul<T, Output=T>{
            type Output = Vec3<T>;
            fn mul(self, _rhs:Vec3<T>)->Vec3<T>{
                let mut res:Vec3<T> = Vec3::<T>{x:T::default(), y:T::default(), z:T::default()};
                res.x = self*(_rhs.x);
                res.y = self*(_rhs.y);
                res.z = self*(_rhs.z);
                return res;
            }
        }
    )*}
}

macro_rules! impl_Vec4_scale
{
    ($($numType:ty)*)=>($(
        impl<T:Algebraic<T>> std::ops::Mul<Vec4<T>> for $numType where $numType:std::ops::Mul<T, Output=T>{
            type Output = Vec4<T>;
            fn mul(self, _rhs:Vec4<T>)->Vec4<T>{
                let mut res:Vec4<T> = Vec4::<T>{x:T::default(), y:T::default(), z:T::default(), w:T::default()};
                res.x = self*(_rhs.x);
                res.y = self*(_rhs.y);
                res.z = self*(_rhs.z);
                res.w = self*(_rhs.w);
                return res;
            }
        }
    )*)
}

macro_rules! impl_Vecn_scale
{
    ($($numType:ty)*)=>($(
        impl<const N:usize, T:Algebraic<T>> std::ops::Mul<Vecn<N, T>> for $numType where $numType:std::ops::Mul<T, Output=T>{
            type Output = Vecn<N,T>;
            fn mul(self, _rhs:Vecn<N,T>)->Vecn<N,T>{
                return Vecn::<N,T>{data:_rhs.data.map(|x:T|self*x)};
            }
        }
    )*)
}

impl_Vec2_scale!(isize i8 i16 i32 i64 i128 f32 f64);
impl_Vec3_scale!(isize i8 i16 i32 i64 i128 f32 f64);
impl_Vec4_scale!(isize i8 i16 i32 i64 i128 f32 f64);
impl_Vecn_scale!(isize i8 i16 i32 i64 i128 f32 f64);


impl<T:Algebraic<T>> std::ops::Mul<Vec2<T>> for Vec2<T>{
    type Output = T;
    fn mul(self, _rhs:Vec2<T>)->T{
        let mut res:T=T::default();
        res += self.x*(_rhs.x);
        res += self.y*(_rhs.y);
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Mul<Vec3<T>> for Vec3<T>{
    type Output = T;
    fn mul(self, _rhs:Vec3<T>)->T{
        let mut res:T = T::default();
        res += self.x*(_rhs.x);
        res += self.y*(_rhs.y);
        res += self.z*(_rhs.z);
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Mul<Vec4<T>> for Vec4<T>{
    type Output = T;
    fn mul(self, _rhs:Vec4<T>)->T{
        let mut res:T = T::default();
        res += self.x*(_rhs.x);
        res += self.y*(_rhs.y);
        res += self.z*(_rhs.z);
        res += self.w*(_rhs.w);
        return res;
    }
}

impl<const N:usize, T:Algebraic<T>> std::ops::Mul<Vecn<N,T>> for Vecn<N,T>{
    type Output = T;
    fn mul(self, _rhs:Vecn<N,T>)->T{
        let mut res:T = T::default();
        for i in 0..N{
            res += self.data[i]*_rhs.data[i];
        }
        return res;
    }
}