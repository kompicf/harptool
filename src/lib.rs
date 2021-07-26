#[derive(Debug)]
struct Scale {
    root: String,
    notes: Vec<String>
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Tuning {
    blow: Vec<Option<usize>>,
    draw: Vec<Option<usize>>,
    bends_half: Vec<Option<usize>>,
    bends_full: Vec<Option<usize>>,
    bends_one_and_half: Vec<Option<usize>>,
    blow_bends_half: Vec<Option<usize>>,
    blow_bends_full: Vec<Option<usize>>,
}

impl Default for Tuning {
    fn default() -> Tuning {
	Tuning {
	    blow: vec![Some(0), Some(4), Some(7), Some(0), Some(4), Some(7), Some(0), Some(4), Some(7), Some(0)],
	    draw: vec![Some(2), Some(7), Some(11), Some(2), Some(5), Some(9), Some(11), Some(2), Some(5), Some(9)],
	    bends_half: vec![Some(1), Some(6), Some(10), Some(1), None, Some(8), None, None, None, None],
	    bends_full: vec![None, Some(5), Some(9), None, None, None, None, None, None, None],
	    bends_one_and_half: vec![None, None, Some(8), None, None, None, None, None, None, None],
	    blow_bends_half: vec![None, None, None, None, None, None, None, Some(3), Some(6), Some(11)],
	    blow_bends_full: vec![None, None, None, None, None, None, None, None, None, Some(10)],
	}
    }
}

impl Tuning {
    pub fn new(top_notes: Vec<usize>, bottom_notes: Vec<usize>) -> Tuning {
	fn is_within_5_semitones(top: usize, bottom: usize) -> bool {
	    (bottom as i32 - top as i32).abs() < 5
	}

	let blow: Vec<Option<usize>> = top_notes.iter().map(|x| Some(*x)).collect();
	let draw: Vec<Option<usize>> = bottom_notes.iter().map(|x| Some(*x)).collect();
	let mut bends_half: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut bends_full: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut bends_one_and_half: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut blow_bends_half: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut blow_bends_full: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];

	for (i, (top, bottom)) in top_notes.iter().zip(bottom_notes.clone()).enumerate() {
	    let mut top = *top;
	    let mut bottom = bottom;

	    if ! is_within_5_semitones(top, bottom) {
		if top > bottom {
		    bottom += 12;
		} else {
		    top += 12;
		}
	    }

	    if bottom > top {
		if bottom - top == 4 {
		    bends_one_and_half.get_mut(i).unwrap().insert((bottom - 3) % 12);
		}
		if bottom - top >= 3 {
		   bends_full.get_mut(i).unwrap().insert((bottom - 2) % 12);
		}
		if bottom - top >= 2 {
		   bends_half.get_mut(i).unwrap().insert((bottom - 1) % 12);
		}
	    } else {
		if top - bottom == 3 {
		    blow_bends_full.get_mut(i).unwrap().insert((top - 2) % 12);
		}
		if top - bottom >= 2 {
		    blow_bends_half.get_mut(i).unwrap().insert((top - 1) % 12);
		}
	    }
	}

	Tuning {
	    blow,
	    draw,
	    bends_half,
	    bends_full,
	    bends_one_and_half,
	    blow_bends_half,
	    blow_bends_full,
	}
    }
}

impl Scale {
    fn new(note: &str) -> Scale {
	let notes = if vec!["C", "G", "D", "A", "E", "B", "F#"].contains(&note) {
	    vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
	} else {
	    vec!["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"]
	};

	let mut pos = notes.iter().position(|&n| n == note).unwrap();
	let mut v = Vec::new();
	for _i in 0..notes.len() {
	    v.push(notes.get(pos % 12).unwrap().to_string());
	    pos += 1;
	}

	Scale {
	    root: note.to_string(),
	    notes: v,
	}
    }

    fn printrow(&self, indices: &Vec<Option<usize>>) {
	//                   0     1    2    3     4    5     6     7    8    9    10   11
	// let notes = vec!["C", "Dd", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
	let notes = &self.notes;

	for i in indices {
	    let n = match *i {
		Some(n) => notes.get(n).unwrap().to_string(),
		None => String::from(" "),
	    };
	    print!("{:width$} ", n, width = 3);
	}
	print!("\n");
    }

    fn printlayout(&self, tuning: &Tuning) {
	print!("{:width$} ", "blow bends full step", width = 20);
	self.printrow(&tuning.blow_bends_full);
	print!("{:width$} ", "blow bends half step", width = 20);
	self.printrow(&tuning.blow_bends_half);
	print!("{:width$} ", "blow", width = 20);
	self.printrow(&tuning.blow);
	println!("{:width$} 1   2   3   4   5   6   7   8   9   10", "",  width = 20);
	print!("{:width$} ", "draw", width = 20);
	self.printrow(&tuning.draw);
	print!("{:width$} ", "bends half step", width = 20);
	self.printrow(&tuning.bends_half);
	print!("{:width$} ", "bends full step", width = 20);
	self.printrow(&tuning.bends_full);
	print!("{:width$} ", "bends 1 1/2 step", width = 20);
	self.printrow(&tuning.bends_one_and_half);
    }
}

pub fn test() {
    let richter = Tuning::default();
    let _country = Tuning {
	draw: vec![Some(2), Some(7), Some(11), Some(2), Some(6), Some(9), Some(11), Some(2), Some(5), Some(9)],
	bends_half: vec![Some(1), Some(6), Some(10), Some(1), Some(5), Some(8), None, None, None],
	..richter.clone()
    };
    let _wilde_tuned = Tuning::new(
	vec![0, 4, 7, 0, 4, 4, 7, 0, 4, 9],
	vec![2, 7, 11, 2, 5, 7, 11, 2, 7, 0],
    );

    let v = Scale::new("C");
    v.printlayout(&_wilde_tuned);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tuning_new() {
	let tuning = Tuning::new(
	    vec![0, 4, 7, 0, 4, 7, 0, 4, 7, 0],
	    vec![2, 7, 11, 2, 5, 9, 11, 2, 5, 9],
	);
	assert_eq!(tuning, Tuning::default());
    }
}
