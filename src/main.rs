/**
Pas d'array, pas de problèmes!
Tout en vec
*/

extern crate image;
extern crate rand;
extern crate orbclient;

use orbclient::{Color, EventOption, GraphicsPath, Mode, Renderer, Window};

use rand::Rng;

type Cell = u8;
type Position = (usize, usize);

struct Grid
{
    datas: Vec<Cell>,
    w: usize,
    h: usize,
    size: usize
}


struct Neighborhood
{
    neighbors: Vec<Vec<usize>>,
}


impl Grid
{
    fn new (w: usize, h: usize) -> Self
    {
        let mut datas = Vec::new();
        for _ in 0..(w*h)
        {
            datas.push(0);
        }
        Grid{
            datas: datas,
            w: w,
            h: h,
            size: w*h
        }
    }



    
    fn set_data(&mut self, value: Cell, pos: Position)
    {
        self.datas[pos_to_index(pos, self.w)] = value;
    }

    fn get_data(&self, pos: Position) -> Cell
    {
        self.datas[pos_to_index(pos, self.w)]
    }

    fn print(&self)
    {
        for j in 0..self.h
        {
            for i in 0..self.w
            {
                print!("{} ", self.get_data((i, j)));
            }
            println!("");
        }
    }
    fn print_formated(&self, code: &Vec<&str>)
    {
        for j in 0..self.h
        {
            for i in 0..self.w
            {
                let v = self.get_data((i, j)) as usize;
                if v < code.len()
                {
                    print!("{} ", code[v]);
                }
                else
                {
                    print!("{} ", v);
                }
            }
            println!("");
        }
    }

    // return true as long as there is change
    fn step_life(&mut self, ngh: &Neighborhood) -> Vec<(usize, Cell)>
    {
        let mut changes = Vec::new();
        for i in 0..self.size
        {
            let new_value = self.transition_life(i, ngh.neighbors[i].iter());
            if new_value != self.datas[i]
            {
                changes.push((i, new_value));
            }
        }

        for (i, new_value) in changes.iter()
        {
            self.datas[*i] = *new_value;
        }

        changes
    }
    
    fn transition_life(&self, i: usize, ngh: std::slice::Iter<usize>) -> Cell
    {
        let mut s = 0;
        for i in ngh
        {
            s += self.datas[*i];
        }
        
        if s < 2 || s > 3
        {
            0
        }
        else if s==3
        {
            1
        }
        else
        {
            self.datas[i]
        }
    }

    fn make_image(&self, palette: &Vec<[u8; 4]>)  -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>
    {
        let mut img = image::RgbaImage::new(self.w as u32, self.h as u32);
        for i in 0..self.size
        {
            let (x, y) = index_to_pos(i, self.w);
            let value = self.datas[i] as usize;
            if value < palette.len()
            {
                img.put_pixel(x as u32, y as u32, image::Rgba(palette[value]));
            }
            else
            {
                img.put_pixel(x as u32, y as u32, image::Rgba([0, 0, 0, 0]));
            }
        }
        img
    }
    fn update_image(&self, img: &mut image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>, changes: &Vec<(usize, Cell)>, palette: &Vec<[u8; 4]>)
    {

        for (i, value) in changes.iter()
        {
            let (x, y) = index_to_pos(*i, self.w);
            if (*value as usize) < palette.len()
            {
                img.put_pixel(x as u32, y as u32, image::Rgba(palette[*value as usize]));
            }
            else
            {
                img.put_pixel(x as u32, y as u32, image::Rgba([0, 255, 0, 0]));
            }
            
        }
    }

    fn randomise(&mut self, min: Cell, max: Cell) //supose que Cell est un entier
    {
        let mut rng = rand::thread_rng();

        for i in 0..self.size
        {
            self.datas[i] = rng.gen_range(min, max);
        }
    }
    fn draw_on_screen(&self,
                      window: &mut orbclient::Window,
                      pas: u32,
                      gap: u32,
                      palette: &Vec<[u8; 4]>)
    {
        for i in 0..self.size
        {
            let (x, y) = index_to_pos(i, self.w);
            if self.datas[i] == 1
            {
                let col = palette[self.datas[i] as usize];
                window.rect((x as u32*pas) as i32, (y as u32*pas) as i32, pas-gap, pas-gap, Color::rgba(col[0], col[1], col[2], 255-col[3]));
            }

        }
        window.sync();
    }
    fn update_screen(&self,
                     changes: &Vec<(usize, Cell)>,
                     window: &mut orbclient::Window,
                     pas: u32,
                     gap: u32,
                     palette: &Vec<[u8; 4]>)
    {
        for (i, value) in changes.iter()
        {
            let (x, y) = index_to_pos(*i, self.w);
            if *value == 1
            {
                let col = palette[*value as usize];
                window.rect((x as u32*pas) as i32, (y as u32*pas) as i32, pas-gap, pas-gap, Color::rgba(col[0], col[1], col[2], 255-col[3]));
            }
            else
            {
                window.rect((x as u32*pas) as i32, (y as u32*pas) as i32, pas-gap, pas-gap, Color::rgba(0, 0, 0, 255));

            }

        }
        window.sync();
    }
}



