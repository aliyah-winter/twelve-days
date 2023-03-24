use print_typewriter::{char_duration, println_typed};

fn main() {
let mut duration = char_duration!(default 50.ms);

const DAYS:  [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

const GIFTS: [&str; 12] = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

for idx in 0..=11 {
  println_typed!(duration, "On the {} day of Christmas my true love sent to me", DAYS[idx]);
    if idx == 0 {
    println_typed!(duration, "{}.", GIFTS[0]); 
    }
    else {
      let mut giftidx = idx;
      while giftidx > 0 {
        if giftidx == 4 {
          duration = char_duration!(default 100.ms);
        } else {
          duration = char_duration!(default 50.ms);
        }
        println_typed!(duration, "{},", GIFTS[giftidx]);
        giftidx -= 1;
      }
      println_typed!(duration, "and {}.", GIFTS[0])
    }
}

}
