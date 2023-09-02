//-----------------------------------------------------------------------------
//--- Терминальная программа тестирования и отработки методов работы со 
//--- структурами и привязанными к ней методами управления 
//--- Библиотека lib.rs содержит описание структуры и ее методов
//---
//--- Разработан вариант программы Life со стандартными правилами Конвея
//--- Предполагается развитие программы с возможностью задания новых правил,
//--- размещением первичных популляций, возможно с полями переменных правил..
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN/Life.git,
//--- e-mail: ln.KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   13.08.2023 22:09:00
//-----------------------------------------------------------------------------

use std::{env};

//--- библиотека lib.rs содержит: ---------------------------------------------
//--- [generation() posev() census() displayworld() run() populate_from_file()]
use life::*;
//-----------------------------------------------------------------------------

fn main() {
	let args: Vec<String> = env::args().collect();

	let about = About {
		firstname: "Leonid".to_string(),
		secondname: "Nikolaevich".to_string(),
		mainname: "Kornilov".to_string(),	
		author: "Kornilov LN (Starmark)".to_string(),
		github: "https://github.com/KornilovLN/Life.git".to_string(),
		e_mail: "ln.KornilovStar@gmail.com".to_string(),
		datetime: "14.08.2023 13:10:00".to_string(),
	};

	//--- Структура (методы описаны в lib.rs)
	let field = Field {
		width : COLUMNS,
		height : RAWS,
		tor : true,
		algo : 0,
	};

	//--- массив под внешнее поле жизни
	let mut world = [[0u8; COLUMNS]; RAWS];

	if args.len() < 2 {	
		//--- заполнение поля случайными клетками		
		if Field::get_tor(&field) { 
			Field::posev_tor(&field, &mut world);
		}else{
			Field::posev(&field, &mut world);
		}
	}else{				
		//--- взять засев из файла
		let filename = env::args().nth(1).unwrap();
		world = Field::populate_from_file(filename);
	}

	
	about.Out();
	about.Target();	
	about.Waiter(1);

	//--- поле жизни
    field.run(world);	

	
}

