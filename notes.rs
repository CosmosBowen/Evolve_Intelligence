// My goal: eye.cells() + ear.nerves()

// keep a struct per file

// run web: yarn start

// struct naming rule:
// All are XxxYyyZzz

// while fn naming:
// xxx_yyy_zzz

// population size < 250,000
// consider  -  rng: &mut dyn RngCore
// otherwise -  rng: &mut R where R: RngCore

// pub trait SelectionMethod {
//     fn select<'a, R, I>(
//         &self, 
//         rng: &mut R, 
//         population: &'a [I]
//     ) -> &'a I
//     where 
//         R: RngCore,
//         I: Individual;
// }

// unwrap() : Result
// expect() : Option


enum MyEnum {
    Variant { value: String },
}

fn extract(e: &MyEnum) -> String {
    match e {
        MyEnum::Variant { value } => *value, // This would be an error: move a shared reference out
    }
}




fn fitness(&self) -> f32 {
    match self {
        Self::WithChromosome { chromosome } => {
            chromosome.iter().sum()
        }
        Self::WithFitness { fitness } => *fitness, // no error
    }
}
// When you dereference a reference to a Copy type (like *fitness where fitness: &f32), 
// you're not actually moving the value out of the reference. Instead, you're making a copy of the value.


// go back 2 upper root: cd ../..