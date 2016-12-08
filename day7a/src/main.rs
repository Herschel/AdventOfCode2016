use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    fn supports_tls(ip: &[u8]) -> bool {
        if ip.len() < 3 {
            return false;
        }

        let mut has_abba = false;
        let mut bracket_count = 0;
        for i in 0..ip.len() - 3 {
            if ip[i] != ip[i+1] && ip[i] == ip[i+3] && ip[i+1] == ip[i+2] {
                has_abba = true;
                if bracket_count > 0 {
                    // ABBA inside brackets is invalid.
                    return false;
                }
            }

            if ip[i] == b'[' {
                bracket_count += 1;
            } else if ip[i] == b']' && bracket_count > 0 {
                bracket_count -= 1;
            }
        }

        has_abba
    }

    let num_tls = input.lines().map(|l| l.bytes().collect::<Vec<u8>>()).filter(|l| supports_tls(l)).count();
    println!("Num TLS IPs: {}", num_tls);
}