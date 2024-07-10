#[derive(Debug)]
struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    fn add_data(&mut self, data: &i32) {
        self.list.push(data.clone());
        self.update_average();
    }

    fn remove_data(&mut self, index: Option<&i32>) -> Option<i32> {
        match index {
            Some(i) => {
                if self.list.get(*i as usize).is_some()
                 {
                    self.list.remove(*i as usize);
                    self.update_average();
                    Some(*i)
                } else {
                    None
                }
            },
            None => {
                match self.list.pop() {
                    Some(x) => {
                        self.update_average();
                        Some(x)
                    },
                    None => None,
                }
            },
        }
    }

    fn update_average(&mut self) {
        let data_len = self.list.len();
        let mut sum = 0;

        for i in self.list.iter() {
            sum += *i as usize;
        }
        
        self.average = sum as f64 / data_len as f64;
    }

    fn average(&self) -> f64 {
        self.average
    }

}

fn main() {
  let mut collection: AveragedCollection = AveragedCollection {
    average: 0.0,
    list: Vec::new()
  };

  collection.add_data(&10);
  collection.add_data(&20);
  collection.add_data(&50);
  collection.add_data(&13);
  collection.add_data(&5);
  collection.add_data(&60);

  print!("{:?}",collection);
  
}
