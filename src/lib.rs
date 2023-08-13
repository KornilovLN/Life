extern crate termion;
extern crate rand;

use std::{thread, time};

use termion::clear;
use termion::color;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub const  RAWS: usize = 64;
pub const  COLUMNS: usize = 238;
pub const  RAWS_1: usize = RAWS-1;
pub const  COLUMNS_1: usize = COLUMNS-1;

const  WAITER_MSEC: u64 = 100; 
const  MAXLOOP: u64 = 1500;

pub struct Field {
	pub width: usize,	//--- длина поля
	pub height: usize,	//--- ширина
	pub tor: bool,		//--- тороидальное замыкание поля (true)
	pub algo: u8,		//--- номер алгоритма правил жизни 
					//--- [0 - Конвей, 
					//---  1 - 2 яруса,только ходом ладьи, 
					//---  2 - 2 яруса, ходом слона,
					//---  3 - ходом ферзя]
}

impl Field {
	//--- алгоритм жизни по Конвею
	pub fn generation(&self, _world: [[u8; COLUMNS]; RAWS]) -> [[u8; COLUMNS]; RAWS]
	{
		//--- сгенерим поле на основе предыдущего состояния
		let mut newworld = [[0u8; COLUMNS]; RAWS];
		
		//--- идя по строкам
		for i in 0..self.height-1 {

			//--- а в строке - по столбцам
			for j in 0..self.width-1 {

				//--- счетчик соседей
				let mut count = 0;

				//--- обойти вокруг текущей клетки и посчитать соседей, если они есть
				if i>0 {
					count = count + _world[i-1][j];
				}
				if i>0 && j>0 {
					count = count + _world[i-1][j-1];
				}
				if i>0 && j<self.width-1 {
					count = count + _world[i-1][j+1];
				}
				if i<self.height-1 && j>0 {
					count = count + _world[i+1][j-1];
				}
				if i<self.height-1 {
					count = count + _world[i+1][j];
				}
				if i<self.height-1 && j<self.width-1 {
					count = count + _world[i+1][j+1]
				}
				if j>0 {
					count = count + _world[i][j-1];
				}
				if j<self.width-1 {
					count = count + _world[i][j+1];
				}
				
				if self.tor == true {
				
				//--- если поле тороидальное, то надо учесть подвороты (var 2)
				//if i == 0 && j == 0 {}
				//if i == 0 && j == COLUMNS_1 {}
				//if i == 0 && (j > 0 && j < COLUMNS_1) {}

				//if (i > 0 && i < RAWS_1) && j == 0 {}
				//if (i > 0 && i < RAWS_1) && j == COLUMNS_1 {}
				//if (i > 0 && i < RAWS_1) && (j > 0 && j < COLUMNS_1) {}

				//if (i == RAWS_1) && j == 0 {}
				//if (i == RAWS_1) && j == COLUMNS_1 {}
				//if (i == RAWS_1) && (j > 0 && j < COLUMNS_1) {}
				
				}

				//--- обнулить исследуемую клетку
				newworld[i][j] = 0;

				//--- правило Конвея
				if (count <2) && (_world[i][j] == 1) {
					newworld[i][j] = 0;
				}
				if _world[i][j] == 1 && (count == 2 || count == 3) {
					newworld[i][j] = 1;
				}
				if (_world[i][j] == 0) && (count == 3) {
					newworld[i][j] = 1;
				}
			}

		}

		newworld
	}
}

impl Field {
	//--- Первичный засев поля life перед циклом
	pub fn posev(&self, wrld: &mut[[u8; COLUMNS]; RAWS]) {
		for i in 0..self.height-1 {
			for j in 0..self.width-1 {
				if rand::random() {
					wrld[i][j] = 1;
				} else {
					wrld[i][j] = 0;
				}
			}
		}
	}
}

impl Field {
	//--- подсчет живых клеток
	pub fn census(&self, _world: [[u8; COLUMNS]; RAWS]) -> u64
	{
		let mut count = 0;

		for i in 0..self.height-1 {
			for j in 0..self.width-1 {
				if _world[i][j] == 1
				{
					count += 1;
				}
			}
		}

		count
	}
}

impl Field {
	//--- Отрисовка поля 
	pub fn displayworld(&self, wrld: [[u8; COLUMNS]; RAWS]) {
		println!("{}", clear::All);

		for i in 0..self.height-1 {
			for j in 0..self.width-1 {
				if wrld[i][j] == 1 {
					print!("{red}0", red = color::Fg(color::Red));
				}else{
					print!(" ");
				}
			}

			println!("");
		}
	}
}

impl Field {
	//--- вывод статистики
	pub fn displaystat(generations: &mut u64, quantity: u64) {
		println!("{blue}Population at generation {g} is {c}", 
				  blue = color::Fg(color::Blue), 
				  g = generations, c = quantity);
	}
}

impl Field {
	//--- Собственно, работа - регенерация поля
	pub fn run(&self, mut world: [[u8; COLUMNS]; RAWS]) {
		let mut generations: u64  = 0;

		for _gens in 0..MAXLOOP {
			let temp = Field::generation(self, world);
			world = temp;

			generations += 1;

			Field::displayworld(self, world);
			let quantity: u64 = Field::census(self, world);
			Field::displaystat(&mut generations, quantity);

			thread::sleep(time::Duration::from_millis(WAITER_MSEC));
		}
	}
}
//-------------------------------------------------------------------------------

impl Field {
	pub fn populate_from_file(filename: String) -> [[u8; COLUMNS]; RAWS] {
		let mut newworld = [[0u8; COLUMNS]; RAWS];

		let file = File::open(filename).unwrap();
		let reader = BufReader::new(file);
		let mut pairs: Vec<(usize, usize)> = Vec::new();
		
		for (_index, line) in reader.lines().enumerate() {
			let l = line.unwrap();
			let mut words = l.split_whitespace();
			let left = words.next().unwrap();
			let right = words.next().unwrap();
			pairs.push((left.parse::<usize>().unwrap(),
			right.parse::<usize>().unwrap()));
		}
		
		for i in 0..RAWS_1 {
			for j in 0..COLUMNS_1 {
				newworld[i][j] = 0;
			}	
		}

		for (x,y) in pairs {
			newworld[x][y] = 1;
		}
		
		newworld
	}
}
