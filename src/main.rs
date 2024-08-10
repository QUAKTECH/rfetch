/*
Name : RFetch (Rust Fetch)
Author : AeternusDio
Date : Tue August 6th 2024
Description : Minimalist system information tool written in Rust.
Version : 1.0.0
License : QUAKTECH license v2 & MIT Licenses
Editor : Vim
Language : Rust

#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#

Copyright 2024 QuakTech

Permission is hereby granted, free of charge, 
to any person obtaining a copy of this software and associated documentation files (the “Software”), 
to deal in the Software without restriction, including without limitation the rights to 
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, 
nd to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice 
shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. 
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, 
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

use std::process::Command;
use std::env;
use sys_info;
use whoami;
use colored::*;

fn truncate_if_needed(input: String) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.len() > 6 {
        words[..6].join(" ") + " ..."
    } else {
        input
    }
}

fn get_uptime() -> String {
    Command::new("uptime")
        .arg("-p")
        .output()
        .map_or("Unknown".to_string(), |output| {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        })
}

fn main() {
    let kernel_name = truncate_if_needed(Command::new("uname").arg("-s").output().map_or("Unknown".to_string(), |output| String::from_utf8_lossy(&output.stdout).trim().to_string()));
    let os_type = truncate_if_needed(Command::new("uname").arg("-o").output().map_or("Unknown".to_string(), |output| String::from_utf8_lossy(&output.stdout).trim().to_string()));
    let _os_release = truncate_if_needed(Command::new("uname").arg("-r").output().map_or("Unknown".to_string(), |output| String::from_utf8_lossy(&output.stdout).trim().to_string()));
    let cpu_num = sys_info::cpu_num().unwrap_or_else(|_| 0);
    let cpu_speed = sys_info::cpu_speed().unwrap_or_else(|_| 0);
    let cpu_info = truncate_if_needed(format!("CPUs: {}, Speed: {} MHz", cpu_num, cpu_speed));
    let username = truncate_if_needed(whoami::username());
    let shell = truncate_if_needed(env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string()));
    let de = truncate_if_needed(env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string()));
    let wm = truncate_if_needed(env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "Unknown".to_string()));
    let gpu_info = if cfg!(target_os = "linux") {
        Command::new("lspci")
            .arg("-nnk")
            .output()
            .map_or("Unknown".to_string(), |output| {
                let out_str = String::from_utf8_lossy(&output.stdout);
                let gpu = out_str.lines()
                    .find(|line| line.contains("VGA compatible controller"))
                    .unwrap_or("Unknown")
                    .to_string();
                truncate_if_needed(gpu)
            })
    } else {
        "Unknown".to_string()
    };
    let uptime = truncate_if_needed(get_uptime());
    let line = "──────────────────────".magenta();
    let colored_symbol = "◉  ";    

    println!(
        "{} {}",
        format!("  User       ❯").truecolor(237, 135, 150),
        username
    );
    println!(
        "{}", line
    );
    println!(
        "{} {}",
        format!("  Kernel     ❯").truecolor(138, 173, 244),
        kernel_name
    );
    println!(
        "{} {}",
        format!("  OS         ❯").truecolor(166, 218, 149),
        os_type
    );
    println!(
        "{} {}",
        format!("󰻠  CPU        ❯").truecolor(238, 212, 159),
        cpu_info
    );
    println!(
        "{} {}",
        format!("  Shell      ❯").truecolor(237, 135, 150),
        shell
    );
    println!(
        "{} {}",
        format!("󰧨  DE         ❯").truecolor(166, 218, 149),
        de
    );
    println!(
        "{} {}",
        format!("  WM         ❯").truecolor(184, 192, 224),
        wm
    );
    println!(
        "{} {}",
        format!("󰍛  GPU        ❯").truecolor(238, 212, 159),
        gpu_info
    );
    println!(
        "{} {}",
        format!("  Uptime     ❯").truecolor(166, 218, 149),
        uptime
    );

    eprint!("{}", colored_symbol.red());
    eprint!("{}", colored_symbol.green());
    eprint!("{}", colored_symbol.blue());
    eprint!("{}", colored_symbol.yellow());
    eprint!("{}", colored_symbol.magenta());
    eprint!("{}", colored_symbol.cyan());
}
