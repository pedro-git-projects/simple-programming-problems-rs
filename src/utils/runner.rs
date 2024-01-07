use std::collections::HashMap;

pub struct Runner {
    exercises: HashMap<String, Box<dyn Fn()>>,
}

impl Runner {
    pub fn add_exercise(
        &mut self,
        exercise_name: String,
        exercise: Box<dyn Fn()>,
    ) -> Result<bool, &'static str> {
        if self.exercises.contains_key(&exercise_name) {
            Err("Exercise with the same name already exists")
        } else {
            self.exercises.insert(exercise_name, exercise);
            Ok(true)
        }
    }

    pub fn run_exercise(&self, exercise_name: String) -> Result<(), String> {
        match self.exercises.get(&exercise_name) {
            Some(exercise) => {
                exercise();
                Ok(())
            }
            None => Err(format!("Exercise {} not found", exercise_name)),
        }
    }

    fn print_exercise_list(&self) {
        for (index, (key, _)) in self.exercises.iter().enumerate() {
            println!("{}: {}", index + 1, key);
        }
    }

    pub fn print_messages(&self) {
        println!("Pelase, choose an exercise.");
        println!("Type its name, not its number.");
        println!();
        self.print_exercise_list();
        println!("-------------------------------");
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            exercises: HashMap::new(),
        }
    }
}
