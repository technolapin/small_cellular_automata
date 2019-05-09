/**
Pas d'array, pas de problèmes!
Tout en vec
*/

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
    neighbors: Vec<Vec<Position>>,
    w: usize,
    h: usize,
    size: usize
}

impl Grid
{
    fn new (w: usize, h: usize) -> Self
    {
        let mut datas = Vec::new();
        for i in 0..(w*h)
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

    // return true as long as there is change
    fn step_life(&mut self, ngh: &Neighborhood) -> bool 
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

        !changes.is_empty()
    }
    
    fn transition_life(&self, i: usize, ngh: std::slice::Iter<Position>) -> Cell
    {
        let mut s = 0;
        for pos in ngh
        {
            s += self.datas[pos_to_index(*pos, self.w)];
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
    
}



impl Neighborhood
{
    fn new_moore(w: usize, h: usize) -> Self
    {
        let mut ngh = Vec::new();
        let mut n = 0;
        for j in 0..h
        {
            for i in 0..w
            {
                n = pos_to_index((i, j), w);
                ngh.push(Vec::new());
                for x in (i.max(1)-1)..(i.min(w-2)+2)
                {
                    for y in (j.max(1)-1)..(j.min(h-2)+2)
                    {
                        if x != i || y != j
                        {
                            ngh[n].push((x, y));
                        }
                    }
                }
            }
        }
        
        Neighborhood
        {
            neighbors: ngh,
            w: w,
            h: h,
            size: w*h
        }
    }


}

fn pos_to_index((i, j): Position, w: usize) -> usize
{
    j*w+i
}

fn index_to_pos(n: usize, w: usize) -> (usize, usize)
{
    (n%w, n/w)
}

fn main() {

    let mut grid = Grid::new(100, 100);
    let moore_ngh = Neighborhood::new_moore(100, 100);

    grid.set_data(1, (2, 1));
    grid.set_data(1, (1, 0));
    grid.set_data(1, (2, 2)); 
    grid.set_data(1, (1, 2));
    grid.set_data(1, (0, 2));

    
    //grid.print();
    
    while grid.step_life(&moore_ngh)
    {
        //grid.print();
    }

    
}