impl Neighborhood
{
    fn new_moore(w: usize, h: usize) -> Self
    {
        let mut ngh = Vec::new();
        for j in 0..h
        {
            for i in 0..w
            {
                let n = pos_to_index((i, j), w);
                ngh.push(Vec::new());
                for x in (i.max(1)-1)..(i.min(w-2)+2)
                {
                    for y in (j.max(1)-1)..(j.min(h-2)+2)
                    {
                        if x != i || y != j
                        {
                            ngh[n].push(pos_to_index((x, y), w));
                        }
                    }
                }
            }
        }
        
        Neighborhood
        {
            neighbors: ngh,
        }
    }


}

fn pos_to_index((i, j): Position, w: usize) -> usize
{
    j*w+i
}

fn index_to_pos(n: usize, w: usize) -> Position
{
    (n%w, n/w)
}



fn life_catalogue_parse(rle: &str) -> Vec<(usize, usize)>
{
    let mut x = 0;
    let mut y = 0;
    let mut mul = 1;

    let mut cells = Vec::new();
    
    for c in rle.chars()
    {
        if c == 'b'
        {
            x += mul;
            mul = 1;
        } else
        if c == '$'
        {
            y += mul;
            x = 0;
            mul = 1;
        } else
        if c == 'o'
        {
            while mul > 1
            {
                mul -= 1;
                cells.push((x, y));
                x += 1;
            }
        } else
        {
            let parse = c.
        }
    }

    cells
}

fn main() {

    let gap = 1;
    let (width, height) = orbclient::get_display_size().unwrap();

    let (w, h) = (width as usize/4, height as usize/4);
    //let (w, h) = (80, 80);

    
    let (ratio_w, ratio_h) = (width as f32/w as f32, height as f32/h as f32);

    let pas = ratio_w.min(ratio_h).floor().max(1.) as u32;
    let window_width = w as u32*pas;
    let window_height = h as u32*pas;
    let mut window = Window::new_flags(
        0,
        0,
        window_width,
        window_height,
        "TITLE",
        &[
            orbclient::WindowFlag::Transparent,
            orbclient::WindowFlag::Async,
        ]
    )
    .unwrap();


    
    let mut grid = Grid::new(w, h);
    let moore_ngh = Neighborhood::new_moore(w, h);

    let code = vec!["·", "@"];
    /*
    grid.set_data(1, (2, 1));
    grid.set_data(1, (1, 0));
    grid.set_data(1, (2, 2)); 
    grid.set_data(1, (1, 2));
    grid.set_data(1, (0, 2));
    */
    let gun = vec![
        (0usize, 4),
        (0, 5),
        (1, 4),
        (1, 5),

        (13, 2),
        (12, 2),
        (11, 3),
        (10, 4),
        (10, 5),
        (10, 6),
        (11, 7),
        (12, 8),
        (13, 8),

        (14, 5),
        
        (15, 3),
        (15, 7),
        (16, 4),
        (16, 5),
        (16, 6),
        (17, 5),

        (20, 4),
        (20, 3),
        (20, 2),
        (21, 4),
        (21, 3),
        (21, 2),

        (22, 1),
        (22, 5),

        (24, 0),
        (24, 1),

        (24, 5),
        (24, 6),

        (34, 2),
        (34, 3),
        (35, 2),
        (35, 3)

    ];

    for (x, y) in gun.iter()
    {
        grid.set_data(1, (x+1usize, y+1usize));
    }

    for (x, y) in gun.iter()
    {
        grid.set_data(1, (100-x, y+3usize));
    }

//    grid.randomise(0, 2);
   
    let colors = vec![
        [0, 0, 0, 255],
        [160, 130, 0, 0]
    ];
    
    //grid.print();
    
    let mut n = 0;
    let mut img = grid.make_image(&colors);
    img.save(format!("images/frame-{:04}.bmp", n));
    //grid.print_formated(&code);

    
    let mut changes = grid.step_life(&moore_ngh); 


    grid.draw_on_screen(&mut window, pas, gap, &colors);

   'events:  while !changes.is_empty()
    {
        n += 1;
        println!("step {}", n);

//        grid.update_image(&mut img, &changes, &colors);
        //img.save(format!("images/frame-{:04}.bmp", n));
        //grid.print_formated(&code);
        changes = grid.step_life(&moore_ngh);
        grid.update_screen(&changes, &mut window, pas, gap, &colors);

        'nul: for event in window.events() {
            println!("{:?}", event.to_option());
            match event.to_option() {
                EventOption::Quit(_quit_event) => break 'events,
                    
                EventOption::Mouse(evt) =>
                {
                    println!(
                        "At position {:?} pixel color is : {:?}",
                        (evt.x, evt.y),
                        window.getpixel(evt.x, evt.y)
                    );
                    let x = (evt.x as f32 / pas as f32) as usize;
                    let y = (evt.y as f32 / pas as f32) as usize;
                    grid.set_data(
                        1 - grid.get_data((x, y)),
                        (x, y)
                    );
                    
                },
                event_option => println!("{:?}", event_option),
                 
                _ => break 'nul
            }
        }
    }

    
    
}


