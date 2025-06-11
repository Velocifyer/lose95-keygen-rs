/*

Copyright 2025 𝕍𝕖𝕝𝕠𝕔𝕚𝕗𝕪𝕖𝕣

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */
/* 

Copyright (c) 2025 𝕍𝕖𝕝𝕠𝕔𝕚𝕗𝕪𝕖𝕣

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the " Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
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
    while first3 == 333 || first3 == 444 || first3 == 555 || first3 == 666 || first3 == 777 || first3 == 888{//* we can skip check if      *\\
        first3= rand::rng().random_range(0..=998);                                                           //* first3 != 999 becuase of  *\\
    }                                                                                                        //* random_range not allowing *\\
    let first3 = first3;// make first3 imutable                                                         //* first3 to be 999          *\\
    // second part
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
    let array = array; // Make array imutable becuase it should change after here
    let second3: u16 = rand::rng().random_range(1..=366);
    if second3 > 366 || second3 <1 {println!("Second3 randomisation failed");}
    let first2: i8 = rand::rng().random_range(95..103) % 100;
    print!("Retail key: {:0>3}-", first3);
    for x in array { 
        print!("{}",x);
    }
    println!();
    print!("OEM key == {:0>3}{:0>2}-OEM-",second3,first2);
    for x in array { 
        print!("{}",x);
    }
    println!("-{:0>5}", rand::rng().random_range(0..100000));
}
