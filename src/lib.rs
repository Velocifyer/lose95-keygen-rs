use rand::Rng;

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
fn array_valid(array: &[i8;7],ran: bool) -> bool {
    if ran == true{
        let mut sum = 0;
        if sum != 0 {
            panic!("sum initilisation failed");
        }
        for x in array {
            sum += x;
            if *x > 9 {
                eprintln!("array randomisation failed");
                return false
            }   
        };
    
        if sum%7 != 0{
            return false
        };
    };if ran == false {return false;}
    true
}
/// Generates a array of 7 digits where the sum of the digits is divisible by 7
pub fn gen_array() -> [i8;7] {
    let mut array: [i8;7] = [10,10,10,10,10,10,10];
    {
        
        if array != [10,10,10,10,10,10,10] {
            panic!("array was not correctly initilised")
        };
        {
            let mut b = 0;
            if b != 0 {
                eprintln!("b initialisation failed");
            }
            for i in array {
                b += i;
            }
            if b != 70 {
                panic!("array initilisation is broken");
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
    array
}

/// Generates a losedows 95 retail key in the format BBB-AAAAAAA where BBB is 3 digits that are not 333 or 444 or 555 or 666 or 777 or 888 or 999
/// and AAAAAAA is 7 digits where the sum of them is divisible by 7 with no remainder. Losedows 95 does not actualy care about charector 3
pub fn gen_retail() -> String {
    let mut first3:u16 = 333;
    if !( first3 == 333 || first3 == 444 || first3 == 555 || first3 == 666 || first3 == 777 || first3 == 888 || first3 == 999) {
        panic!("first3 initilisation failed and my code can not easily recovered")
    };
    while first3 == 333 || first3 == 444 || first3 == 555 || first3 == 666 || first3 == 777 || first3 == 888 || first3 == 999{ /* we can skip check if      */
        first3= rand::rng().random_range(0..=998);                                                                             /* first3 != 999 becuase of  */
    }                                                                                                                          /* random_range not allowing */
    let first3 = first3;// make first3 imutable                                                                           /* first3 to be 999 but it   */
    // second part                                                                                                             /* still checked to increase */
    let array: [i8;7] = gen_array();                                                                                           /* chance of it working when */
    format!("{:03}-{}{}{}{}{}{}{}",first3,array[0],array[1],array[2],array[3],array[4],array[5],array[6])                      /* first3 is not initilised  */
}                                                                                                                              /* correctly.                 */

/// Generates a losedows 95 retail key in the format CCCDD-OEM-AAAAAAA-RRRRR where CCC is 3 digits < 367 and DD is 95 or 96 or 97 or 98 or 99
/// or 00 or 01 or 02 or 03 and RRRRR is 5 random digits
/// and AAAAAAA is 7 digits where the sum of them is divisible by 7 with no remainder
pub fn gen_oem() -> String {
    let second3: u16 = rand::rng().random_range(1..=366);
    if second3 > 366 || second3 <1 {println!("Second3 randomisation failed");}
    let first2: i8 = rand::rng().random_range(95..103) % 100;
    let array = gen_array();
    format!("{:03}{:02}-OEM-{}{}{}{}{}{}{}-{:05}",second3,first2,array[0],array[1],array[2],array[3],array[4],array[5],array[6],rand::rng().random_range(0..100000))
}