const Icon = {
	// num[0] is meaningless, zero-index is just used as padding so that num[1] is icon "1"
	num: Array.from(Array(9).keys()).map(String),
	flag: "flag",
};

class Square {
	constructor(x, y) {
		this.x = x;
		this.y = y;
		this.elt = document.createElement("span");
		this.elt.classList.add("square");
		this.elt.classList.add("hidden-square");
	}

	// `setIcon(Icon.flag)` (displays a flag)
	// `setIcon(Icon.num[1])` (displays a "1")
	setIcon(iconName) {
		this.elt.style.backgroundImage = `url("/${iconName}.svg")`;
	}
}

class Minefield {
	constructor(boardElement, width, height) {
		boardElement.innerHTML = "";
		this.boardElement = boardElement;
		this.squares = [];

		for (let y = 0; y < height; y++) {
			let row = [];
			for (let x = 0; x < width; x++) {
				let square = new Square(x, y);
				row.push(square);
				this.boardElement.appendChild(square.elt);
			}
			this.boardElement.appendChild(document.createElement("br"));
			this.squares.push(row);
		}
	}

	getSquare(x, y) {
		return this.squares[y][x];
	}
}

function main() {
	let boardElt = document.querySelector("#board");
	let field = new Minefield(boardElt, 30, 16);
}
addEventListener("load", main);
