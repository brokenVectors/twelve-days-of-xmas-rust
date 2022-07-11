
fn main() {
    let gifts = [
        "And a partridge in a pear tree",
        "Two turtledoves",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let numbers = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", numbers[i]);
        if i == 0 {
            println!("A partridge in a pear tree");
        }
        else {
            for j in (0..i+1).rev() {
                println!("{}", gifts[j]);
            }
        }
        println!("");
    }
}