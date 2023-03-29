use print_typewriter::{char_duration, println_typed};

fn main() {
let mut duration = char_duration!(default 50.ms, '.' -> 1.s);

const DAYS:  [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

const GIFTS: [&str; 12] = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

for mut idx in 0..=11 {
  println_typed!(duration, "On the {} day of Christmas my true love sent to me", DAYS[idx]);
    if idx == 0 {
    println_typed!(duration, "{}.\n", GIFTS[0]); 
    }
    else {
      while idx > 0 {
        if idx == 4 {
          duration = char_duration!(default 100.ms, ',' -> 1.s);
        } else {
          duration = char_duration!(default 50.ms, '.' -> 1.s);
        }
        println_typed!(duration, "{},", GIFTS[idx]);
        idx -= 1;
      }
      println_typed!(duration, "and {}.\n", GIFTS[0])
    }
}

}
