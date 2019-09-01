fn main() {
    let program = "fn main() {\n    let program = \"Z\";\n\n    for c in program.chars() {\n        if c as u8 == 90 {\n            for d in program.chars() {\n                match d as u8 {\n                    10 => print!(\"\\\\n\"),\n                    34 => print!(\"\\\\\\\"\"),\n                    92 => print!(\"\\\\\\\\\"),\n                    _ => print!(\"{}\", d),\n                }\n            }\n        } else {\n            print!(\"{}\", c);\n        }\n    }\n}\n";

    for c in program.chars() {
        if c as u8 == 90 {
            for d in program.chars() {
                match d as u8 {
                    10 => print!("\\n"),
                    34 => print!("\\\""),
                    92 => print!("\\\\"),
                    _ => print!("{}", d),
                }
            }
        } else {
            print!("{}", c);
        }
    }
}
