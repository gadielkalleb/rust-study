mod lib; // NODEJS => import lib from './lib';

fn main() {
    println!("{}", lib::HELLO_WORLD);
}

// NODEJS
// function main() {
//     console.log(`${lib.HELLO_WORLD}`)
// }
