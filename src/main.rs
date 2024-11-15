use std::io;

fn main(){

    let mut input = String::new();
    let _stdin = io::stdin();


    let mut go: bool = true;

    while go {
        println!("RBG 2 HEX: R");
        println!("HEX 2 RGB: H");
        println!("EXIT: X");

        io::stdin().read_line(&mut input)
                .expect("Failed to read line");
        let opc = input.trim();

        if opc == "R"{

            input.clear();
            
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;

            let mut gotR: bool = false;
            let mut gotG: bool = false;
            let mut gotB: bool = false;

            println!("Ingrese los valores RGB");
            while !gotR {
                println!("R:");

                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let aux = input.trim();

                println!("{}",aux);

                match aux.parse::<u8>() {
                    Ok(i) => {r = i; gotR = true;},
                    Err(..) => println!("input is either not an integer or not within range: {}", aux),
                };
                input.clear();
            }

            while !gotG {
                println!("G:");

                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let aux = input.trim();

                match aux.parse::<u8>() {
                    Ok(i) => {g = i; gotG = true;},
                    Err(..) => println!("input is either not an integer or not within range: {}", aux),
                };
                input.clear();
            }

            while !gotB {
                println!("B:");

                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let aux = input.trim();

                match aux.parse::<u8>() {
                    Ok(i) => {b = i; gotB = true;},
                    Err(..) => println!("input is either not an integer or not within range: {}", aux),
                };
                input.clear();
            }

            let resHex = RGBtoHEX(r, g, b);

            println!("RGB:({},{},{}) -> HEX:{}", r, g, b, resHex);

        } else if opc == "H"{

            input.clear();

            println!("Ingrese el valor Hexadecimal");
            io::stdin().read_line(&mut input)
                    .expect("Failed to read line");

            let aux = input.trim();

            let (resR, resG, resB) = HEXtoRGB(aux.to_string());

            println!("HEX:#{} -> RGB:({},{},{})", aux.to_string(), resR, resG, resB);

            input.clear();

        } else if opc == "X"{
            println!("Hasta Pronto");
            go = false;
        } else {
            println!("Opcion Invalida, Ingrese una Opcion Valida");
            input.clear();
        }
    }
}

fn RGBtoHEX(r: u8, g: u8, b: u8) -> String{
    let mut hex = String::from("#");
    if r < 16{
        hex.push_str(&format!("{:X}", 0) as &str); 
        hex.push_str(&format!("{:X}", r) as &str); 
    } else {
        hex.push_str(&format!("{:X}", r) as &str);
    } 
    if g < 16{
        hex.push_str(&format!("{:X}", 0) as &str); 
        hex.push_str(&format!("{:X}", g) as &str); 
    } else {
        hex.push_str(&format!("{:X}", g) as &str);
    } 
    if b < 16{
        hex.push_str(&format!("{:X}", 0) as &str); 
        hex.push_str(&format!("{:X}", b) as &str); 
    } else {
        hex.push_str(&format!("{:X}", b) as &str);
    } 
    return hex;
}

fn HEXtoRGB(hex: String) -> (u8, u8, u8){
    let mut j: u8 = 0;
    let mut rHex= String::from("");
    let mut gHex= String::from("");
    let mut bHex= String::from("");

    if hex.chars().count() == 7 {
        if hex.starts_with('#') {
            for i in 1..hex.chars().count() {
                if j<2{
                    rHex.push(hex.chars().nth(i).expect("REASON"));
                } else if j<4 {
                    gHex.push(hex.chars().nth(i).expect("REASON"));
                } else if j<6 {
                    bHex.push(hex.chars().nth(i).expect("REASON"));
                }
                j+=1;
            }
        } else {
            return (0,0,0);
        }
    }
    else if hex.chars().count() == 6{
        for i in 0..hex.chars().count() {
            if j<2{
                rHex.push(hex.chars().nth(i).expect("REASON"));
            } else if j<4 {
                gHex.push(hex.chars().nth(i).expect("REASON"));
            } else if j<6 {
                bHex.push(hex.chars().nth(i).expect("REASON"));
            }
            j+=1;
        }
    } else {
        return (0,0,0);
    }

    let r = u8::from_str_radix(&rHex as &str, 16);
    let g = u8::from_str_radix(&gHex as &str, 16);
    let b = u8::from_str_radix(&bHex as &str, 16);

    (r.expect("REASON"), g.expect("REASON"), b.expect("REASON"))
}