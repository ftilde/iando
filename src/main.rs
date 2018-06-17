#![feature(proc_macro)]
extern crate quine_derive;
use quine_derive::quine;

use std::env;

#[quine(FMT_BYTES)]
fn fmt_bytes(name: &str, text: &[u8], w: &mut ::std::fmt::Write) -> ::std::fmt::Result {
    let sp = " ";
    write!(w, "const{}{}:&'static[u8] = &[", sp, name)?;
    for b in text {
        write!(w, "{},", b)?;
    }
    write!(w, "];")
}

#[quine(EXP)]
fn exp(name: &str, text: &[u8], w: &mut ::std::fmt::Write) -> ::std::fmt::Result {
    fmt_bytes(name, text, w)?;
    write!(w, "{}", ::std::str::from_utf8(text).unwrap())
}

#[quine(FND_TOK)]
fn fnd_tok(text: &str) -> Vec<(&str, usize)> {
    let mut out = Vec::new();
    {
        let mut beg = 0;
        let mut in_s = false;
        let mut in_q = false;
        let mut qtd = false;
        let mut b_lt = false;
        let chars = text.chars().collect::<Vec<_>>();
        let mut p_tok = |beg, end| {
            out.push((::std::str::from_utf8(&text.as_bytes()[beg..end]).unwrap(), end-beg));
        };
        for (i,c) in chars.iter().enumerate() {
            let c = *c;
            if in_s {
                if c == '"' && !qtd {
                    in_s = false;
                    p_tok(beg, i+1);
                    beg = i+1;
                }
            } else if in_q {
                if c == '\'' && !qtd {
                    in_q = false;
                    p_tok(beg, i+1);
                    beg = i+1;
                }
            } else {
                match c {
                    '}' | '{' | ',' | '[' | ']' | ' ' | ';' | '(' | ')' | '<' => {
                        if beg != i {
                            p_tok(beg, i);
                        }
                        p_tok(i, i+1);
                        beg = i+1;
                    },
                    '\'' if !b_lt => {
                        in_q = true;
                    }
                    '"' => {
                        in_s = true;
                    }
                    _ => {},
                }
            }
            qtd = c == '\\' && !qtd;
            b_lt = (c == '&' || c == '<') && !qtd;
        }
        if beg != chars.len() {
            p_tok(beg, chars.len());
        }
    }
    out
}

#[quine(EXP_COM)]
fn exp_com(w: &mut ::std::fmt::Write) -> ::std::fmt::Result {
    macro_rules! exp_all {
        ( $w:expr, $( $x:ident ),* ) => {
            {
                $(
                    exp(stringify!($x), $x, $w)?;
                 )*
            }
        };
    }

    exp_all!(w, EXP_COM, FMT_BYTES, EXP, WCONST, HCONST, FND_TOK, FILL_W_TOK, RAST);
    Ok(())
}

//const FILL_W_TOK: &'static[u8] = &[];
#[quine(FILL_W_TOK)]
fn fill_w_tok<'a, I: Iterator<Item=(&'a str, usize)>> (mut space: usize, tokens: &mut ::std::iter::Peekable<I>) {
    let mut l_tok = Vec::new();
    loop {
        let next_w = {
            tokens.peek().unwrap().1
        };
        assert!(next_w > 0, "invalid width");
        if next_w > space {
            break;
        }
        let tok = tokens.next().unwrap();
        l_tok.push(tok);
        space -= next_w;
    }
    match l_tok.len() {
        0 => {
            for _ in 0..space {
                print!(" ");
            }
        },
        1 => {
            let (s, _) = l_tok.first().unwrap();
            print!("{}", s);
            for _ in 0..space {
                print!(" ");
            }
        }
        _ => {
            let (last_token, l_tok) = l_tok.split_last().unwrap();
            let mut spnd_sp = 0f32;
            let per_tok = space as f32 / l_tok.len() as f32;
            for (s, _) in l_tok {
                print!("{}", s);
                spnd_sp += per_tok;
                let sp = spnd_sp as usize;
                for _ in 0..sp {
                    print!(" ");
                }
                space -= sp;
                spnd_sp -= sp as f32;
            }
            for _ in 0..space {
                print!(" ");
            }
            print!("{}", last_token.0);
        },
    }
}

