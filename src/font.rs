
pub const CHAR_WIDTH:usize  = 8;
pub const CHAR_HEIGHT:usize = 10;
pub const CHAR_SPACE:usize  = 2;
pub const LINE_SPACE:usize  = 2;

type Glyph = [u8;10];
pub type Font = [Glyph;128];

pub fn get() -> Font
{
	let mut font = [[0;10];128];

	font[b'0' as usize ][0] = 0b01111110;
	font[b'0' as usize ][1] = 0b11111111;
	font[b'0' as usize ][2] = 0b11000011;
	font[b'0' as usize ][3] = 0b11000011;
	font[b'0' as usize ][4] = 0b11000011;
	font[b'0' as usize ][5] = 0b11000011;
	font[b'0' as usize ][6] = 0b11000011;
	font[b'0' as usize ][7] = 0b11000011;
	font[b'0' as usize ][8] = 0b11111111;
	font[b'0' as usize ][9] = 0b01111110;

	font[b'1' as usize ][0] = 0b00111000;
	font[b'1' as usize ][1] = 0b11111000;
	font[b'1' as usize ][2] = 0b11111000;
	font[b'1' as usize ][3] = 0b00111000;
	font[b'1' as usize ][4] = 0b00111000;
	font[b'1' as usize ][5] = 0b00111000;
	font[b'1' as usize ][6] = 0b00111000;
	font[b'1' as usize ][7] = 0b00111000;
	font[b'1' as usize ][8] = 0b11111111;
	font[b'1' as usize ][9] = 0b11111111;

	font[b'2' as usize ][0] = 0b00111100;
	font[b'2' as usize ][1] = 0b11111111;
	font[b'2' as usize ][2] = 0b11100111;
	font[b'2' as usize ][3] = 0b00000111;
	font[b'2' as usize ][4] = 0b00001110;
	font[b'2' as usize ][5] = 0b00011100;
	font[b'2' as usize ][6] = 0b00111000;
	font[b'2' as usize ][7] = 0b01110000;
	font[b'2' as usize ][8] = 0b11111111;
	font[b'2' as usize ][9] = 0b11111111;

	font[b'3' as usize ][0] = 0b00111100;
	font[b'3' as usize ][1] = 0b11111111;
	font[b'3' as usize ][2] = 0b11100111;
	font[b'3' as usize ][3] = 0b00000111;
	font[b'3' as usize ][4] = 0b01111110;
	font[b'3' as usize ][5] = 0b01111110;
	font[b'3' as usize ][6] = 0b00001111;
	font[b'3' as usize ][7] = 0b11100111;
	font[b'3' as usize ][8] = 0b11111111;
	font[b'3' as usize ][9] = 0b00111100;

	font[b'4' as usize ][0] = 0b00011110;
	font[b'4' as usize ][1] = 0b00111110;
	font[b'4' as usize ][2] = 0b11100110;
	font[b'4' as usize ][3] = 0b11000110;
	font[b'4' as usize ][4] = 0b11111111;
	font[b'4' as usize ][5] = 0b01111111;
	font[b'4' as usize ][6] = 0b00001110;
	font[b'4' as usize ][7] = 0b00001110;
	font[b'4' as usize ][8] = 0b00001110;
	font[b'4' as usize ][9] = 0b00001110;

	font[b'5' as usize ][0] = 0b11111111;
	font[b'5' as usize ][1] = 0b11111111;
	font[b'5' as usize ][2] = 0b11100000;
	font[b'5' as usize ][3] = 0b11100000;
	font[b'5' as usize ][4] = 0b01111000;
	font[b'5' as usize ][5] = 0b00111100;
	font[b'5' as usize ][6] = 0b00001111;
	font[b'5' as usize ][7] = 0b11100111;
	font[b'5' as usize ][8] = 0b11111111;
	font[b'5' as usize ][9] = 0b01111110;

	font[b'6' as usize ][0] = 0b00001111;
	font[b'6' as usize ][1] = 0b00111100;
	font[b'6' as usize ][2] = 0b01111000;
	font[b'6' as usize ][3] = 0b11110000;
	font[b'6' as usize ][4] = 0b11111110;
	font[b'6' as usize ][5] = 0b11111111;
	font[b'6' as usize ][6] = 0b11100111;
	font[b'6' as usize ][7] = 0b11100111;
	font[b'6' as usize ][8] = 0b11111111;
	font[b'6' as usize ][9] = 0b01111110;

	font[b'7' as usize ][0] = 0b11111111;
	font[b'7' as usize ][1] = 0b11111111;
	font[b'7' as usize ][2] = 0b11100111;
	font[b'7' as usize ][3] = 0b00001110;
	font[b'7' as usize ][4] = 0b00011100;
	font[b'7' as usize ][5] = 0b00111000;
	font[b'7' as usize ][6] = 0b00111000;
	font[b'7' as usize ][7] = 0b00111000;
	font[b'7' as usize ][8] = 0b00111000;
	font[b'7' as usize ][9] = 0b00111000;

	font[b'8' as usize ][0] = 0b00111100;
	font[b'8' as usize ][1] = 0b11100111;
	font[b'8' as usize ][2] = 0b11000011;
	font[b'8' as usize ][3] = 0b11100111;
	font[b'8' as usize ][4] = 0b00111100;
	font[b'8' as usize ][5] = 0b00111100;
	font[b'8' as usize ][6] = 0b11100111;
	font[b'8' as usize ][7] = 0b11000011;
	font[b'8' as usize ][8] = 0b11100111;
	font[b'8' as usize ][9] = 0b00111100;

	font[b'9' as usize ][0] = 0b01111110;
	font[b'9' as usize ][1] = 0b11111111;
	font[b'9' as usize ][2] = 0b11100111;
	font[b'9' as usize ][3] = 0b11100111;
	font[b'9' as usize ][4] = 0b11111111;
	font[b'9' as usize ][5] = 0b01111111;
	font[b'9' as usize ][6] = 0b00001111;
	font[b'9' as usize ][7] = 0b00011110;
	font[b'9' as usize ][8] = 0b00111100;
	font[b'9' as usize ][9] = 0b11110000;

	font[b'A' as usize ][0] = 0b00111100;
	font[b'A' as usize ][1] = 0b01100110;
	font[b'A' as usize ][2] = 0b11100111;
	font[b'A' as usize ][3] = 0b11100111;
	font[b'A' as usize ][4] = 0b11111111;
	font[b'A' as usize ][5] = 0b11111111;
	font[b'A' as usize ][6] = 0b11100111;
	font[b'A' as usize ][7] = 0b11100111;
	font[b'A' as usize ][8] = 0b11100111;
	font[b'A' as usize ][9] = 0b11100111;

	font[b'B' as usize ][0] = 0b11111110;
	font[b'B' as usize ][1] = 0b11100011;
	font[b'B' as usize ][2] = 0b11100011;
	font[b'B' as usize ][3] = 0b11100011;
	font[b'B' as usize ][4] = 0b11111110;
	font[b'B' as usize ][5] = 0b11111110;
	font[b'B' as usize ][6] = 0b11100011;
	font[b'B' as usize ][7] = 0b11100011;
	font[b'B' as usize ][8] = 0b11100011;
	font[b'B' as usize ][9] = 0b11111110;

	font[b'C' as usize ][0] = 0b01111110;
	font[b'C' as usize ][1] = 0b11100111;
	font[b'C' as usize ][2] = 0b11100000;
	font[b'C' as usize ][3] = 0b11100000;
	font[b'C' as usize ][4] = 0b11100000;
	font[b'C' as usize ][5] = 0b11100000;
	font[b'C' as usize ][6] = 0b11100000;
	font[b'C' as usize ][7] = 0b11100000;
	font[b'C' as usize ][8] = 0b11100111;
	font[b'C' as usize ][9] = 0b01111110;

	font[b'D' as usize ][0] = 0b11111100;
	font[b'D' as usize ][1] = 0b11100110;
	font[b'D' as usize ][2] = 0b11100011;
	font[b'D' as usize ][3] = 0b11100011;
	font[b'D' as usize ][4] = 0b11100011;
	font[b'D' as usize ][5] = 0b11100011;
	font[b'D' as usize ][6] = 0b11100011;
	font[b'D' as usize ][7] = 0b11100011;
	font[b'D' as usize ][8] = 0b11100110;
	font[b'D' as usize ][9] = 0b11111100;

	font[b'E' as usize ][0] = 0b11111111;
	font[b'E' as usize ][1] = 0b11111111;
	font[b'E' as usize ][2] = 0b11100001;
	font[b'E' as usize ][3] = 0b11100000;
	font[b'E' as usize ][4] = 0b11111110;
	font[b'E' as usize ][5] = 0b11111110;
	font[b'E' as usize ][6] = 0b11100000;
	font[b'E' as usize ][7] = 0b11100001;
	font[b'E' as usize ][8] = 0b11111111;
	font[b'E' as usize ][9] = 0b11111111;

	font[b'F' as usize ][0] = 0b11111111;
	font[b'F' as usize ][1] = 0b11111111;
	font[b'F' as usize ][2] = 0b11100001;
	font[b'F' as usize ][3] = 0b11100000;
	font[b'F' as usize ][4] = 0b11111110;
	font[b'F' as usize ][5] = 0b11111110;
	font[b'F' as usize ][6] = 0b11100000;
	font[b'F' as usize ][7] = 0b11100000;
	font[b'F' as usize ][8] = 0b11100000;
	font[b'F' as usize ][9] = 0b11100000; 

	font[b'G' as usize ][0] = 0b00001111;
	font[b'G' as usize ][1] = 0b00111111;
	font[b'G' as usize ][2] = 0b01110000;
	font[b'G' as usize ][3] = 0b01100000;
	font[b'G' as usize ][4] = 0b01100111;
	font[b'G' as usize ][5] = 0b01100111;
	font[b'G' as usize ][6] = 0b01100001;
	font[b'G' as usize ][7] = 0b01110001;
	font[b'G' as usize ][8] = 0b00111111;
	font[b'G' as usize ][9] = 0b00011110; 

	font[b'H' as usize ][0] = 0b11100111;
	font[b'H' as usize ][1] = 0b11100111;
	font[b'H' as usize ][2] = 0b11100111;
	font[b'H' as usize ][3] = 0b11100111;
	font[b'H' as usize ][4] = 0b11111111;
	font[b'H' as usize ][5] = 0b11111111;
	font[b'H' as usize ][6] = 0b11100111;
	font[b'H' as usize ][7] = 0b11100111;
	font[b'H' as usize ][8] = 0b11100111;
	font[b'H' as usize ][9] = 0b11100111; 

	font[b'I' as usize ][0] = 0b00011100;
	font[b'I' as usize ][1] = 0b00011100;
	font[b'I' as usize ][2] = 0b00000000;
	font[b'I' as usize ][3] = 0b00000000;
	font[b'I' as usize ][4] = 0b00011100;
	font[b'I' as usize ][5] = 0b00011100;
	font[b'I' as usize ][6] = 0b00011100;
	font[b'I' as usize ][7] = 0b00011100;
	font[b'I' as usize ][8] = 0b00011100;
	font[b'I' as usize ][9] = 0b00011100; 

	font[b'J' as usize ][0] = 0b00000111;
	font[b'J' as usize ][1] = 0b00000111;
	font[b'J' as usize ][2] = 0b00000111;
	font[b'J' as usize ][3] = 0b00000111;
	font[b'J' as usize ][4] = 0b00000111;
	font[b'J' as usize ][5] = 0b00000111;
	font[b'J' as usize ][6] = 0b11100111;
	font[b'J' as usize ][7] = 0b11100111;
	font[b'J' as usize ][8] = 0b11111111;
	font[b'J' as usize ][9] = 0b11111111; 

	font[b'K' as usize ][0] = 0b11100111;
	font[b'K' as usize ][1] = 0b11101110;
	font[b'K' as usize ][2] = 0b11111100;
	font[b'K' as usize ][3] = 0b11111000;
	font[b'K' as usize ][4] = 0b11110000;
	font[b'K' as usize ][5] = 0b11110000;
	font[b'K' as usize ][6] = 0b11111000;
	font[b'K' as usize ][7] = 0b11111100;
	font[b'K' as usize ][8] = 0b11101110;
	font[b'K' as usize ][9] = 0b11100111; 

	font[b'L' as usize ][0] = 0b11100000;
	font[b'L' as usize ][1] = 0b11100000;
	font[b'L' as usize ][2] = 0b11100000;
	font[b'L' as usize ][3] = 0b11100000;
	font[b'L' as usize ][4] = 0b11100000;
	font[b'L' as usize ][5] = 0b11100000;
	font[b'L' as usize ][6] = 0b11100000;
	font[b'L' as usize ][7] = 0b11100000;
	font[b'L' as usize ][8] = 0b11111111;
	font[b'L' as usize ][9] = 0b11111111; 

	font[b'M' as usize ][0] = 0b11100111;
	font[b'M' as usize ][1] = 0b11100111;
	font[b'M' as usize ][2] = 0b11111111;
	font[b'M' as usize ][3] = 0b11011011;
	font[b'M' as usize ][4] = 0b11011011;
	font[b'M' as usize ][5] = 0b11000011;
	font[b'M' as usize ][6] = 0b11000011;
	font[b'M' as usize ][7] = 0b11000011;
	font[b'M' as usize ][8] = 0b11000011;
	font[b'M' as usize ][9] = 0b11000011; 

	font[b'N' as usize ][0] = 0b11100011;
	font[b'N' as usize ][1] = 0b11100011;
	font[b'N' as usize ][2] = 0b11110011;
	font[b'N' as usize ][3] = 0b11110011;
	font[b'N' as usize ][4] = 0b11011011;
	font[b'N' as usize ][5] = 0b11011011;
	font[b'N' as usize ][6] = 0b11001111;
	font[b'N' as usize ][7] = 0b11001111;
	font[b'N' as usize ][8] = 0b11000111;
	font[b'N' as usize ][9] = 0b11000111; 

	font[b'O' as usize ][0] = 0b11111111;
	font[b'O' as usize ][1] = 0b11111111;
	font[b'O' as usize ][2] = 0b11000111;
	font[b'O' as usize ][3] = 0b11000111;
	font[b'O' as usize ][4] = 0b11000111;
	font[b'O' as usize ][5] = 0b11000111;
	font[b'O' as usize ][6] = 0b11000111;
	font[b'O' as usize ][7] = 0b11000111;
	font[b'O' as usize ][8] = 0b11111111;
	font[b'O' as usize ][9] = 0b11111111; 

	font[b'P' as usize ][0] = 0b11111100;
	font[b'P' as usize ][1] = 0b11111111;
	font[b'P' as usize ][2] = 0b11100111;
	font[b'P' as usize ][3] = 0b11100111;
	font[b'P' as usize ][4] = 0b11100111;
	font[b'P' as usize ][5] = 0b11111110;
	font[b'P' as usize ][6] = 0b11111000;
	font[b'P' as usize ][7] = 0b11100000;
	font[b'P' as usize ][8] = 0b11100000;
	font[b'P' as usize ][9] = 0b11100000; 

	font[b'Q' as usize ][0] = 0b11111111;
	font[b'Q' as usize ][1] = 0b11111111;
	font[b'Q' as usize ][2] = 0b11000011;
	font[b'Q' as usize ][3] = 0b11000011;
	font[b'Q' as usize ][4] = 0b11000011;
	font[b'Q' as usize ][5] = 0b11001011;
	font[b'Q' as usize ][6] = 0b11001011;
	font[b'Q' as usize ][7] = 0b11001110;
	font[b'Q' as usize ][8] = 0b11111111;
	font[b'Q' as usize ][9] = 0b11110111; 

	font[b'R' as usize ][0] = 0b11111100;
	font[b'R' as usize ][1] = 0b11111111;
	font[b'R' as usize ][2] = 0b11100111;
	font[b'R' as usize ][3] = 0b11100111;
	font[b'R' as usize ][4] = 0b11100111;
	font[b'R' as usize ][5] = 0b11111110;
	font[b'R' as usize ][6] = 0b11111000;
	font[b'R' as usize ][7] = 0b11111100;
	font[b'R' as usize ][8] = 0b11101110;
	font[b'R' as usize ][9] = 0b11100111; 

	font[b'S' as usize ][0] = 0b01111111;
	font[b'S' as usize ][1] = 0b11111111;
	font[b'S' as usize ][2] = 0b11000000;
	font[b'S' as usize ][3] = 0b11000000;
	font[b'S' as usize ][4] = 0b11111110;
	font[b'S' as usize ][5] = 0b01111111;
	font[b'S' as usize ][6] = 0b00000111;
	font[b'S' as usize ][7] = 0b00000111;
	font[b'S' as usize ][8] = 0b11111111;
	font[b'S' as usize ][9] = 0b11111110;

	font[b'T' as usize ][0] = 0b11111111;
	font[b'T' as usize ][1] = 0b11111111;
	font[b'T' as usize ][2] = 0b00011000;
	font[b'T' as usize ][3] = 0b00011000;
	font[b'T' as usize ][4] = 0b00011000;
	font[b'T' as usize ][5] = 0b00011000;
	font[b'T' as usize ][6] = 0b00011000;
	font[b'T' as usize ][7] = 0b00011000;
	font[b'T' as usize ][8] = 0b00011000;
	font[b'T' as usize ][9] = 0b00011000;  

	font[b'U' as usize ][0] = 0b11000011;
	font[b'U' as usize ][1] = 0b11000011;
	font[b'U' as usize ][2] = 0b11000011;
	font[b'U' as usize ][3] = 0b11000011;
	font[b'U' as usize ][4] = 0b11000011;
	font[b'U' as usize ][5] = 0b11000011;
	font[b'U' as usize ][6] = 0b11000011;
	font[b'U' as usize ][7] = 0b11000011;
	font[b'U' as usize ][8] = 0b11111111;
	font[b'U' as usize ][9] = 0b11111111; 

	font[b'V' as usize ][0] = 0b11000001;
	font[b'V' as usize ][1] = 0b11000011;
	font[b'V' as usize ][2] = 0b11100111;
	font[b'V' as usize ][3] = 0b11100111;
	font[b'V' as usize ][4] = 0b11100111;
	font[b'V' as usize ][5] = 0b11111111;
	font[b'V' as usize ][6] = 0b00111110;
	font[b'V' as usize ][7] = 0b00111100;
	font[b'V' as usize ][8] = 0b00011000;
	font[b'V' as usize ][9] = 0b00011000; 

	font[b'W' as usize ][0] = 0b10000001;
	font[b'W' as usize ][1] = 0b10000001;
	font[b'W' as usize ][2] = 0b10000001;
	font[b'W' as usize ][3] = 0b11011011;
	font[b'W' as usize ][4] = 0b01011010;
	font[b'W' as usize ][5] = 0b01011010;
	font[b'W' as usize ][6] = 0b01111110;
	font[b'W' as usize ][7] = 0b00100100;
	font[b'W' as usize ][8] = 0b00100100;
	font[b'W' as usize ][9] = 0b00100100; 

	font[b'X' as usize ][0] = 0b11000011;
	font[b'X' as usize ][1] = 0b11000011;
	font[b'X' as usize ][2] = 0b00100100;
	font[b'X' as usize ][3] = 0b00111100;
	font[b'X' as usize ][4] = 0b00011000;
	font[b'X' as usize ][5] = 0b00011000;
	font[b'X' as usize ][6] = 0b00111100;
	font[b'X' as usize ][7] = 0b00100100;
	font[b'X' as usize ][8] = 0b11000011;
	font[b'X' as usize ][9] = 0b11000011; 

	font[b'Y' as usize ][0] = 0b11100111;
	font[b'Y' as usize ][1] = 0b11100111;
	font[b'Y' as usize ][2] = 0b11100111;
	font[b'Y' as usize ][3] = 0b11100111;
	font[b'Y' as usize ][4] = 0b11100111;
	font[b'Y' as usize ][5] = 0b00011100;
	font[b'Y' as usize ][6] = 0b00011100;
	font[b'Y' as usize ][7] = 0b00011100;
	font[b'Y' as usize ][8] = 0b00011100;
	font[b'Y' as usize ][9] = 0b00011100; 

	font[b'Z' as usize ][0] = 0b11111111;
	font[b'Z' as usize ][1] = 0b11111111;
	font[b'Z' as usize ][2] = 0b00000110;
	font[b'Z' as usize ][3] = 0b00000110;
	font[b'Z' as usize ][4] = 0b00011000;
	font[b'Z' as usize ][5] = 0b00011000;
	font[b'Z' as usize ][6] = 0b01100000;
	font[b'Z' as usize ][7] = 0b01100000;
	font[b'Z' as usize ][8] = 0b11111111;
	font[b'Z' as usize ][9] = 0b11111111; 

	return font;
}