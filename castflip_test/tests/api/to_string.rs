//
// To allow tests to print to the console:
//
//      % cargo test -- --show-output
//      % cargo test -- --nocapture
//
// References:
//      https://doc.rust-lang.org/rustc/tests/
//      https://doc.rust-lang.org/cargo/commands/cargo-test.html
//

use std::fmt;

use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
            UData2, UVals2, UData3, UVals3, UData4, UVals4};


impl fmt::Display for FData1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for IData1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UData1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UData2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UData3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UData4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for FVals1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for IVals1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UVals1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UVals2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UVals3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}

impl fmt::Display for UVals4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_indented_string(0))
    }
}


pub trait ToIndentedString {
    fn to_indented_string(&self, depth: usize) -> String;
}


impl ToIndentedString for FData1 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("FData1 {{"),
                  format!("    ne_bytes: {},",
                          (&self.ne_bytes[..]).to_indented_string(depth + 1)),
                  format!("    se_bytes: {},",
                          (&self.se_bytes[..]).to_indented_string(depth + 1)),
                  format!("    le_bytes: {},",
                          (&self.le_bytes[..]).to_indented_string(depth + 1)),
                  format!("    be_bytes: {},",
                          (&self.be_bytes[..]).to_indented_string(depth + 1)),
                  format!("    ne_vals: {},",
                          self.ne_vals.to_indented_string(depth + 1)),
                  format!("    se_vals: {},",
                          self.se_vals.to_indented_string(depth + 1)),
                  format!("    le_vals: {},",
                          self.le_vals.to_indented_string(depth + 1)),
                  format!("    be_vals: {},",
                          self.be_vals.to_indented_string(depth + 1)),
                  format!("}}"), ],
            depth)
    }
}

impl ToIndentedString for IData1 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("IData1 {{"),
                  format!("    ne_bytes: {},",
                          (&self.ne_bytes[..]).to_indented_string(depth + 1)),
                  format!("    se_bytes: {},",
                          (&self.se_bytes[..]).to_indented_string(depth + 1)),
                  format!("    le_bytes: {},",
                          (&self.le_bytes[..]).to_indented_string(depth + 1)),
                  format!("    be_bytes: {},",
                          (&self.be_bytes[..]).to_indented_string(depth + 1)),
                  format!("    ne_vals: {},",
                          self.ne_vals.to_indented_string(depth + 1)),
                  format!("    se_vals: {},",
                          self.se_vals.to_indented_string(depth + 1)),
                  format!("    le_vals: {},",
                          self.le_vals.to_indented_string(depth + 1)),
                  format!("    be_vals: {},",
                          self.be_vals.to_indented_string(depth + 1)),
                  format!("}}"), ],
            depth)
    }
}

impl ToIndentedString for UData1 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("UData1 {{"),
                  format!("    ne_bytes: {},",
                          (&self.ne_bytes[..]).to_indented_string(depth + 1)),
                  format!("    se_bytes: {},",
                          (&self.se_bytes[..]).to_indented_string(depth + 1)),
                  format!("    le_bytes: {},",
                          (&self.le_bytes[..]).to_indented_string(depth + 1)),
                  format!("    be_bytes: {},",
                          (&self.be_bytes[..]).to_indented_string(depth + 1)),
                  format!("    ne_vals: {},",
                          self.ne_vals.to_indented_string(depth + 1)),
                  format!("    se_vals: {},",
                          self.se_vals.to_indented_string(depth + 1)),
                  format!("    le_vals: {},",
                          self.le_vals.to_indented_string(depth + 1)),
                  format!("    be_vals: {},",
                          self.be_vals.to_indented_string(depth + 1)),
                  format!("}}"), ],
            depth)
    }
}

impl ToIndentedString for UData2 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("UData2 {{"),
                  format!("    ne_bytes: {},",
                          (&self.ne_bytes[..]).to_indented_string(depth + 1)),
                  format!("    se_bytes: {},",
                          (&self.se_bytes[..]).to_indented_string(depth + 1)),
                  format!("    le_bytes: {},",
                          (&self.le_bytes[..]).to_indented_string(depth + 1)),
                  format!("    be_bytes: {},",
                          (&self.be_bytes[..]).to_indented_string(depth + 1)),
                  format!("    ne_vals: {},",
                          self.ne_vals.to_indented_string(depth + 1)),
                  format!("    se_vals: {},",
                          self.se_vals.to_indented_string(depth + 1)),
                  format!("    le_vals: {},",
                          self.le_vals.to_indented_string(depth + 1)),
                  format!("    be_vals: {},",
                          self.be_vals.to_indented_string(depth + 1)),
                  format!("}}"), ],
            depth)
    }
}

impl ToIndentedString for UData3 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("UData3 {{"),
                  format!("    ne_bytes: {},",
                          (&self.ne_bytes[..]).to_indented_string(depth + 1)),
                  format!("    se_bytes: {},",
                          (&self.se_bytes[..]).to_indented_string(depth + 1)),
                  format!("    le_bytes: {},",
                          (&self.le_bytes[..]).to_indented_string(depth + 1)),
                  format!("    be_bytes: {},",
                          (&self.be_bytes[..]).to_indented_string(depth + 1)),
                  format!("    ne_vals: {},",
                          self.ne_vals.to_indented_string(depth + 1)),
                  format!("    se_vals: {},",
                          self.se_vals.to_indented_string(depth + 1)),
                  format!("    le_vals: {},",
                          self.le_vals.to_indented_string(depth + 1)),
                  format!("    be_vals: {},",
                          self.be_vals.to_indented_string(depth + 1)),
                  format!("}}"), ],
            depth)
    }
}

