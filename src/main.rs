use std::io::{self, Write};
use std::net::Ipv4Addr;

fn count_where_0(ip: Ipv4Addr) -> i32 {
        let bit_ip = ip.to_bits();

        let binary = format!("{:08b}", bit_ip);
        let mut position = 0;

        for ch in binary.chars() {
            if ch == '0' {
                return position;
            }
            else {
                position += 1;
            }
        }
        0
    }

fn to_binary_string_with_bar(ip: Ipv4Addr, pos: i32) -> String {
    let bit_ip = ip.to_bits();
    let binary = format!("{:032b}", bit_ip);
    let mut result = String::new();
        
    for (i, ch) in binary.chars().enumerate() {
        if i % 8 == 0 && i != 0 {
            result.push('.');
        }
        if i == pos as usize {
            result.push_str(" | ");
        }
        result.push(ch);
    }
    
    result
}

fn network_address(ip: Ipv4Addr, pos: i32) -> String {
    
    let bit_ip = ip.to_bits();
    
    let result = Ipv4Addr::from_bits((bit_ip >> pos) & 0xFF);
    
    to_binary_string_with_bar(result, pos)
}

fn main() {
    print!("Enter IPv4 address (e.g., 192.168.1.1): ");
    io::stdout().flush().unwrap();

    let mut ipv4_input = String::new();
    io::stdin().read_line(&mut ipv4_input).unwrap();
    let ipv4_input = ipv4_input.trim();

    let ipv4: Ipv4Addr = match ipv4_input.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IPv4 address.");
            return;
        }
    };

    print!("Enter subnet mask (e.g., 255.255.255.0 or 24): ");
    io::stdout().flush().unwrap();

    let mut mask_input = String::new();
    io::stdin().read_line(&mut mask_input).unwrap();
    let mask_input = mask_input.trim();

    let mask = if let Ok(prefix_len) = mask_input.parse::<u8>() {
        if prefix_len <= 32 {
            // Calculate subnet mask based on prefix length
            let mut mask = 0u32;
            for i in 0..prefix_len {
                mask |= 1 << (31 - i);
            }
            // Convert the mask to Ipv4Addr
            Ipv4Addr::new(
                ((mask >> 24) & 0xFF) as u8,
                ((mask >> 16) & 0xFF) as u8,
                ((mask >> 8) & 0xFF) as u8,
                (mask & 0xFF) as u8,
            )
        } else {
            eprintln!("Invalid subnet prefix length.");
            return;
        }
    } else {
        match mask_input.parse::<Ipv4Addr>() {
            Ok(mask) => mask,
            Err(_) => {
                eprintln!("Invalid subnet mask format.");
                return;
            }
        }
    };

    let split_pos = count_where_0(mask);

    println!("IPv4 Address: {} ({})", ipv4, to_binary_string_with_bar(ipv4, split_pos));
    println!("Subnet Mask: {} ({})", mask, to_binary_string_with_bar(mask, split_pos));
}
