pub trait Algebraic<T>:
std::ops::Mul<Output=T>
+Copy
+std::ops::Add<Output=T>
+std::ops::AddAssign
+std::default::Default{}

pub struct vec2<T:Algebraic<T>>{
    x:T,
    y:T
}
pub struct vec3<T:Algebraic<T>>{
    x:T,
    y:T,
    z:T
}

pub struct vec4<T:Algebraic<T>>{
    x:T,
    y:T,
    z:T,
    w:T
}

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

impl<T:Algebraic<T>> vec4<T>{
    
}