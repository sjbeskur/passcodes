extern crate rand;
extern crate ansi_term;

use ansi_term::{Style};


const PASSCODE_LEN: usize = 8;

fn main() {

    let style = Style::new().bold();
    for i in (0..1000000) {
        let mut password = gen_string(PASSCODE_LEN);
        password.insert(PASSCODE_LEN/2, '-');
        println!("{}", style.paint( password));
    }
}

fn gen_string(len: usize) -> String{
    use rand::Rng;
    const CHARSET: &[u8] = b"ACDEFGHIJKLMNPQRSTUVWXYZ\
                            23456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..len)
        .map(|_| {
            let i = rng.gen_range(0, CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(i) }) //(i is in the range of CHARSET)
            //char::from({ *CHARSET.get(i).unwrap() }) // safe but slower 
        }).collect();
    password

}
