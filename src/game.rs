use crate::OccupiedError;

#[derive(Debug)]
pub struct Game {
    length: usize,
    board: Vec<Vec<u8>>,
    players: u8,
    current_player: u8
}

impl Game {
    pub fn new(len: usize, players: u8) -> Self {
        if len % 2 == 0 { panic!("Length must be odd"); }
        let mut line = Vec::with_capacity(len);
        line.resize(len, 0);

	    let mut board = Vec::with_capacity(len);
        board.resize(len, line);
        Self { length: len, board, players, current_player: 1 }
    }

    pub fn length(&self) -> usize { self.length }

    pub fn place(&mut self, x: usize, y: usize) -> Result<(), OccupiedError> {
        if self.board[y][x] == 0 {
            self.board[y][x] = self.current_player;
            self.new_player();
            Ok(())
        } else {
            Err(OccupiedError::new())
        }
    }

    pub fn current_player(&self) -> u8 {
        self.current_player
    }

    pub fn to_str(&self, num: u8) -> String {
        if num == 0 {return "□".to_string()}
        if self.players <= 2 { if num == 1 {return "x".to_string();} else if num == 2 {return "o".to_string();} }
        num.to_string().to_owned()
    }

    pub fn show_current(&self) {
        self.show("current")
    }

    pub fn show(&self, tagline: &str) {

        println!("{} board", tagline);
        println!("");

        let mut rows: Vec<String> = Vec::with_capacity(self.length + 1);
        let mut build = "0".to_string();
        for i in 1..(self.length + 1) {
            build = format!("{} {}", build, &i.to_string());
        }
        rows.push(build);

        for y in 1..self.length+1 {
            let mut build = y.to_string();
            for x in 0..self.length {
                build = format!("{} {}", build, &self.to_str(self.board[y - 1][x]));
            }
            rows.push(build);
        }
        for row in rows {
            println!("{}", row);
        }
    }

    fn new_player(&mut self) {
        self.current_player = (self.current_player + 1) % (self.players + 1);
        if self.current_player == 0 { self.current_player = 1; }
    }

    pub fn check(&self) -> i8 {
        /*
         * this function keeps track of whether a change has occurred in any specific row, column, or diagonal in the board
         * 0 => no value defined, overridden when iterating over the board => no win condition yet
         * -1 => more than one player has occupied a slot => no win condition here
         * _  => a player has occupied every slot => that player wins
         */
        
        let mut tie = true;
        for y in 0..self.length {
            for x in 0..self.length {
                if self.board[y][x] != 0 { tie = false; break; }
            }
        }
        if tie { return -2; }

        let mut vert: Vec<i8> = vec![-2; self.length];

        for y in 0..self.length {
            let line = &self.board[y];
            let mut hori: i8 = -2;
            for x in 0..line.len() {
                let slot = line[x];
                if hori == -2 { hori = slot as i8; }
                if hori != slot as i8 { hori = -1; }

                if vert[x] == -2 { vert[x] = slot as i8; }
                if vert[x] != slot as i8 { vert[x] = -1; }
            }
            if hori > 0 { return hori; }
        }

        for col in vert {
            if col > 0 { return col; }
        }

        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut checker = -2i8;
        for _i in 0..self.length {
            if checker == -2 { checker = self.board[y][x] as i8; }
            if checker != self.board[y][x] as i8 {
                checker = -1;
                break;
            }

            x = x + 1;
            y = y + 1;
        };
        if checker > 0 { return checker; };
        let mut x: usize = self.length - 1;
        let mut y: usize = 0;
        let mut checker = -2i8;
        for _i in 1..self.length {
            if checker == -2 { checker = self.board[y][x] as i8; }
            if checker != self.board[y][x] as i8 { checker = -1; break; }

            x = x - 1;
            y = y + 1;
        };

        if checker > 0 { return checker; };
        -1
    }
}
