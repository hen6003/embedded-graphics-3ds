use ctru::console::Console;
use ctru::gfx::Screen as _;
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use ctru::Gfx;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct SnakeSegment {
    x: u8,
    y: u8,
    visible: bool,
}

struct Snake {
    segments: Vec<SnakeSegment>,
    direction: Direction,
}

impl Snake {
    fn new(direction: Direction) -> Self {
        let mut ret = Self {
            segments: vec![SnakeSegment {
                x: 25,
                y: 15,
                visible: true,
            }],
            direction,
        };

        ret.add_segment();
        ret.add_segment();

        ret
    }

    fn tick(&mut self) {
        for i in (0..self.segments.len()).rev() {
            if i != 0 {
                self.segments[i] = self.segments[i - 1];
            } else {
                let head = &mut self.segments[i];

                match self.direction {
                    Direction::Up => head.y -= 1,
                    Direction::Right => head.x += 1,
                    Direction::Down => head.y += 1,
                    Direction::Left => head.x -= 1,
                }
            }
        }
    }

    fn add_segment(&mut self) {
        self.segments.push(SnakeSegment {
            x: 0,
            y: 0,
            visible: false,
        });
    }
}

struct Apple {
    x: u8,
    y: u8,
}

impl Apple {
    fn rand(&mut self) {
        let mut buf = [0; 2];

        getrandom::getrandom(&mut buf).unwrap();

        self.x = buf[0] % 51;
        self.y = buf[1] % 30;

        if self.x == 0 {
            self.x += 1;
        }

        if self.y == 0 {
            self.y += 1;
        }
    }
}

const REFRESH_RATE: u8 = 30;

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    let mut apple = Apple { x: 0, y: 0 };
    apple.rand();
    let mut snake = Snake::new(Direction::Right);
    let mut frame = 0;
    let mut gameover = false;
    let mut paused = false;

    // Main loop
    while apt.main_loop() {
        frame += 1;

        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        let keys_down = hid.keys_down();
        let keys_held = hid.keys_held();

        if keys_down.contains(KeyPad::KEY_SELECT) {
            break;
        }
        
        if keys_down.contains(KeyPad::KEY_START) {
            if gameover {
                gameover = false;
                
                // Reset
                snake = Snake::new(Direction::Right);
                apple.rand();
            } else {
                paused = !paused;
            }
        }

        if keys_held.intersects(KeyPad::KEY_UP) && snake.direction != Direction::Down {
            snake.direction = Direction::Up;
        }

        if keys_held.intersects(KeyPad::KEY_RIGHT) && snake.direction != Direction::Left {
            snake.direction = Direction::Right;
        }

        if keys_held.intersects(KeyPad::KEY_DOWN) && snake.direction != Direction::Up {
            snake.direction = Direction::Down;
        }

        if keys_held.intersects(KeyPad::KEY_LEFT) && snake.direction != Direction::Right {
            snake.direction = Direction::Left;
        }

        if frame == REFRESH_RATE {
            println!("\x1b[2J");            

            if gameover {
                println!("\x1b[15;20HGame over!");
            } else if paused {
                println!("\x1b[15;22HPaused!");            
            } else {
                snake.tick();
                let head = &snake.segments[0];

                // Check for game overs
                if head.x <= 0 || head.x > 50 || head.y <= 0 || head.y > 29 {
                    gameover = true;
                }
                
                for i in 1..snake.segments.len() {
                    if *head == snake.segments[i] {
                        gameover = true;
                    }
                }
                
                // Check for snake eating apple
                if head.x == apple.x && head.y == apple.y {
                    snake.add_segment();
                    apple.rand();
                }

                for segment in &snake.segments {
                    if segment.visible {
                        println!("\x1b[{};{}H#", segment.y, segment.x)
                    }
                }

                println!("\x1b[{};{}H\x1b[31m@\x1b[0m", apple.y, apple.x);
            }

            frame = 0;
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
