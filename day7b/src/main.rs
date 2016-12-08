use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    fn supports_ssl(ip: &[u8]) -> bool {
        if ip.len() < 2 {
            return false;
        }

        let mut aba_map = HashMap::new();
        let mut bracket_count = 0;
        for i in 0..ip.len() - 2 {
            if ip[i] != ip[i+1] && ip[i] == ip[i+2]  {
                if bracket_count == 0 {
                    let &mut (ref mut has_aba, _) = aba_map.entry((ip[i], ip[i+1])).or_insert((false, false));
                    *has_aba = true;
                } else {
                    let &mut (_, ref mut has_bab) = aba_map.entry((ip[i+1], ip[i])).or_insert((false, false));
                    *has_bab = true;
                }
            }

            if ip[i] == b'[' {
                bracket_count += 1;
            } else if ip[i] == b']' && bracket_count > 0 {
                bracket_count -= 1;
            }
        }

        aba_map.into_iter().any(|(_, (has_aba, has_bab))| has_aba && has_bab)
    }

    let num_ssl = input.lines().map(|l| l.bytes().collect::<Vec<u8>>()).filter(|l| supports_ssl(l)).count();
    println!("Num SSL IPs: {}", num_ssl);
}