pub mod lazy{
    use std::cell::Cell;
    use std::cell::RefCell;

    pub struct Lazy<T: Copy> {
        initial_value: Cell<T>,
        functions: RefCell<Vec<Box<fn(T) -> T>>>
    }

    impl<T: Copy> Lazy<T> {
        pub fn from(value: T) -> Lazy<T>{
            let x: Vec<Box<fn(T) -> T>> = Vec::new();
            Lazy{
                initial_value: Cell::from(value),
                functions: RefCell::from(x),
            }
        }

        pub fn add_function(&self, func: fn(T) -> T){
            let mut x = self.functions.borrow_mut();
            x.push(Box::from(func));
        }

        pub fn calculate(&self){
            let mut x: T = self.initial_value.get();
            let mut funcs = self.functions.borrow_mut();
            for f in funcs.iter() {
                x = f(x);
            }
            funcs.clear();
            self.initial_value.set(x);
        }

        pub fn get(&self) -> T{
            self.calculate();
            self.initial_value.get()
        }
    }
}

#[cfg(test)]
mod tests{
    #[derive(Copy, Clone)]
    struct ComplexTestType{
        v1: i32,
        v2: i32,
        v3: f32,
    }

    #[test]
    fn test_lazy_float() {
        let x = super::lazy::Lazy::from(8.0);
        
        //Same value
        x.add_function(|y|{y * 1.0});
        assert_eq!(x.get(), 8.0);

        //Dif value
        x.add_function(|y|{y * 2.0});
        assert_eq!(x.get(), 16.0);
    }
 
    #[test]
    fn test_lazy_int() {
        let x = super::lazy::Lazy::from(8);
        
        //Same value
        x.add_function(|y|{y * 1});
        assert_eq!(x.get(), 8);

        //Dif value
        x.add_function(|y|{y * 2});
        assert_eq!(x.get(), 16);
    }

    #[test]
    fn test_lazy_complex() {
        let x = ComplexTestType{v1: 1, v2: 2, v3: 3.0};
        let x = super::lazy::Lazy::from(x);
        
        //Same value
        x.add_function(|y|test_lazy_complex_helper(y, 0, 0.0));
        let a = x.get();
        assert_eq!(a.v1, 1);
        assert_eq!(a.v2, 2);
        assert_eq!(a.v3, 3.0);

        //Same value
        x.add_function(|y|test_lazy_complex_helper(y, 6, 5.0));
        let a = x.get();
        assert_eq!(a.v1, 7);
        assert_eq!(a.v2, 8);
        assert_eq!(a.v3, 8.0);
    }

    fn test_lazy_complex_helper(complex: ComplexTestType, int: i32, float: f32) -> ComplexTestType{
        let mut x = complex;
        x.v1 += int;
        x.v2 += int;
        x.v3 += float;
        x
    }
}