impl ToIndentedString for UData4 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("UData4 {{"),
                  format!("    ne_bytes: {},",
                          (&self.ne_bytes[..]).to_indented_string(depth + 1)),
                  format!("    se_bytes: {},",
                          (&self.se_bytes[..]).to_indented_string(depth + 1)),
                  format!("    le_bytes: {},",
                          (&self.le_bytes[..]).to_indented_string(depth + 1)),
                  format!("    be_bytes: {},",
                          (&self.be_bytes[..]).to_indented_string(depth + 1)),
                  format!("    ne_vals: {},",
                          self.ne_vals.to_indented_string(depth + 1)),
                  format!("    se_vals: {},",
                          self.se_vals.to_indented_string(depth + 1)),
                  format!("    le_vals: {},",
                          self.le_vals.to_indented_string(depth + 1)),
                  format!("    be_vals: {},",
                          self.be_vals.to_indented_string(depth + 1)),
                  format!("}}"), ],
            depth)
    }
}


impl ToIndentedString for FVals1 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("FVals1 {{"),
                  format!("    val1_f32: {:.24},", self.val1_f32),
                  format!("    val2_f32: {:.24},", self.val2_f32),
                  format!("    val1_f64: {:.53},", self.val1_f64),
                  format!("    val2_f64: {:.53},", self.val2_f64),
                  format!("}}") ],
            depth)
    }
}

impl ToIndentedString for IVals1 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("IVals1 {{"),
                  format!("    val1_i8: 0x{:02X},", self.val1_i8),
                  format!("    val2_i8: 0x{:02X},", self.val2_i8),
                  format!("    val_i16: 0x{:04X},", self.val_i16),
                  format!("    val_i32: 0x{:08X},", self.val_i32),
                  format!("    val_i64: 0x{:016X},", self.val_i64),
                  format!("    val_i128: 0x{:032X},", self.val_i128),
                  format!("}}") ],
            depth)
    }
}

impl ToIndentedString for UVals1 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("UVlas1 {{"),
                  format!("    val1_u8: 0x{:02X},", self.val1_u8),
                  format!("    val2_u8: 0x{:02X},", self.val2_u8),
                  format!("    val_u16: 0x{:04X},", self.val_u16),
                  format!("    val_u32: 0x{:08X},", self.val_u32),
                  format!("    val_u64: 0x{:016X},", self.val_u64),
                  format!("    val_u128: 0x{:032X},", self.val_u128),
                  format!("}}") ],
            depth)
    }
}

impl ToIndentedString for UVals2 {
    fn to_indented_string(&self, depth: usize) -> String {
        let mut numbers = Vec::new();
        for i in 0 .. self.array.len() {
            numbers.push(format!(" 0x{:08X},", self.array[i]));
        }

        indented_join(
            vec![ format!("UVlas2 {{"),
                  format!("    array: [{} ],", numbers.concat()),
                  format!("}}") ],
            depth)
    }
}

impl ToIndentedString for UVals3 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("UVlas3( 0x{:04X},  0x{:04X},  0x{:08X}, )",
                          self.0, self.1, self.2), ],
            depth)
    }
}

impl ToIndentedString for UVals4 {
    fn to_indented_string(&self, depth: usize) -> String {
        indented_join(
            vec![ format!("UVlas4 {{"),
                  format!("    vals1: {},",
                          self.vals1.to_indented_string(depth + 1)),
                  format!("    vals2: {},",
                          self.vals2.to_indented_string(depth + 1)),
                  format!("    vals3: {},",
                          self.vals3.to_indented_string(depth + 1)),
                  format!("}}") ],
            depth)
    }
}

impl ToIndentedString for &[u8] {
    fn to_indented_string(&self, depth: usize) -> String {
        let mut lines = Vec::new();
        lines.push(String::from("["));

        if self.len() > 0 {
            let mut line = String::from("   ");
            for i in 0 .. self.len() {
                if i > 0 && (i % 8) == 0 {
                    lines.push(line);
                    line = String::from("   ");
                }
                let hex = format!(" 0x{:02X},", self[i]);
                line.push_str(&hex);
            }
            lines.push(line);
        }

        lines.push(String::from("]"));

        return indented_join(lines, depth);
    }
}


fn indented_join(lines: Vec<String>, depth: usize) -> String {
    let mut separator = String::from("\n");
    for _i in 0 .. depth {
        separator.push_str("    ");
    }

    return lines.join(&separator);
}


#[test]
fn test() {
    if false {
        let fdata1 = FData1::gen();
        let idata1 = IData1::gen();
        let udata1 = UData1::gen();
        let udata2 = UData2::gen();
        let udata3 = UData3::gen();
        let udata4 = UData4::gen();

        println!("{}", fdata1);
        println!("{}", idata1);
        println!("{}", udata1);
        println!("{}", udata2);
        println!("{}", udata3);
        println!("{}", udata4);

        println!("{}", fdata1.ne_vals);
        println!("{}", idata1.ne_vals);
        println!("{}", udata1.ne_vals);
        println!("{}", udata2.ne_vals);
        println!("{}", udata3.ne_vals);
        println!("{}", udata4.ne_vals);
    }
}
