pub mod User {
    pub struct Model {
        fullname: String,
        firstname: String,
        lastname: String,
        age: u8,
    }

    impl Model {
        pub fn new(fullname: String,firstname: String,lastname: String,age: u8) -> Model{
            return Model {
                fullname,
                firstname,
                lastname,
                age
            }
        }

        pub fn print_all(&self) {
            println!("
                Fullname : {}
                Firstname : {}
                Lastname : {}
                Age : {}
            ",self.fullname,self.firstname,self.lastname,self.age)
        }

    }
}