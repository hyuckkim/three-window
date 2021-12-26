extern crate piston_window;

use piston_window::*;

struct Win {
    dow: PistonWindow,
    pos: (i32, i32),
}
impl Win {
    fn new(i: usize, x: i32, y: i32) -> Win {
        let win = Win {
            dow: WindowSettings::new(format!("window {}", i), [512, 512])
            .exit_on_esc(true)
            .resizable(false)
            .build::<PistonWindow>()
            .unwrap()
            .position([x, y]),
            pos: (x, y),
        };
        win
    }
}
#[derive(Debug)]
struct Rect {
    x1: i32, y1: i32, x2: i32, y2: i32,
}
impl Rect {
    fn new(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> Rect {
        let xpair = if x1 < x2 {(x1, x2)} else {(x2, x1)};
        let ypair = if y1 < y2 {(y1, y2)} else {(y2, y1)};
        Rect {
            x1: xpair.0, y1: ypair.0, x2: xpair.1, y2: ypair.1
        }
    }
    fn pairs(&self, other: &Rect) -> ((i32, i32), (i32, i32)) {
        ((
            if self.x1 < other.x1 {other.x1} else {self.x1},
            if self.x2 < other.x2 {self.x2} else {other.x2}
            ),
        (
            if self.y1 < other.y1 {other.y1} else {self.y1},
            if self.y2 < other.y2 {self.y2} else {other.y2}
        ))
    }
    fn is_overwrap(&self, other: &Rect) -> bool {
        let (xpair, ypair) = self.pairs(other);
        
        xpair.0 < xpair.1 && ypair.0 < ypair.1
    }
    fn overwrap(&self, other: &Rect) -> Option<Rect> {
        let (xpair, ypair) = self.pairs(other);
        if self.is_overwrap(other) {
            Some(Rect::new(xpair.0, ypair.0, xpair.1, ypair.1))
        } 
        else {
            None
        }
    }
}
impl Copy for Rect { }

impl Clone for Rect {
    fn clone(&self) -> Rect {
        *self
    }
}
fn main() {
    let default_rect = Rect::new(0, 0, 0, 0);
    let color = [
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [1.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [1.0, 0.0, 1.0, 1.0],
        [0.0, 1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    ];

    let mut windows : Vec<Win> = (0..3).into_iter().map(| i | {
        Win::new(i + 1, (i * 600) as i32, 0)
    }).collect();
    let mut rectangles = [Some(Rect{..default_rect}); 7];

    loop {
        let mut any_alive = false;
        for (i, w) in windows.iter_mut().enumerate() {
            if let Some(e) = w.dow.next() {
                let pos = w.dow.get_position().unwrap();
                let rect = Rect::new(pos.x, pos.y, pos.x + 512, pos.y + 512);
                any_alive = true;
                w.pos = (pos.x, pos.y);
                w.dow.draw_2d(&e, |c, g, _device| {
                    clear(color[0], g);
                    for (j, r) in rectangles.iter().enumerate() {
                        if let Some(t) = rect.overwrap(&r.unwrap_or(default_rect)) {
                            let rect = math::margin_rectangle(
                                [(t.x1 - pos.x) as f64, (t.y1 - pos.y) as f64,
                                 (t.x2 - t.x1) as f64, (t.y2 - t.y1) as f64], 
                                0.0);
                            rectangle(color[j + 1], rect, c.transform, g);
                        }
                    }
                });
                if w.dow.should_close() {w.dow.hide(); }
                rectangles[binary(i) - 1] = Some(rect);
            }
            else {
                rectangles[binary(i) - 1] = None;
            }
            rectangles[2] = if let (Some(a), Some(b)) = (rectangles[0], rectangles[1]) { a.overwrap(&b) } else { None };
            rectangles[4] = if let (Some(a), Some(b)) = (rectangles[0], rectangles[3]) { a.overwrap(&b) } else { None };
            rectangles[5] = if let (Some(a), Some(b)) = (rectangles[1], rectangles[3]) { a.overwrap(&b) } else { None };
            rectangles[6] = if let (Some(a), Some(b)) = (rectangles[4], rectangles[5]) { a.overwrap(&b) } else { None };
        }
        if !any_alive { break; }
    }
}
fn binary(number: usize) -> usize {
    1 << number
}