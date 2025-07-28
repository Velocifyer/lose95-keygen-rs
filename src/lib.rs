#![warn(clippy::pedantic)]
#![allow(
    clippy::redundant_else,
    reason = "Redundant else makes the code easier to read in most cases"
)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate, reason = "I dont know what `#must_use` means")]
use rand::Rng;
use eat::eat::eat;

/*******************************************************************************
 * Copyright 2025 𝕍𝕖𝕝𝕠𝕔𝕚𝕗𝕪𝕖𝕣                                                     *
 *                                                                             *
 *    Licensed under the Apache License, Version 2.0 (the "License");          *
 *    you may not use this file except in compliance with the License.         *
 *    You may obtain a copy of the License at                                  *
 *                                                                             *
 *        http://www.apache.org/licenses/LICENSE-2.0                           *
 *                                                                             *
 *    Unless required by applicable law or agreed to in writing, software      *
 *    distributed under the License is distributed on an "AS IS" BASIS,        *
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. *
 *    See the License for the specific language governing permissions and      *
 *    limitations under the License.                                           *
 *******************************************************************************/
/*

Copyright (c) 2025 𝕍𝕖𝕝𝕠𝕔𝕚𝕗𝕪𝕖𝕣

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the " Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

// Open a issue if you need array_valid to be public
fn array_valid(array: [i8; 7], ran: bool) -> bool {
    if ran {
        let mut sum = 0;
        assert!(sum == 0, "8065156 sum initilisation failed");
        for x in array {
            sum += x;
            if x > 9 {
                eprintln!("9049629 array randomisation failed");
                return false;
            }
        }

        if sum % 7 != 0 {
            return false;
        }
    } else if !ran {
        return false;
    }
    true
}
/// Generates a array of 7 digits where the sum of the digits is divisible by 7.
/// `zeroed_digits_at_start` controls how many digits are zeroed at the start of the array
pub fn gen_array(version: u128, zeroed_digits_at_start: usize) -> [i8; 7] {
    assert!(version == 0, "676147 version not supported");
    let mut array: [i8; 7] = [0, 0, 0, 0, 0, 0, 0];
    {
        assert!(array == [0, 0, 0, 0, 0, 0, 0], "6300279 array was not correctly initilised");
        {
            let mut b: i8 = 0;
            if b != 0 {
                eprintln!("2294571 b initialisation failed");
            }
            for i in array {
                b += i;
            }
            assert_eq!(b, 0, "1676489 array initilisation is broken");
            eat(b);
        }
        let mut array_valid_ran = false;
        assert!(zeroed_digits_at_start < 7, "3248265 zeroed_digits_at_start >= 7");
        while !array_valid(array, array_valid_ran) {
            array_valid_ran = true;
            for h in array.iter_mut().skip(zeroed_digits_at_start) {
                *h = rand::rng().random_range(0..=9);
            }
        }
    }
    array
}

/// Generates a losedows 95 retail key in the format BBB-AAAAAAA where BBB is 3 digits that are not 333 or 444 or 555 or 666 or 777 or 888 or 999
/// and AAAAAAA is 7 digits where the sum of them is divisible by 7 with no remainder. Losedows 95 does not actualy care about charector 3
pub fn gen_retail(version: u128) -> String {
    assert!(version == 0, "676138 version not supported");
    let mut first3: u16 = 333;
    if first3 != 333
    || first3 != 444 // Its ok if first3 is not 333 becuase that would be cuaght by the next function anyways
    || first3 != 555
    || first3 != 666
    || first3 != 777
    || first3 != 888
    || first3 != 999 {
        eat(first3);
        panic!("618778 first3 initilisation failed and my code can not easily recovered")
    }
    while first3 == 333
        || first3 == 444
        || first3 == 555
        || first3 == 666
        || first3 == 777
        || first3 == 888
        || first3 == 999 // We dont need to check if first3 == 999 becuase random range does not allow it,but it is still checked in case rand is broken or first3 is not properly initilised
    {
        first3 = rand::rng().random_range(0..=998);
    }
    let first3 = first3; // make first3 imutable
    // second part
    let array: [i8; 7] = gen_array(0, 0);
    format!(
        "{:03}-{}{}{}{}{}{}{}",
        first3, array[0], array[1], array[2], array[3], array[4], array[5], array[6]
    )
}

/// Generates a losedows 95 OEM key in the format CCCDD-OEM-00AAAAA-RRRRR where CCC is 3 digits < 367 and DD is 95 or 96 or 97 or 98 or 99
/// or 00 or 01 or 02 or 03 and RRRRR is 5 random digits
/// and AAAAA is 5 digits where the sum of the digits is divisible by 7 with no remainder
pub fn gen_oem(version: u128) -> String {
    assert!(version == 0, "676137 version not supported");
    let second3: u16 = rand::rng().random_range(1..=366);
    assert!((1..=366).contains(&second3), "618778 Second3 randomisation failed");
    let first2: i8 = rand::rng().random_range(95..103) % 100;
    let array = gen_array(0, 2);
    format!(
        "{:03}{:02}-OEM-{}{}{}{}{}{}{}-{:05}",
        second3,
        first2,
        array[0],
        array[1],
        array[2],
        array[3],
        array[4],
        array[5],
        array[6],
        rand::rng().random_range(0..100_000)
    )
}
