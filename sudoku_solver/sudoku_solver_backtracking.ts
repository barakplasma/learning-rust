const expect = require('expect');
type Board = Array<Array<number>>;

class sudoku_solver {
  board: Board;
  empty_location = [0, 0];
  constructor(board: Board) {
    this.board = board;
  }
  solve(): boolean {
    if (!this.find_empty_location()) {
      return true;
    }

    let row = this.empty_location[0];
    let col = this.empty_location[1];

    for (let num = 1; num < 10; num++) {
      if (this.check_location_is_safe(row, col, num)) {
        this.board[row][col] = num;
        if (this.solve()) {
          return true;
        }
        this.board[row][col] = 0;
      }
    }
    return false;
  }

  find_empty_location(): Boolean {
    for (let row = 0; row < 9; row++) {
      for (let col = 0; col < 9; col++) {
        if (this.board[row][col] === 0) {
          this.empty_location[0] = row;
          this.empty_location[1] = col;
          return true;
        }
      }
    }
    return false;
  }

  used_in_row(row: number, num: number): Boolean {
    return this.board[row].includes(num);
  }

  used_in_col(col: number, num: number): Boolean {
    return this.transpose()[col].includes(num);
  }

  transpose() {
    return this.board[0].map((_, c) => this.board.map(r => r[c]));
  }
  used_in_box(boxStartRow: number, boxStartCol: number, num: number): Boolean {
    for (let row = 0; row < 3; row++) {
      for (let col = 0; col < 3; col++) {
        if (this.board[row + boxStartRow]
        [col + boxStartCol] === num) {
          return true;
        }
      }
    }
    return false;
  }

  check_location_is_safe(row: number, col: number, num: number): Boolean {
    return !this.used_in_row(row, num) && !this.used_in_col(col, num) && !this.used_in_box(row - row % 3,
      col - col % 3, num) && this.board[row][col] === 0;
  }
}

function sudoku(puzzle: Board): Board {
  //return the solved puzzle as a 2d array of 9 x 9 
  const solution = new sudoku_solver(puzzle);
  solution.solve();
  return solution.board;
}


// https://www.codewars.com/kata/5296bc77afba8baa690002d7
const puzzle1 = [
  [5, 3, 0, 0, 7, 0, 0, 0, 0],
  [6, 0, 0, 1, 9, 5, 0, 0, 0],
  [0, 9, 8, 0, 0, 0, 0, 6, 0],
  [8, 0, 0, 0, 6, 0, 0, 0, 3],
  [4, 0, 0, 8, 0, 3, 0, 0, 1],
  [7, 0, 0, 0, 2, 0, 0, 0, 6],
  [0, 6, 0, 0, 0, 0, 2, 8, 0],
  [0, 0, 0, 4, 1, 9, 0, 0, 5],
  [0, 0, 0, 0, 8, 0, 0, 7, 9]];

const puzzle2 = [
  [0, 0, 6, 0, 2, 0, 0, 5, 0],
  [0, 0, 2, 0, 0, 0, 1, 9, 4],
  [0, 0, 0, 1, 0, 0, 2, 0, 7],
  [6, 0, 7, 0, 8, 2, 0, 1, 9],
  [0, 8, 5, 0, 7, 0, 0, 3, 0],
  [0, 0, 0, 6, 0, 5, 4, 0, 0],
  [0, 9, 0, 0, 1, 3, 0, 4, 0],
  [0, 0, 1, 0, 0, 9, 0, 0, 0],
  [7, 3, 0, 0, 0, 8, 9, 0, 0]];

// puzzle2.join().replace(/0/g, '.').replace(/,/g, '') /*?*/

var solution = [
  [5, 3, 4, 6, 7, 8, 9, 1, 2],
  [6, 7, 2, 1, 9, 5, 3, 4, 8],
  [1, 9, 8, 3, 4, 2, 5, 6, 7],
  [8, 5, 9, 7, 6, 1, 4, 2, 3],
  [4, 2, 6, 8, 5, 3, 7, 9, 1],
  [7, 1, 3, 9, 2, 4, 8, 5, 6],
  [9, 6, 1, 5, 3, 7, 2, 8, 4],
  [2, 8, 7, 4, 1, 9, 6, 3, 5],
  [3, 4, 5, 2, 8, 6, 1, 7, 9]];

const solution2 = [
  [
    4,
    1,
    6,
    9,
    2,
    7,
    8,
    5,
    3
  ],
  [
    3,
    7,
    2,
    8,
    5,
    6,
    1,
    9,
    4
  ],
  [
    8,
    5,
    9,
    1,
    3,
    4,
    2,
    6,
    7
  ],
  [
    6,
    4,
    7,
    3,
    8,
    2,
    5,
    1,
    9
  ],
  [
    9,
    8,
    5,
    4,
    7,
    1,
    6,
    3,
    2
  ],
  [
    1,
    2,
    3,
    6,
    9,
    5,
    4,
    7,
    8
  ],
  [
    5,
    9,
    8,
    2,
    1,
    3,
    7,
    4,
    6
  ],
  [
    2,
    6,
    1,
    7,
    4,
    9,
    3,
    8,
    5
  ],
  [
    7,
    3,
    4,
    5,
    6,
    8,
    9,
    2,
    1
  ]
]

expect(sudoku(puzzle1)).toEqual(solution);
expect(sudoku(puzzle2)).toEqual(solution2);

// See https://www.chaijs.com for how to use Chai.
import { assert } from "chai";

import { sudoku } from "./solution";

type Board = Array<Array<number>>;

var sudoku_puzzles = [
            ["605720039400005100020100004090030706100809005204050080800003020002900001350067408",
             "615724839487395162923186574598432716136879245274651983849513627762948351351267498"],
            ["008030540300407900410008002043502060500000008060309410100800027005603004029070800",
             "978231546352467981416958372843512769591746238267389415134895627785623194629174853"],
            ["600108203020040090803005400504607009030000050700803102001700906080030020302904005",
                "645198273127346598893275461514627389238419657769853142451782936986531724372964815"],
            ["019060540000000000820974036001503800000000000002701600750138092000000000083040710",
                "319862547467315289825974136671593824538426971942781653756138492194257368283649715"],
            ["046000000902060008008400250000800070500702003010006000064003900300080102000000730",
                "146258397952367418738491256623849571589712643417536829264173985375984162891625734"],
            ["006020050002000194000100207607082019085070030000605400090013040001009000730008900",
                "416927853372856194859134267647382519985471632123695478598213746261749385734568921"]
];
let transpose = (a: Board) => a[0].map((_, c) => a.map(r => r[c]));


const strToBoard = (str: String): Board => {
  const empty_board: Board = [[], [], [], [], [], [], [], [], []];
  return transpose(str.split('').map(Number).reduce((acc, cur, i) => {
    acc[i%9].push(cur); return acc;
  }, empty_board))
}

for (let puzzle of sudoku_puzzles) {
    const parts = puzzle.map(strToBoard);
    expect(sudoku(parts[0])).toEqual(parts[1]);
}