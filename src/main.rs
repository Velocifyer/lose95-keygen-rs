use lose95_keygen_rs::{gen_oem, gen_retail};
use std::env;
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
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut version = String::from("0");
    let mut print_selection = 255;
    if args.len() > 1 {
        version = args[1].clone();
        if args.len() > 2 {
            print_selection =
                command_ifyer::what_to_execute(0, &args[2], &["Retail", "Oem"], true).expect("0");
        }
    }
    if print_selection > 1 && print_selection != 255 {
        panic!("170989488 command not avalible, consider updating")
    }
    if 0 != version
        .trim()
        .parse()
        .expect("8381374 version is not valid")
    {
        panic!("3781596 version not supported");
    }
    if print_selection == 255 || print_selection == 0 {
        if print_selection == 255 {
            print!("Retail key: ")
        }
        println!("{}", gen_retail(0));
    }
    if print_selection == 1 || print_selection == 255 {
        if print_selection == 255 {
            print!("OEM key: ")
        }
        println!("{}", gen_oem(0));
    }
}
