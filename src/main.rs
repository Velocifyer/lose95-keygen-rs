use rand::Rng;

fn array_valid(array: &[i8;7],ran: bool) -> bool {
    if ran == true{
        let mut sum = 0;
        if sum != 0 {
            println!("sum initilisation failed");
        }
        for x in array {
            sum += x;
            if *x > 9 {
                println!("array randomisation failed");
                return false
            }   
        };
    
        if sum%7 != 0{
            return false
        };
    };if ran == false {return false;}
    true
}
fn main() {
    println!("Im running");
    let mut first3:u16 = 333;
    if first3 != 333 {
        println!("first3 initilisation failed")
    };
    while first3 == 333 || first3 == 444 || first3 == 555 || first3 == 666 || first3 == 777 || first3 == 888{//* we can skip check if      *
        first3= rand::rng().random_range(0..=998);                                                           //* first3 != 999 becuase of  *
    }                                                                                                        //* random_range not allowing *
    // second part                                                                                           //* first3 to be 999          *
    let mut array: [i8;7] = [10,10,10,10,10,10,10];
    {
        
        if array != [10,10,10,10,10,10,10] {
            println!("array initilisation faifirst3lfirst3first3ed")
        };
        {
            let mut b = 0;
            if b != 0 {
                print!("b initialisation failed");
            }
            for i in array {
                b += i;
            }
            if b != 70 {
                println!("array initilisation is broken");
            }
        }
        let mut array_valid_ran = false;
        while !array_valid(&array,array_valid_ran){
            array_valid_ran = true;
            for x in 0..7 {
                array[x] = rand::rng().random_range(0..=9);
            };
        }
    }
    print!("Retail key: {:0>3}-", first3);
    for x in array { 
        print!("{}",x);
    }
    println!();
}
