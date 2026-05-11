fn main() {
    let verses_arr = [
        "On the first day of Christmas, my true love sent to me",
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for verses in 2..=verses_arr.len() {
        println!("-------------------------------------------------------");
        println!("Verse [{}]", verses - 1);
        println!("-------------------------------------------------------");

        println!("{}", verses_arr[0]);
        for lines in (1..verses).rev() {
            println!("{}", verses_arr[lines]);
        }
    }
}
