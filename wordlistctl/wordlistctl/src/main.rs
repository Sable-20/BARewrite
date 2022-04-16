#![feature(decl_macro)]
#![allow(dead_code)]

mod utils;

// consts
const ORGANIZATION: &str = "blackarch.org";
const LICENSE: &str = "GPLv3";
const VERSION: &str = "v1.0.0";
const PROJECT: &str = "wordlistctl";
const DESCRIPTION: &str = "Fetch, install, and search wordlist archives from websites";

fn main() {
    banner();    
}

fn banner() {
    let msg = format!("--==[ {} by {} ]==--", PROJECT, ORGANIZATION);
    utils::gen_banner(&msg);
}