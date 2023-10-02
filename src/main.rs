// grid constants
const DEFAULT_GRID_BORDER_SIZE: u32 = 32;
const DEFAULT_GRID_SIZE: u32 = 2048;
const DEFAULT_GRID_CELL_SIZE: u32 = 640;

// symbols constants
// const DEFAULT_CROSS_SIZE : u32 = 512;
// const DEFAULT_CIRCLE_SIZE : u32 = 512;

// // origins of the cells
// const DEFAULT_CELL_ORIGINS : [(u32, u32); 9] = [
//     // cell number * 32 + (cell number - 1) * cell size
//     (32, 32),
//     (704, 32),
//     (1376, 32),
//     (32, 704),
//     (704, 704),
//     (1376, 704),
//     (32, 1376),
//     (704, 1376),
//     (1376, 1376)
// ];

#[derive(Copy, Clone)]
struct FRect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl FRect {
    fn new(x: f64, y: f64, w: f64, h: f64) -> FRect {
        FRect { x, y, w, h }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum PlayerId {
    Player1,
    Player2,
}

struct Grid {
    grid: [Option<PlayerId>; 9],
}
struct Player {
    id: PlayerId,
    played: bool,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            grid: [Option::None; 9],
        }
    }

    fn is_full(&self) -> bool {
        !self.grid.contains(&Option::None)
    }
}

impl Player {
    fn new(id: PlayerId) -> Player {
        Player {
            id: id,
            played: false,
        }
    }

    fn has_won(&self, grid: &Grid) -> bool {
        (grid.grid[0] == Some(self.id)
            && grid.grid[1] == Some(self.id)
            && grid.grid[2] == Some(self.id))
            || (grid.grid[3] == Some(self.id)
                && grid.grid[4] == Some(self.id)
                && grid.grid[5] == Some(self.id))
            || (grid.grid[6] == Some(self.id)
                && grid.grid[7] == Some(self.id)
                && grid.grid[8] == Some(self.id))
            || (grid.grid[0] == Some(self.id)
                && grid.grid[3] == Some(self.id)
                && grid.grid[6] == Some(self.id))
            || (grid.grid[1] == Some(self.id)
                && grid.grid[4] == Some(self.id)
                && grid.grid[7] == Some(self.id))
            || (grid.grid[2] == Some(self.id)
                && grid.grid[5] == Some(self.id)
                && grid.grid[8] == Some(self.id))
            || (grid.grid[0] == Some(self.id)
                && grid.grid[4] == Some(self.id)
                && grid.grid[8] == Some(self.id))
            || (grid.grid[2] == Some(self.id)
                && grid.grid[4] == Some(self.id)
                && grid.grid[6] == Some(self.id))
    }
}

