use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::{thread, time::Duration};

const WIDTH: usize = 80;
const HEIGHT: usize = 22;
const BUFFER_SIZE: usize = WIDTH * HEIGHT;
const LUM_CHARS: &[u8] = b".,-~:;=!*#$@";

fn term_size() -> (usize, usize) {
    Command::new("stty")
        .arg("size")
        .stdin(Stdio::inherit())
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            let mut p = s.split_whitespace();
            Some((p.next()?.parse().ok()?, p.next()?.parse().ok()?))
        })
        .unwrap_or((24, 80))
}

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let (rows, cols) = term_size();
    let start_row = rows.saturating_sub(HEIGHT) / 2 + 1;
    let start_col = cols.saturating_sub(WIDTH) / 2 + 1;
    print!("\x1b[2J\x1b[?25l");
    loop {
        let mut output = [b' '; BUFFER_SIZE];
        let mut zbuffer = [0.0_f64; BUFFER_SIZE];
        let (sin_a, cos_a) = a.sin_cos();
        let (sin_b, cos_b) = b.sin_cos();
        let mut theta: f64 = 0.0;
        while theta < std::f64::consts::TAU {
            let (sin_t, cos_t) = theta.sin_cos();
            let mut phi: f64 = 0.0;
            while phi < std::f64::consts::TAU {
                let (sin_p, cos_p) = phi.sin_cos();
                let h = cos_t + 2.0;
                let d = 1.0 / (sin_p * h * sin_a + sin_t * cos_a + 5.0);
                let t = sin_p * h * cos_a - sin_t * sin_a;
                let x = (40.0 + 30.0 * d * (cos_p * h * cos_b - t * sin_b)) as isize;
                let y = (11.0 + 15.0 * d * (cos_p * h * sin_b + t * cos_b)) as isize;
                if y >= 0 && (y as usize) < HEIGHT && x >= 0 && (x as usize) < WIDTH {
                    let o = (x as usize) + WIDTH * (y as usize);
                    let n = (8.0
                        * ((sin_t * sin_a - sin_p * cos_t * cos_a) * cos_b
                            - sin_p * cos_t * sin_a
                            - sin_t * cos_a
                            - cos_p * cos_t * sin_b)) as isize;
                    if d > zbuffer[o] {
                        zbuffer[o] = d;
                        let char_idx = n.max(0).min((LUM_CHARS.len() - 1) as isize) as usize;
                        output[o] = LUM_CHARS[char_idx];
                    }
                }
                phi += 0.02;
            }
            theta += 0.07;
        }
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for (i, row) in output.chunks(WIDTH).enumerate() {
            let _ = write!(handle, "\x1b[{};{}H", start_row + i, start_col);
            let _ = handle.write_all(row);
        }
        let _ = handle.flush();
        a += 0.04;
        b += 0.02;
        thread::sleep(Duration::from_millis(30));
    }
}
