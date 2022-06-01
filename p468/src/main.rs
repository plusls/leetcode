use std::io;

static IPV4: &str = "IPv4";
static IPV6: &str = "IPv6";
static NEITHER: &str = "Neither";

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        if query_ip.contains(':') {
            // ipv6
            let mut count = 0;
            for (i, ip_slice) in query_ip.split(':').enumerate() {
                if ip_slice.len() > 4 || u16::from_str_radix(ip_slice, 16).is_err() {
                    return NEITHER.to_string();
                }
                count = i;
            }
            if count != 7 {
                return NEITHER.to_string();
            }
            IPV6.to_string()
        } else if query_ip.contains('.') {
            // ipv4
            let mut count = 0;
            for (i, ip_slice) in query_ip.split('.').enumerate() {
                if (ip_slice.len() > 1 && ip_slice.as_bytes()[0] == b'0') || ip_slice.parse::<u8>().is_err() {
                    return NEITHER.to_string();
                }
                count = i;
            }
            if count != 3 {
                return NEITHER.to_string();
            }
            IPV4.to_string()
        } else {
            NEITHER.to_string()
        }
        // if query_ip
    }
}

struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        println!("{}", Solution::valid_ip_address(input.to_string()));
    }
}
