pub trait Algebraic<T>:
std::ops::Mul<Output=T>
+Copy
+std::ops::Add<Output=T>
+std::ops::AddAssign
+std::ops::Neg<Output=T>
+std::default::Default
{}

macro_rules! impl_algebraic{
    ($numType:ty) => {
        impl Algebraic<$numType> for $numType{
        }
    };
}

impl_algebraic!(i32);
impl_algebraic!(i64);
impl_algebraic!(f32);
impl_algebraic!(f64);

#[derive(Debug, Copy, Clone, Default)]
pub struct vec2<T:Algebraic<T>>{
    pub x:T,
    pub y:T
}

#[derive(Debug, Copy, Clone, Default)]
pub struct vec3<T:Algebraic<T>>{
    pub x:T,
    pub y:T,
    pub z:T
}

#[derive(Debug, Copy, Clone, Default)]
pub struct vec4<T:Algebraic<T>>{
    pub x:T,
    pub y:T,
    pub z:T,
    pub w:T
}

//Addition

impl<T:Algebraic<T>> std::ops::Add<vec2<T>> for vec2<T>{
    type Output = vec2<T>;
    fn add(self, _rhs:vec2<T>)->vec2<T>{
        let mut res:vec2<T> = vec2::<T>{x:T::default(), y:T::default()};
        res.x = self.x+_rhs.x;
        res.y = self.y+_rhs.y;
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Add<vec3<T>> for vec3<T>{
    type Output = vec3<T>;
    fn add(self, _rhs:vec3<T>)->vec3<T>{
        let mut res:vec3<T> = vec3::<T>{x:T::default(), y:T::default(), z:T::default()};
        res.x = self.x+_rhs.x;
        res.y = self.y+_rhs.y;
        res.z = self.z+_rhs.z;
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Add<vec4<T>> for vec4<T>{
    type Output = vec4<T>;
    fn add(self, _rhs:vec4<T>)->vec4<T>{
        let mut res:vec4<T> = vec4::<T>{x:T::default(), y:T::default(), z:T::default(), w:T::default()};
        res.x = self.x+_rhs.x;
        res.y = self.y+_rhs.y;
        res.z = self.z+_rhs.z;
        res.w = self.w+_rhs.w;
        return res;
    }
}

// Negation

impl<T:Algebraic<T>> std::ops::Neg for vec2<T>{
    type Output = vec2<T>;
    fn neg(self)->vec2<T>{
        return vec2::<T>{x:-self.x, y:-self.y};
    }
}

impl<T:Algebraic<T>> std::ops::Neg for vec3<T>{
    type Output = vec3<T>;
    fn neg(self)->vec3<T>{
        return vec3::<T>{x:-self.x, y:-self.y, z:-self.z};
    }
}

impl<T:Algebraic<T>> std::ops::Neg for vec4<T>{
    type Output = vec4<T>;
    fn neg(self)->vec4<T>{
        return vec4::<T>{x:-self.x, y:-self.y, z:-self.z, w:-self.w};
    }
}

// Subtration
macro_rules! impl_vec_subtract{
    ($vecType:ty)=>{
        impl<T:Algebraic<T>> std::ops::Sub<$vecType> for $vecType{
            type Output = $vecType;
            fn sub(self, _rhs:$vecType)->$vecType{
                return self+(-_rhs);
            }
        }
    }
}

impl_vec_subtract!(vec2<T>);
impl_vec_subtract!(vec3<T>);
impl_vec_subtract!(vec4<T>);

macro_rules! impl_vec2_scale
{
    ($numType:ty)=>{
        impl<T:Algebraic<T>> std::ops::Mul<vec2<T>> for $numType where $numType:std::ops::Mul<T, Output=T>{
            type Output = vec2<T>;
            fn mul(self, _rhs:vec2<T>)->vec2<T>{
                let mut res:vec2<T> = vec2::<T>{x:T::default(), y:T::default()};
                res.x = self*(_rhs.x);
                res.y = self*(_rhs.y);
                return res;
            }
        }
    }
}

macro_rules! impl_vec3_scale
{
    ($numType:ty)=>{
        impl<T:Algebraic<T>> std::ops::Mul<vec3<T>> for $numType where $numType:std::ops::Mul<T, Output=T>{
            type Output = vec3<T>;
            fn mul(self, _rhs:vec3<T>)->vec3<T>{
                let mut res:vec3<T> = vec3::<T>{x:T::default(), y:T::default(), z:T::default()};
                res.x = self*(_rhs.x);
                res.y = self*(_rhs.y);
                res.z = self*(_rhs.z);
                return res;
            }
        }
    }
}

macro_rules! impl_vec4_scale
{
    ($numType:ty)=>{
        impl<T:Algebraic<T>> std::ops::Mul<vec4<T>> for $numType where $numType:std::ops::Mul<T, Output=T>{
            type Output = vec4<T>;
            fn mul(self, _rhs:vec4<T>)->vec4<T>{
                let mut res:vec4<T> = vec4::<T>{x:T::default(), y:T::default(), z:T::default(), w:T::default()};
                res.x = self*(_rhs.x);
                res.y = self*(_rhs.y);
                res.z = self*(_rhs.z);
                res.w = self*(_rhs.w);
                return res;
            }
        }
    }
}

impl_vec2_scale!(f32);
impl_vec2_scale!(f64);
impl_vec3_scale!(f32);
impl_vec3_scale!(f64);
impl_vec4_scale!(f32);
impl_vec4_scale!(f64);


impl<T:Algebraic<T>> std::ops::Mul<vec2<T>> for vec2<T>{
    type Output = <T as std::ops::Mul>::Output where <T as std::ops::Mul>::Output:std::default::Default;
    fn mul(self, _rhs:vec2<T>)-><T as std::ops::Mul>::Output{
        let mut res:<T as std::ops::Mul>::Output = <T as std::ops::Mul>::Output::default();
        res += self.x*(_rhs.x);
        res += self.y*(_rhs.y);
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Mul<vec3<T>> for vec3<T>{
    type Output = T;
    fn mul(self, _rhs:vec3<T>)->T{
        let mut res:T = T::default();
        res += self.x*(_rhs.x);
        res += self.y*(_rhs.y);
        res += self.z*(_rhs.z);
        return res;
    }
}

impl<T:Algebraic<T>> std::ops::Mul<vec4<T>> for vec4<T>{
    type Output = T;
    fn mul(self, _rhs:vec4<T>)->T{
        let mut res:T = T::default();
        res += self.x*(_rhs.x);
        res += self.y*(_rhs.y);
        res += self.z*(_rhs.z);
        res += self.w*(_rhs.w);
        return res;
    }
}