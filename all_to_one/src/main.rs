trait Shape {
    fn say(&self);
}

struct Rectagle;
struct Circle;
struct Triagle;

impl Shape for Rectagle {
    fn say(&self) {
        println!("Saya Kotak")
    }
}

impl Shape for Circle {
    fn say(&self) {
        println!("Saya Lingkaran")
    }
}

impl Shape for Triagle {
    fn say(&self) {
        println!("Saya Segitiga")
    }
}

struct StorageShape {
    shapes: Vec<Box<dyn Shape>>,
}

impl StorageShape {
    fn new() -> StorageShape {
        StorageShape { shapes: Vec::new() }
    }

    fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    fn remove_shape(&mut self) {
        self.shapes.pop();
    }

    fn get_shape(&self, index: usize) -> Option<&dyn Shape> {
        self.shapes.get(index).map(|shape| shape.as_ref())
    }

    fn todos<F: Fn(&mut Self)>(&mut self, closure: F) {
        closure(self)
    }

    fn says(&self) {
        for shape in self.shapes.iter() {
            shape.say()
        }
    }
}

fn main() {
    let mut storage: StorageShape = StorageShape::new();

    storage.add_shape(Box::new(Circle {}));
    storage.add_shape(Box::new(Triagle {}));
    storage.add_shape(Box::new(Rectagle {}));
    storage.add_shape(Box::new(Rectagle {}));
    storage.add_shape(Box::new(Circle {}));

    storage.todos(move |x| println!("{}", x.shapes.iter().len()))
}
