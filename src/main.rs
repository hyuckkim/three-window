#![windows_subsystem = "windows"]

extern crate piston_window;

use piston_window::*;

static WINDOW_SIZE: u32 = 512;
struct Win {
    dow: PistonWindow,
    rect: Rect,
}
impl Win {
    fn new(title: &str, x: i32, y: i32) -> Win {
        let win = Win {
            dow: WindowSettings::new(title, [WINDOW_SIZE, WINDOW_SIZE])
                .exit_on_esc(true)
                .resizable(false)
                .build::<PistonWindow>()
                .unwrap()
                .position([x, y]),
            rect: Rect::new(0, 0, 0, 0),
        };
        win
    }
    fn draw(&mut self, e: Event, rects: [Option<Rect>; 7]) {
        let default_rect = Rect::new(0, 0, 0, 0);

        // bit. 1: red / 2: green / 4: blue
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

        self.dow.draw_2d(&e, |c, g, _device| {
            clear(color[0], g); // background is black but nobody see them

            // coloring by each colors
            for (j, r) in rects.iter().enumerate() {
                // overwrap rectangles with window
                if let Some(t) = self.rect.overwrap(&r.unwrap_or(default_rect)) {
                    let rect = math::margin_rectangle(
                        // x, y, w, h (not x2)
                        [
                            (t.a.x - self.rect.a.x) as f64,
                            (t.a.y - self.rect.a.y) as f64,
                            (t.b.x - t.a.x) as f64,
                            (t.b.y - t.a.y) as f64,
                        ],
                        0.0,
                    );
                    rectangle(color[j + 1], rect, c.transform, g);
                }
            }
        });
    }
}
struct Point {
    x: i32,
    y: i32,
}
// rectangle with two point.
struct Rect {
    a: Point,
    b: Point,
}
impl Rect {
    // make rect, x1/y1 is left/up
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
        let xpair = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let ypair = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        Rect {
            a: Point {
                x: xpair.0,
                y: ypair.0,
            },
            b: Point {
                x: xpair.1,
                y: ypair.1,
            },
        }
    }
    // make small rect
    fn pairs(&self, other: &Rect) -> (Point, Point) {
        (
            Point {
                x: if self.a.x < other.a.x {
                    other.a.x
                } else {
                    self.a.x
                },
                y: if self.a.y < other.a.y {
                    other.a.y
                } else {
                    self.a.y
                },
            },
            Point {
                x: if self.b.x < other.b.x {
                    self.b.x
                } else {
                    other.b.x
                },
                y: if self.b.y < other.b.y {
                    self.b.y
                } else {
                    other.b.y
                },
            },
        )
    }
    // is small rect can make real rect
    fn is_overwrap(&self, other: &Rect) -> bool {
        let (a, b) = self.pairs(other);

        a.x < b.x && a.y < b.y
    }
    // make small rect to struct if it can
    fn overwrap(&self, other: &Rect) -> Option<Rect> {
        let (a, b) = self.pairs(other);
        if self.is_overwrap(other) {
            Some(Rect::new(a.x, a.y, b.x, b.y))
        } else {
            None
        }
    }
}
impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}
impl Copy for Rect {}

impl Clone for Rect {
    fn clone(&self) -> Rect {
        *self
    }
}
fn main() {
    let mut windows: Vec<Win> = (0..3)
        .into_iter()
        .map(|i| {
            Win::new(
                &format!("window {}", i + 1),
                (i * (WINDOW_SIZE + 50)) as i32,
                0,
            )
        })
        .collect(); //todo : make them array, this project contains only three windows (r, g, b)
    let mut rectangles: [Option<Rect>; 7] = [None; 7];

    loop {
        let mut any_alive = false;
        for (i, w) in windows.iter_mut().enumerate() {
            // window can draw
            if let Some(e) = w.dow.next() {
                any_alive = true; // for loop

                let pos = w.dow.get_position().unwrap(); // on desktop screen
                w.rect = Rect::new(
                    pos.x,
                    pos.y,
                    pos.x + WINDOW_SIZE as i32,
                    pos.y + WINDOW_SIZE as i32,
                );
                w.draw(e, rectangles);

                if w.dow.should_close() {
                    w.dow.hide();
                } // X button pressed
                rectangles[binary(i) - 1] = Some(w.rect);
            } else {
                rectangles[binary(i) - 1] = None;
            }
        }
        rectangles[2] = if let (Some(a), Some(b)) = (rectangles[0], rectangles[1]) {
            a.overwrap(&b)
        } else {
            None
        }; // overwrap to yellow
        rectangles[4] = if let (Some(a), Some(b)) = (rectangles[0], rectangles[3]) {
            a.overwrap(&b)
        } else {
            None
        }; // overwrap to magenta
        rectangles[5] = if let (Some(a), Some(b)) = (rectangles[1], rectangles[3]) {
            a.overwrap(&b)
        } else {
            None
        }; // overwrap to cyan
        rectangles[6] = if let (Some(a), Some(b)) = (rectangles[4], rectangles[5]) {
            a.overwrap(&b)
        } else {
            None
        }; // overwrap to while (overwrap all)

        if !any_alive {
            break;
        } //all windows cannot draw, then close
    }
}
fn binary(number: usize) -> usize {
    1 << number
}
