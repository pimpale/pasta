use std::convert::AsRef;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        usage();
    } else {
        match args[1].as_ref() {
            "-h" => usage(),
            "--help" => usage(),
            "lenny" => lenny(),
            "shrek" => shrekascii(),
            "navyseal" => navyseal(),
            "comsym" => comsym(),
            "noose" => noose(),
            _ => usage(),
        }
    }
}

fn usage() -> () {
    println!(
        "Usage: pasta [OPTION]
Print copypasta and ascii art

--help, -h            view this usage
lenny                 print the lenny face
shrek                 print shrek ascii art
navyseal              print the navy seal copypasta
comsym                print the communist logo
noose                 print an ascii noose
"
    );
}

fn lenny() -> () {
    println!("( ͡° ͜ʖ ͡°)");
}

fn noose() -> () {
    println!(
        "
     |\\|
     |\\|
     |\\|
     |\\|
     |\\|
     |\\|
     |_| 
    (___)
    (___)
    (___)
    (___)
    (___)
    (___)
  ,(/   \\),
 ('/     \\')
('/       \\')
|/|       |/|
|/|       |/|
|/|       |/|
(-\\       /-)
 \\-\\     /-/
  \\-\\___/-/ 
   '-----'
"
    );
}

fn shrekascii() -> () {
    println!(
        "
                      _____ 
                   ,-'     `._ 
                 ,'           `.        ,-. 
               ,'               \\       ),.\\ 
     ,.       /                  \\     /(  \\; 
    /'\\\\     ,o.        ,ooooo.   \\  ,'  `-') 
    )) )`. d8P\"Y8.    ,8P\"\"\"\"\"Y8.  `'  .--\"' 
   (`-'   `Y'  `Y8    dP       `'     / 
    `----.(   __ `    ,' ,---.       ( 
           ),--.`.   (  ;,---.        ) 
          / \\O_,' )   \\  \\O_,'        | 
         ;  `-- ,'       `---'        | 
         |    -'         `.           | 
        _;    ,            )          : 
     _.'|     `.:._   ,.::\" `..       | 
  --'   |   .'     \"\"\"         `      |`. 
        |  :;      :   :     _.       |`.`.-'--. 
        |  ' .     :   :__.,'|/       |  \\ 
        `     \\--.__.-'|_|_|-/        /   ) 
         \\     \\_   `--^\"__,'        ,    | 
          ;  `    `--^---'          ,'     | 
          \\  `                    /      / 
           \\   `    _ _          / 
            \\           `       / 
             \\           '    ,' 
              `.       ,   _,' 
                `-.___.---' 
"
    );
}

fn navyseal() -> () {
    println!("
What the fuck did you just fucking say about me, you little bitch? I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You're fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that's just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little \"clever\" comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn't, you didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You're fucking dead, kiddo.");
}

fn comsym() -> () {
    println!(
        "
░░░░░░░░░░░░░░░█████▄▄▄▄░░░░░░░░░░░ 
░░░░░░░▄▄▄▄▄▄▄░░░░░▀▀▀████▄▄░░░░░░░ 
░░░░░▄███████▀░░░░░░░░░▀▀████▄░░░░░ 
░░░▄███████▀░░░░░░░░░░░░░░▀████▄░░░ 
░▄█████████░░░░░░░░░░░░░░░░░▀███▄░░ 
████████████▄░░░░░░░░░░░░░░░░▀███▄░ 
░▀███▀▀░░▀█████▄░░░░░░░░░░░░░░████░ 
░░░▀░░░░░░░▀█████▄░░░░░░░░░░░░████░ 
░░░░░░░░░░░░░▀▀████▄░░░░░░░░░░████░ 
░░░░░░░░░░░░░░░░▀████▄░░░░░░░░████░ 
░░░░░░░░░░░░░░░░░░▀████▄▄░░░░▄████░ 
░░░░░░░░░░░░░░░░░░░░▀█████▄░▄████▀░ 
░░░░░░░░░░░░░░░░░░░░░░▀█████████▀░░ 
░░░░░▄██▄░░░░░░░░░░░░░░░███████▀░░░ 
░░░▄███████▄▄▄░░░░░░▄▄██████████▄░░ 
░░▀████▀▀██████████████████▀▀████░░ 
░░░░▀▀░░░░░▀▀▀████████▀▀▀░░░░░▀▀░░░"
    );
}