//const RAST: &'static[u8] = &[];
#[quine(RAST)]
fn rast<F: Fn(f32, f32) -> bool>(text: &str, inside: F) {
    let mut tokens = fnd_tok(&text).into_iter().chain(::std::iter::repeat(("/**/",4))).peekable();

    for y in 0..H {
        let mut r_beg = None;
        for x in 0..W {
            let dx = (x as f32/W as f32)*2.0-1.0;
            let dy = (y as f32/H as f32)*2.0-1.0;
            match (inside(dx, dy), r_beg) {
                (true, None) => {
                    r_beg = Some(x);
                },
                (false, Some(b)) => {
                    let mut space = x - b as usize;
                    fill_w_tok(space, &mut tokens);
                    r_beg = None;
                },
                (false, _) => {
                    print!(" ");
                },
                (true, _) => { },
            }
        }
        if let Some(b) = r_beg {
            let mut space = W - b as usize;
            fill_w_tok(space, &mut tokens);
        }
        println!("");
    }
}

const D: &'static[u8] = &[84,104,105,115,32,105,115,32,97,110,32,105,110,116,114,111,110,32,116,104,97,116,32,104,97,115,32,110,111,32,117,115,101,32,119,104,97,116,115,111,101,118,101,114,44,32,98,117,116,32,105,116,32,105,115,32,115,116,105,108,108,32,114,101,112,108,105,99,97,116,101,100,32,111,118,101,114,32,97,110,100,32,111,118,101,114,32,98,121,32,98,111,116,104,32,116,104,101,32,105,32,97,110,100,32,116,104,101,32,111,32,112,114,111,103,114,97,109,46,32,70,117,110,110,121,32,115,116,117,102,102,44,32,105,115,110,39,116,32,105,116,63];

#[quine(WCONST)]
const W: usize = 300;

#[quine(HCONST)]
const H: usize = 230;

//const I: &'static[u8] = &[];
#[quine(I, fn_i)]
fn fn_i() -> ::std::fmt::Result {
    let mut text = String::new();

    exp("O", O, &mut text)?;
    fmt_bytes("D", D, &mut text)?;
    exp_com(&mut text)?;
    text.push_str("/*IO*/");
    fmt_bytes("I", I, &mut text)?;


    rast(&text, |dx, dy| {
        let r_o_x = 0.9005000;
        let r_o_y = 1.001000;
        let r_o_x2 = r_o_x*r_o_x;
        let r_o_y2 = r_o_y*r_o_y;
        let r_i_x = 0.6410000;
        let r_i_y = 0.9210000;
        let r_i_x2 = r_i_x*r_i_x;
        let r_i_y2 = r_i_y*r_i_y;
        let dx2 = dx*dx;
        let dy2 = dy*dy;
        dx2/r_i_x2 + dy2/r_i_y2 > 1.0 && dx2/r_o_x2 + dy2/r_o_y2 < 1.0
    });
    Ok(())
}

//const O: &'static[u8] = &[];
#[quine(O, fn_o)]
fn fn_o() -> ::std::fmt::Result {
    let mut text = String::new();

    exp("I", I, &mut text)?;
    exp_com(&mut text)?;
    text.push_str("/*OI*/");
    fmt_bytes("D", D, &mut text)?;
    fmt_bytes("O", O, &mut text)?;

    //text.push_str("/*THIS IS I*/");

    rast(&text, |dx, dy| {
        let h =  1.0000;
        let lh = 0.7450;
        let w =  0.6250;
        let lw = 0.1740;

        let dx = dx.abs();
        let dy = dy.abs();

        let c_x = w;
        let c_y = lh;
        let e_w = w-lw;
        let e_h = (h-lh)*0.543;
        let d_x = c_x - dx;
        let d_y = c_y - dy;
        (dy > lh && dx < w || dx < lw) && ((d_x*d_x)/(e_w*e_w) + (d_y*d_y)/(e_h*e_h)) > 1.00
    });
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("i") => fn_i().unwrap(),
        Some("o") => fn_o().unwrap(),
        Some(_) => panic!("Invalid argument (i or o)"),
        None => panic!("Provide at least one argument (i or o)"),
    }
}
