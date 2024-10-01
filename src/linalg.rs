pub trait Algebraic<T>:
std::ops::Mul<Output=T>
+std::ops::Div<Output=T>
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

#[derive(Debug, Copy, Clone)]
pub struct Vecn<const N:usize, T>{
    pub data:[T; N]
}

impl<const N:usize, T:Algebraic<T>> Vecn<N, T>{
    pub fn x(self)->T{
       assert!(N>=1, "Trying to acess x of vector smaller than 1");
       return self.data[0];
    }
    pub fn y(self)->T{
        assert!(N>=2, "Trying to acess x of vector smaller than 2");
        return self.data[1];
    }

    pub fn z(self)->T{
        assert!(N>=3, "Trying to acess y of vector smaller than 3");
        return self.data[2];
    }
    pub fn w(self)->T{
        assert!(N>=4, "Trying to acess x of vector smaller than 4");
        return self.data[3];
    }
}

impl<const N:usize, T:Algebraic<T>> std::default::Default for Vecn<N, T>{
    fn default() -> Self {
        return Vecn { data: [T::default();N]};
    }
}

//Addition
impl<const N:usize, T:Algebraic<T>> std::ops::Add<Vecn<N, T>> for Vecn<N, T>{
    type Output = Vecn<N, T>;
    fn add(self, _rhs:Vecn<N, T>)->Vecn<N, T>{
        let mut res:Vecn<N, T>=Vecn::<N,T>::default();
        for i in 0..N{
            res.data[i] = self.data[i]+_rhs.data[i];
        }
        return res;
    }
}
impl<const N:usize, T:Algebraic<T>> std::ops::AddAssign<Vecn<N, T>> for Vecn<N, T>{
    fn add_assign(&mut self, _rhs:Vecn<N, T>){
        for i in 0..N{
            self.data[i] = self.data[i]+_rhs.data[i];
        }
    }
}

//Negation
impl<const N:usize, T:Algebraic<T>> std::ops::Neg for Vecn<N, T>{
    type Output = Vecn<N, T>;
    fn neg(self)->Vecn<N, T>{
        return Vecn::<N,T> {data:self.data.map(|x:T|-x)};
    }
}

//Subtraction
impl<const N:usize, T:Algebraic<T>> std::ops::Sub<Vecn<N,T>> for Vecn<N,T>{
    type Output = Vecn<N,T>;
    fn sub(self, _rhs:Vecn<N,T>)->Vecn<N,T>{
        return self+(-_rhs);
    }
}
impl<const N:usize, T:Algebraic<T>> std::ops::SubAssign<Vecn<N,T>> for Vecn<N,T>{
    fn sub_assign(&mut self, _rhs:Vecn<N,T>){
        for i in 0..N{
            self.data[i] = self.data[i]+_rhs.data[i];
        }
    }
}

//Scale
impl<const N:usize, T:Algebraic<T>> std::ops::Mul<T> for Vecn<N,T>{
    type Output = Vecn<N,T>;
    fn mul(self, _rhs:T)->Vecn<N,T>{
        return Vecn::<N,T>{data:self.data.map(|x:T|x*_rhs)};
    }
}

//Divide
impl<const N:usize, T:Algebraic<T>> std::ops::Div<T> for Vecn<N, T>{
    type Output = Vecn<N,T>;
    fn div(self, _rhs:T)->Vecn<N,T>{
        return Vecn::<N,T>{data:self.data.map(|x:T|x/_rhs)};
    }
}

//Dot product
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