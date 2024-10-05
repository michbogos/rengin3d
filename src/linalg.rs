pub trait Algebraic<T>:
std::ops::Mul<Output=T>
+std::ops::Div<Output=T>
+Copy
+std::ops::Add<Output=T>
+std::ops::AddAssign
+std::ops::SubAssign
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


//Vectors

#[derive(Debug, Copy, Clone)]
pub struct Vecn<const N:usize, T>{
    pub data:[T; N]
}

//Methods
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

//Default
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



//Matrix stuff
#[derive(Debug, Copy, Clone)]
pub struct  Matrix<const N:usize, const M:usize, T>{
    pub data:[[T;N]; M]
}

//Methods
impl Matrix<2,2, f32>{
    pub fn rotation(theta:f32)->Matrix<2, 2, f32>{
        let mut res:Matrix<2, 2, f32> = Matrix::<2, 2, f32>::default();
        res.data[0][0] = theta.cos();
        res.data[0][1] = -theta.sin();
        res.data[1][0] = theta.sin();
        res.data[1][1] = theta.cos();
        return  res;
    }
}

//Generic Matrix
impl<const N:usize, const M:usize, T:Algebraic<T>> Matrix<N,M,T>{
    pub fn ident(val:T)->Matrix<N,M,T>{
        let mut res:Matrix<N,M,T> = Matrix::<N,M,T>::default();
        for i in 0..std::cmp::min(N,M){
            res.data[i][i]+=val;
        }
        return res;
    }
}


//Square Matrix
impl<const N:usize, T:Algebraic<T>> Matrix<N,N,T>{
    pub fn scale(vec:Vecn<N, T>){
        let mut res:Matrix<N,N,T> = Matrix::<N,N,T>::default();
        for i in 0..N{
            res.data[i][i]+=vec.data[i];
        }
    }
}

//Default
impl<const N:usize, const M:usize, T:Algebraic<T>> std::default::Default for Matrix<N, M, T>{
    fn default() -> Self {
        return Matrix::<N, M, T>{data:[[T::default();N]; M]};
    }
}

//Addition
impl<const N:usize, const M:usize, T:Algebraic<T>> std::ops::Add for Matrix<N, M, T>{
    type Output = Matrix<N, M, T>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res:Self::Output = Self::Output::default();
        for i in 0..M{
            for j in 0..N{
                res.data[i][j] = self.data[i][j]+rhs.data[i][j];
            }
        }
        return res;
    }
}
impl<const N:usize, const M:usize, T:Algebraic<T>> std::ops::AddAssign for Matrix<N, M, T>{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M{
            for j in 0..N{
                self.data[i][j] += rhs.data[i][j];
            }
        }
    }
}

//Negation
impl<const N:usize, const M:usize, T:Algebraic<T>> std::ops::Neg for Matrix<N, M, T>{
    type Output = Matrix<N, M, T>;
    fn neg(self) -> Self::Output {
        let mut res:Self::Output = Self::Output::default();
        for i in 0..M{
            for j in 0..N{
                res.data[i][j] = -self.data[i][j];
            }
        }
        return res;
    }
}

//Subtraction
impl<const N:usize, const M:usize, T:Algebraic<T>> std::ops::Sub for Matrix<N, M, T>{
    type Output = Matrix<N, M, T>;
    fn sub(self, rhs:Self::Output) -> Self::Output {
        return self+(-rhs);
    }
}
impl<const N:usize, const M:usize, T:Algebraic<T>> std::ops::SubAssign for Matrix<N, M, T>{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..M{
            for j in 0..N{
                self.data[i][j] -= rhs.data[i][j];
            }
        }
    }
}

//Matmul
impl<const N:usize,const M:usize, const K:usize, T:Algebraic<T>> std::ops::Mul<Matrix<M, K, T>> for Matrix<N, M, T>{
    type Output = Matrix<N,K,T>;
    fn mul(self, _rhs:Matrix<M, K,T>)->Matrix<N,K,T>{
        let mut res:Self::Output = Self::Output::default();
        for i in 0..N{
            for j in 0..M{
                for k in 0..K{
                    res.data[i][k] += self.data[i][j]*_rhs.data[j][k];
                }
            }
        }
        return res;
    }
}

//Vector matrix multiplication
impl<const N:usize,const M:usize, T:Algebraic<T>> std::ops::Mul<Vecn<M, T>> for Matrix<N, M, T>{
    type Output = Vecn<N,T>;
    fn mul(self, _rhs:Vecn<M, T>)->Vecn<N,T>{
        let mut res:Self::Output = Self::Output::default();
        for i in 0..N{
            for j in 0..M{
                res.data[i] += self.data[i][j]*_rhs.data[j];
            }
        }
        return res;
    }
}

//Scaling Multiplication
impl<const N:usize,const M:usize, T:Algebraic<T>> std::ops::Mul<T> for Matrix<N, M, T>{
    type Output = Matrix<N,M,T>;
    fn mul(self, _rhs:T)->Matrix<N,M,T>{
        let mut res:Self::Output = Self::Output::default();
        for i in 0..M{
            for j in 0..N{
                res.data[i][j]=self.data[i][j]*_rhs;
            }
        }
        return res;
    }
}
