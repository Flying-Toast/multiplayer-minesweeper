let ws = null;

const Icon = {
	// num[0] is meaningless, zero-index is just used as padding so that num[1] is icon "1"
	num: Array.from(Array(9).keys()).map(String),
	flag: "flag",
};

const OutgoingMessage = {
	Reveal: function(x, y) {
		return `reveal\n${x}\n${y}`;
	}
};

function eltDisplayRevealedStyle(elt) {
	elt.classList.remove("hidden-square");
	elt.classList.add("revealed-square");
}

function eltDisplayHiddenStyle(elt) {
	elt.classList.remove("revealed-square");
	elt.classList.add("hidden-square");
}

class Square {
	constructor(x, y) {
		this.x = x;
		this.y = y;
		let elt = document.createElement("span");
		elt.classList.add("square");
		elt.classList.add("hidden-square");
		elt.addEventListener("click", function() {
			ws.send(OutgoingMessage.Reveal(x, y));
		});
		elt.addEventListener("mousedown", function() {
			let lastTarget = elt;
			let wasRevealed = elt.classList.contains("revealed-square");
			eltDisplayRevealedStyle(elt);
			let mousemoveHandler = function(e) {
				if (!wasRevealed && lastTarget.classList.contains("square")) {
					eltDisplayHiddenStyle(lastTarget);
				}
				lastTarget = e.target;
				wasRevealed = e.target.classList.contains("revealed-square");
				if (e.target.classList.contains("square")) {
					eltDisplayRevealedStyle(e.target);
				}
			};
			addEventListener("mousemove", mousemoveHandler);
			addEventListener("mouseup", function(e) {
				removeEventListener("mousemove", mousemoveHandler);
				if (e.target.classList.contains("square")) {
					e.target.click();
				}
			});
		});
		this.elt = elt;
	}

	displayRevealedStyle() {
		eltDisplayRevealedStyle(this.elt);
	}

	displayHiddenStyle() {
		eltDisplayHiddenStyle(this.elt);
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
	addEventListener("dragstart", e => e.preventDefault());
	let boardElt = document.querySelector("#board");
	let field;
	ws = new WebSocket(`ws://${location.hostname}:12345`);

	ws.addEventListener("message", function(m) {
		const message = JSON.parse(m.data);
		switch (message.t) {
			case "newgame":
				field = new Minefield(boardElt, message.width, message.height);
				break;
			case "reveal":
				const numContent = parseInt(message.content);
				let square = field.getSquare(message.x, message.y);

				if (!isNaN(numContent) && numContent > 0 && numContent < 9) {
					square.displayRevealedStyle();
					square.setIcon(Icon.num[numContent]);
				} else if (numContent == 0) {
					square.displayRevealedStyle();
				} else if (message.content == "!") {
					//TODO: implement
					console.log("LOSE!");
				}
				break;
			default:
				console.log("Unhandled message:", message);
		}
	});

	ws.addEventListener("close", function() {
		console.log("Websocket closed");
	});
}
addEventListener("load", main);
