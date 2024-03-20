use std::io::Write;

fn main() -> std::io::Result<()> {
    //let mut first_two_lines;
    let day_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for i in 0..12 {
        let mut text = Vec::new();
        write!(
            &mut text,
            "On the {} day of christmas,\nmy true love gave to me\n",
            day_numbers[i]
        )?;
        if i == 0 {
            write!(&mut text, "A partridge in a pear tree.\n")?
        } else {
            for i2 in (0..=i).rev() {
                write!(&mut text, "{}\n", gifts[i2])?;
            }
        }
        println!("{}", String::from_utf8_lossy(&text));
    }

    Ok(())
}