fn main() -> anyhow::Result<()> {
    let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;

    let sdl_ttf_context = sdl2::ttf::init()?;

    let video_subsystem = sdl_context.video().map_err(anyhow::Error::msg)?;

    let mut window = video_subsystem
        .window("Tic-Tac-Toe using sdl2 crate", 640, 640)
        .position_centered()
        .resizable()
        .build()?;
    window.set_minimum_size(300u32, 300u32)?;

    let mut canvas = window.into_canvas().accelerated().build()?;

    let mut event_pump = sdl_context.event_pump().map_err(anyhow::Error::msg)?;

    let texture_creator = canvas.texture_creator();

    let mut grid_dst_frect = FRect::new(0.0, 0.0, 0.0, 0.0);
    let min = if canvas.window().drawable_size().0 < canvas.window().drawable_size().1 {
        canvas.window().drawable_size().0
    } else {
        canvas.window().drawable_size().1
    };
    grid_dst_frect.w = 0.8 * min as f64;
    grid_dst_frect.h = grid_dst_frect.w;
    grid_dst_frect.x = (canvas.window().drawable_size().0 as f64 / 2.0) - (grid_dst_frect.w / 2.0);
    grid_dst_frect.y = (canvas.window().drawable_size().1 as f64 / 2.0) - (grid_dst_frect.h / 2.0);
    let mut grid_dst_rect = sdl2::rect::Rect::new(
        grid_dst_frect.x.round() as i32,
        grid_dst_frect.y.round() as i32,
        grid_dst_frect.w.round() as u32,
        grid_dst_frect.h.round() as u32,
    );

    let grid_surface: sdl2::surface::Surface =
        sdl2::image::LoadSurface::from_file(&std::path::Path::new("./tic-tac-toe_grid.png"))
            .map_err(anyhow::Error::msg)?;
    let grid_texture = texture_creator.create_texture_from_surface(grid_surface)?;

    let mut cells_collisions_frects: [FRect; 9] = [FRect::new(0.0, 0.0, 0.0, 0.0); 9];
    for i in 0..3 {
        for j in 0..3 {
            let size_factor = grid_dst_rect.w as f64 / DEFAULT_GRID_SIZE as f64;
            cells_collisions_frects[i * 3 + j].w = size_factor * DEFAULT_GRID_CELL_SIZE as f64;
            cells_collisions_frects[i * 3 + j].h = cells_collisions_frects[i * 3 + j].w;
            cells_collisions_frects[i * 3 + j].x = grid_dst_rect.x as f64
                + (j as f64 + 1.0) * (size_factor * DEFAULT_GRID_BORDER_SIZE as f64)
                + j as f64 * (size_factor * DEFAULT_GRID_CELL_SIZE as f64);
            cells_collisions_frects[i * 3 + j].y = grid_dst_rect.y as f64
                + (i as f64 + 1.0) * (size_factor * DEFAULT_GRID_BORDER_SIZE as f64)
                + i as f64 * (size_factor * DEFAULT_GRID_CELL_SIZE as f64);
        }
    }
    let mut cells_collisions_rects: [sdl2::rect::Rect; 9] = [sdl2::rect::Rect::new(0, 0, 0, 0); 9];
    for i in 0..9 {
        cells_collisions_rects[i].w = cells_collisions_frects[i].w.round() as i32;
        cells_collisions_rects[i].h = cells_collisions_frects[i].h.round() as i32;
        cells_collisions_rects[i].x = cells_collisions_frects[i].x.round() as i32;
        cells_collisions_rects[i].y = cells_collisions_frects[i].y.round() as i32;
    }

    let mut cells_dst_frects: [FRect; 9] = [FRect::new(0.0, 0.0, 0.0, 0.0); 9];
    for i in 0..3 {
        for j in 0..3 {
            let size_factor = grid_dst_rect.w as f64 / DEFAULT_GRID_SIZE as f64;
            cells_dst_frects[i * 3 + j].w = 0.9 * size_factor * DEFAULT_GRID_CELL_SIZE as f64;
            cells_dst_frects[i * 3 + j].h = cells_dst_frects[i * 3 + j].w;
            cells_dst_frects[i * 3 + j].x = grid_dst_rect.x as f64
                + (j as f64 + 1.0) * (size_factor * DEFAULT_GRID_BORDER_SIZE as f64)
                + j as f64 * (size_factor * DEFAULT_GRID_CELL_SIZE as f64)
                + (cells_collisions_frects[i * 3 + j].w / 2.0)
                - (cells_dst_frects[i * 3 + j].w / 2.0);
            cells_dst_frects[i * 3 + j].y = grid_dst_rect.y as f64
                + (i as f64 + 1.0) * (size_factor * DEFAULT_GRID_BORDER_SIZE as f64)
                + i as f64 * (size_factor * DEFAULT_GRID_CELL_SIZE as f64)
                + (cells_collisions_frects[i * 3 + j].h / 2.0)
                - (cells_dst_frects[i * 3 + j].h / 2.0);
        }
    }
    let mut cells_dst_rects: [sdl2::rect::Rect; 9] = [sdl2::rect::Rect::new(0, 0, 0, 0); 9];
    for i in 0..9 {
        cells_dst_rects[i].w = cells_dst_frects[i].w.round() as i32;
        cells_dst_rects[i].h = cells_dst_frects[i].h.round() as i32;
        cells_dst_rects[i].x = cells_dst_frects[i].x.round() as i32;
        cells_dst_rects[i].y = cells_dst_frects[i].y.round() as i32;
    }

    let symbols_surfaces: [sdl2::surface::Surface; 2] = [
        sdl2::image::LoadSurface::from_file("cross.png").map_err(anyhow::Error::msg)?,
        sdl2::image::LoadSurface::from_file("circle.png").map_err(anyhow::Error::msg)?,
    ];
    let symbols_textures = [
        texture_creator.create_texture_from_surface(&symbols_surfaces[0])?,
        texture_creator.create_texture_from_surface(&symbols_surfaces[1])?,
    ];

    let font = sdl_ttf_context
        .load_font(
            std::path::Path::new(&String::from("./proxima_nova.ttf")),
            20u16,
        )
        .map_err(anyhow::Error::msg)?;

    let mut text_surface = font
        .render("Player 1, it's your turn !")
        .blended(sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF))?;
    let mut text_texture = texture_creator.create_texture_from_surface(&text_surface)?;
    let mut text_rect = text_surface.as_mut().rect();

    text_rect.x =
        ((canvas.window().drawable_size().0 as f64 / 2.0) - (text_rect.w as f64 / 2.0)) as i32;
    text_rect.y = ((grid_dst_rect.y as f64 / 2.0) - (text_rect.h as f64 / 2.0)) as i32;

    let mut grid = Grid::new();
    let mut players = [
        Player::new(PlayerId::Player1),
        Player::new(PlayerId::Player2),
    ];
    let mut current_player = PlayerId::Player1 as usize;
    let mut game_over = false;

    let mut display_collision_boxes = false;
    let mut display_drawing_boxes = false;

    'running: loop {
        // Event handling
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                Event::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => {
                    use sdl2::keyboard::Keycode;
                    match keycode {
                        Some(Keycode::F11) => {
                            use sdl2::video::FullscreenType;
                            if canvas.window().fullscreen_state() == FullscreenType::Off {
                                canvas
                                    .window_mut()
                                    .set_fullscreen(FullscreenType::Desktop)
                                    .map_err(anyhow::Error::msg)?;
                            } else {
                                canvas
                                    .window_mut()
                                    .set_fullscreen(FullscreenType::Off)
                                    .map_err(anyhow::Error::msg)?;
                            }
                        }

                        Some(Keycode::Space) => {
                            grid = Grid::new();
                            current_player = PlayerId::Player1 as usize;
                            players[current_player].played = false;
                            text_surface = font
                                .render("Player 1, it's your turn !")
                                .blended(sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF))?;
                            text_texture =
                                texture_creator.create_texture_from_surface(&text_surface)?;
                            text_rect = text_surface.as_mut().rect();
                            text_rect.x = ((canvas.window().drawable_size().0 as f64 / 2.0)
                                - (text_rect.w as f64 / 2.0))
                                as i32;
                            text_rect.y = ((grid_dst_rect.y as f64 / 2.0)
                                - (text_rect.h as f64 / 2.0))
                                as i32;
                            game_over = false;
                        }

                        // toggle debugging display of cells clickable area
                        Some(Keycode::F1) => {
                            display_collision_boxes =
                                if display_collision_boxes { false } else { true };
                        }

                        // toggle debugging display of symbols rendering area
                        Some(Keycode::F2) => {
                            display_drawing_boxes =
                                if display_drawing_boxes { false } else { true };
                        }

                        _ => (),
                    }
                }

                Event::Window { win_event, .. } => {
                    use sdl2::event::WindowEvent;
                    match win_event {
                        // scale each elements on screen
                        WindowEvent::Resized(w, h) => {
                            let min = if w < h { w } else { h };

                            grid_dst_frect.w = 0.8 * min as f64;
                            grid_dst_frect.h = 0.8 * min as f64;
                            grid_dst_frect.x = (w as f64 / 2.0) - (grid_dst_frect.w as f64 / 2.0);
                            grid_dst_frect.y = (h as f64 / 2.0) - (grid_dst_frect.h as f64 / 2.0);

                            grid_dst_rect.x = grid_dst_frect.x.round() as i32;
                            grid_dst_rect.y = grid_dst_frect.y.round() as i32;
                            grid_dst_rect.w = grid_dst_frect.w.round() as i32;
                            grid_dst_rect.h = grid_dst_frect.h.round() as i32;

                            text_rect.x = ((w as f64 / 2.0) - (text_rect.w as f64 / 2.0)) as i32;
                            text_rect.y = ((grid_dst_rect.y as f64 / 2.0)
                                - (text_rect.h as f64 / 2.0))
                                as i32;

                            for i in 0..3 {
                                for j in 0..3 {
                                    let size_factor =
                                        grid_dst_rect.w as f64 / DEFAULT_GRID_SIZE as f64;
                                    cells_collisions_frects[i * 3 + j].w =
                                        size_factor * DEFAULT_GRID_CELL_SIZE as f64;
                                    cells_collisions_frects[i * 3 + j].h =
                                        cells_collisions_frects[i * 3 + j].w as f64;
                                    cells_collisions_frects[i * 3 + j].x = grid_dst_frect.x as f64
                                        + ((j as f64 + 1.0) as f64
                                            * size_factor
                                            * DEFAULT_GRID_BORDER_SIZE as f64)
                                        + (j as f64 * size_factor * DEFAULT_GRID_CELL_SIZE as f64);
                                    cells_collisions_frects[i * 3 + j].y = grid_dst_frect.y as f64
                                        + ((i as f64 + 1.0) as f64
                                            * size_factor
                                            * DEFAULT_GRID_BORDER_SIZE as f64)
                                        + (i as f64 * size_factor * DEFAULT_GRID_CELL_SIZE as f64);
                                }
                            }

                            for i in 0..9 {
                                cells_collisions_rects[i].w =
                                    cells_collisions_frects[i].w.round() as i32;
                                cells_collisions_rects[i].h =
                                    cells_collisions_frects[i].h.round() as i32;
                                cells_collisions_rects[i].x =
                                    cells_collisions_frects[i].x.round() as i32;
                                cells_collisions_rects[i].y =
                                    cells_collisions_frects[i].y.round() as i32;
                            }

                            for i in 0..3 {
                                for j in 0..3 {
                                    let size_factor =
                                        grid_dst_rect.w as f64 / DEFAULT_GRID_SIZE as f64;
                                    cells_dst_frects[i * 3 + j].w =
                                        0.9 * size_factor * DEFAULT_GRID_CELL_SIZE as f64;
                                    cells_dst_frects[i * 3 + j].h =
                                        cells_dst_frects[i * 3 + j].w as f64;
                                    cells_dst_frects[i * 3 + j].x = grid_dst_frect.x as f64
                                        + ((j as f64 + 1.0) as f64
                                            * size_factor
                                            * DEFAULT_GRID_BORDER_SIZE as f64)
                                        + (j as f64 * size_factor * DEFAULT_GRID_CELL_SIZE as f64)
                                        + (cells_collisions_frects[i * 3 + j].w / 2.0)
                                        - (cells_dst_frects[i * 3 + j].w / 2.0);
                                    cells_dst_frects[i * 3 + j].y = grid_dst_frect.y as f64
                                        + ((i as f64 + 1.0) as f64
                                            * size_factor
                                            * DEFAULT_GRID_BORDER_SIZE as f64)
                                        + (i as f64 * size_factor * DEFAULT_GRID_CELL_SIZE as f64)
                                        + (cells_collisions_frects[i * 3 + j].h / 2.0)
                                        - (cells_dst_frects[i * 3 + j].h / 2.0);
                                }
                            }

                            for i in 0..9 {
                                cells_dst_rects[i].w = cells_dst_frects[i].w.round() as i32;
                                cells_dst_rects[i].h = cells_dst_frects[i].h.round() as i32;
                                cells_dst_rects[i].x = cells_dst_frects[i].x.round() as i32;
                                cells_dst_rects[i].y = cells_dst_frects[i].y.round() as i32;
                            }
                        }

                        _ => (),
                    }
                }

                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    use sdl2::mouse::MouseButton;
                    match mouse_btn {
                        MouseButton::Left => {
                            for i in 0..9 {
                                if x >= cells_collisions_rects[i].x
                                    && x <= cells_collisions_rects[i].x
                                        + cells_collisions_rects[i].w
                                    && y >= cells_collisions_rects[i].y
                                    && y <= cells_collisions_rects[i].y
                                        + cells_collisions_rects[i].h
                                {
                                    if !game_over {
                                        if grid.grid[i] == Option::None {
                                            grid.grid[i] = Some(players[current_player].id);
                                            players[current_player].played = true;
                                        } else {
                                            text_surface =
                                                font.render("Cell already used !").blended(
                                                    sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF),
                                                )?;
                                            text_texture = texture_creator
                                                .create_texture_from_surface(&text_surface)?;
                                            text_rect = text_surface.as_mut().rect();
                                            text_rect.x =
                                                ((canvas.window().drawable_size().0 as f64 / 2.0)
                                                    - (text_rect.w as f64 / 2.0))
                                                    as i32;
                                            text_rect.y = ((grid_dst_rect.y as f64 / 2.0)
                                                - (text_rect.h as f64 / 2.0))
                                                as i32;
                                        }
                                    }
                                }
                            }
                        }

                        _ => (),
                    }
                }

                _ => (),
            }
        }

        // Update
        if players[current_player].has_won(&grid) {
            text_surface = font
                .render(&format!(
                    "Player {} has won !",
                    players[current_player].id as u32 + 1
                ))
                .blended(sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF))?;
            text_texture = texture_creator.create_texture_from_surface(&text_surface)?;
            text_rect = text_surface.as_mut().rect();
            text_rect.x = ((canvas.window().drawable_size().0 as f64 / 2.0)
                - (text_rect.w as f64 / 2.0)) as i32;
            text_rect.y = ((grid_dst_rect.y as f64 / 2.0) - (text_rect.h as f64 / 2.0)) as i32;
            game_over = true;
        }

        if !players[PlayerId::Player1 as usize].has_won(&grid)
            && !players[PlayerId::Player2 as usize].has_won(&grid)
            && grid.is_full()
        {
            text_surface = font
                .render("Draw !")
                .blended(sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF))?;
            text_texture = texture_creator.create_texture_from_surface(&text_surface)?;
            text_rect = text_surface.as_mut().rect();
            text_rect.x = ((canvas.window().drawable_size().0 as f64 / 2.0)
                - (text_rect.w as f64 / 2.0)) as i32;
            text_rect.y = ((grid_dst_rect.y as f64 / 2.0) - (text_rect.h as f64 / 2.0)) as i32;
            game_over = true;
        }

        if players[current_player].played && !game_over {
            players[current_player].played = false;
            current_player = match &players[current_player].id {
                PlayerId::Player1 => PlayerId::Player2 as usize,
                PlayerId::Player2 => PlayerId::Player1 as usize,
            };
            text_surface = font
                .render(&format!(
                    "Player {}, it's you're turn !",
                    players[current_player].id as u32 + 1
                ))
                .blended(sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF))?;
            text_texture = texture_creator.create_texture_from_surface(&text_surface)?;
            text_rect = text_surface.as_mut().rect();
            text_rect.x = ((canvas.window().drawable_size().0 as f64 / 2.0)
                - (text_rect.w as f64 / 2.0)) as i32;
            text_rect.y = ((grid_dst_rect.y as f64 / 2.0) - (text_rect.h as f64 / 2.0)) as i32;
        }

        // Draw
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0x11, 0x11, 0x11));
        canvas.clear();
        canvas
            .copy(&text_texture, None, text_rect)
            .map_err(anyhow::Error::msg)?;
        canvas
            .copy(&grid_texture, None, grid_dst_rect)
            .map_err(anyhow::Error::msg)?;

        // simple draw of the collision rect

        if display_collision_boxes {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0xFF, 0x00, 0x00));
            canvas
                .draw_rects(&cells_collisions_rects)
                .map_err(anyhow::Error::msg)?;
        }

        // simple draw of the drawing area of the symbol
        if display_drawing_boxes {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0x00, 0xFF, 0x00));
            canvas
                .draw_rects(&cells_dst_rects)
                .map_err(anyhow::Error::msg)?;
        }

        for i in 0..9 {
            if grid.grid[i] != Option::None {
                canvas
                    .copy(
                        &symbols_textures[grid.grid[i].unwrap() as usize],
                        None,
                        cells_dst_rects[i],
                    )
                    .map_err(anyhow::Error::msg)?;
            }
        }

        canvas.present();
    }

    Ok(())
}
