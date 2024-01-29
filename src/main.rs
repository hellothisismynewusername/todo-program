use chrono::Datelike;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Item {
    name : String,
    desc : String,
    date : (u64, u8, u8),
}

//saving order:
//u64 num of items
//++
//u64 name len
//name
//u64 desc len
//desc
//u64 year
//u8 month
//u8 day
//++

fn main() {
    let mut items : Vec<Item> = Vec::new();
    if (!Path::new("todo_data.bin").exists()) {
        let mut buf : [u8; 3] = [0; 3];
        buf[0] = 1;
        buf[1] = 2;
        buf[2] = 3;
        let mut writefile = File::create("todo_data.bin");
        writefile.expect("failed to name file").write_all(&buf); 
    }
    let mut readfile = File::open("todo_data.bin").expect("couldnt find 'todo_data.bin'");
    let mut introbuf : [u8; 3] = [0; 3];
    readfile.read(&mut introbuf);
    if introbuf[0] != 1 || introbuf[1] != 2 || introbuf[2] != 3 {
        panic!("Not a supported file type");
    }
    let mut itemsnumbuf : [u8; 8] = [0; 8];
    readfile.read(&mut itemsnumbuf);
    let itemsnum : u64 = u64::from_ne_bytes(itemsnumbuf[0..8].try_into().unwrap());
    //println!("{}", itemsnum);
    for i in 0..itemsnum {
        let mut namelenbuf : [u8; 8] = [0; 8];
        readfile.read(&mut namelenbuf);
        let namelen : u64 = u64::from_ne_bytes(namelenbuf[0..8].try_into().unwrap());
        let mut namebuf = vec![0u8; namelen.try_into().unwrap()];
        //println!("nane len is {0}",namebuf.len());
        readfile.read_exact(&mut namebuf);
        let mut strnamebuf = String::from(std::str::from_utf8(&namebuf).unwrap());
        strnamebuf = strnamebuf.chars().map(|x| {
            unoffset(x)
        }).collect();

        let mut desclenbuf : [u8; 8] = [0; 8];
        readfile.read(&mut desclenbuf);
        let desclen : u64 = u64::from_ne_bytes(desclenbuf[0..8].try_into().unwrap());
        let mut descbuf = vec![0u8; desclen.try_into().unwrap()];
        //println!("desc len is {0}",descbuf.len());
        readfile.read_exact(&mut descbuf);
        let mut strdescbuf = String::from(std::str::from_utf8(&descbuf).unwrap());
        strdescbuf = strdescbuf.chars().map(|x| {
            unoffset(x)
        }).collect();

        let mut yearbuf : [u8; 8] = [0; 8];
        readfile.read(&mut yearbuf);
        let year : u64 = u64::from_ne_bytes(yearbuf[0..8].try_into().unwrap());

        let mut monthbuf : [u8; 1] = [0; 1];
        readfile.read(&mut monthbuf);
        let month : u8 = monthbuf[0];

        let mut daybuf : [u8; 1] = [0; 1];
        readfile.read(&mut daybuf);
        let day : u8 = daybuf[0];

        let tempitem : Item = Item {
            name: strnamebuf,
            desc: strdescbuf,
            date: (year, month, day),
        };
        items.push(tempitem);
    }

    /*
    let first : Item = Item {
        name: String::from("first item"),
        desc: String::from("desc"),
        date: (2023, 12, 4),
    };
    let again : Item = Item {
        name: first.name.clone(),
        desc: first.desc.clone(),
        date: (2023, 12, 4),
    };
    items.push(first);
    items.push(again);
    */
    let mut todoing = true;
    while todoing {
        let action : u8 = handle_input();
        if action == 1 {
            todoing = false;
        }
        if action == 2 {
            print_items(&items);
        }
        if action == 3 {
            let item : Item = input_item();
            items.push(item);
        }
        if action == 4 {
            let mut index = input_index(items.len()).unwrap_or(-1);
            if index != -1 {
                items.remove(index as usize);
            }
            println!();
        }
    }
    let mut buf : Vec<u8> = Vec::new();
    let mut intro : [u8; 3] = [0; 3];
    intro[0] = 1;
    intro[1] = 2;
    intro[2] = 3;
    buf.extend(intro);
    let numitems : u64 = items.len() as u64;
    buf.extend(numitems.to_ne_bytes());
    for item in items {
        let namelen : u64 = item.name.len() as u64;
        buf.extend(namelen.to_ne_bytes());
        let newname : String = item.name.chars().map(|x| {
            offset(x)
        }).collect();
        buf.extend(newname.bytes());
        let desclen : u64 = item.desc.len() as u64;
        buf.extend(desclen.to_ne_bytes());
        let newdesc : String = item.desc.chars().map(|x| {
            offset(x)
        }).collect();
        buf.extend(newdesc.bytes());
        buf.extend(item.date.0.to_ne_bytes());
        buf.extend(item.date.1.to_ne_bytes());
        buf.extend(item.date.2.to_ne_bytes());
    }
    let mut writefile = File::create("todo_data.bin");
    writefile.expect("failed to name file").write_all(&buf);
}

fn offset(c : char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap_or(c)
}

fn unoffset(c : char) -> char {
    std::char::from_u32(c as u32 - 1).unwrap_or(c)
}

fn input_index(len : usize) -> Option<i64> {
    let mut out : i64;
    let mut input = String::new();
    println!("Input index to delete");
    std::io::stdin().read_line(&mut input).unwrap();
    input.truncate(input.len() - 1);
    if input.parse::<i64>().is_err() {
        println!("super ug og");
        std::process::exit(2);
    }
    out = input.parse().unwrap();
    if out >= len as i64 || out < 0 {
        None
    } else {
        Some(out)
    }
}

fn input_item() -> Item {
    let date = chrono::Local::now();
    let mut out : Item = Item {
        name: String::from("blank"),
        desc: String::from("descblank"),
        date: (date.year().try_into().unwrap(), date.month().try_into().unwrap(), date.day().try_into().unwrap()),
    };
    let mut name = String::new();
    println!("\nInput name");
    std::io::stdin().read_line(&mut name).unwrap();
    name.truncate(name.len() - 1);
    let mut desc = String::new();
    println!("Input description");
    std::io::stdin().read_line(&mut desc).unwrap();
    desc.truncate(desc.len() - 1);
    out.name = name;
    out.desc = desc;
    print!("\n");
    out
}

fn print_items(items : &Vec<Item>) {
    println!("\n");
    let mut cntr : u64 = 0;
    for i in items {
        println!("{cntr}");
        println!("{0}: {1} {2} {3}", items[cntr as usize].name, items[cntr as usize].date.0, items[cntr as usize].date.1, items[cntr as usize].date.2);
        println!("{0}\n", items[cntr as usize].desc);
        cntr += 1;
    }
}

fn handle_input() -> u8 {
    let mut out : u8 = 0;
    println!("Add item (a), View list (v), Delete (d), Quit (q)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.truncate(1);
    if input == "q" {
        out = 1;
    } else if input == "v" {
        out = 2;
    } else if input == "a" {
        out = 3;
    } else if input == "d" {
        out = 4;
    }
    out
}
