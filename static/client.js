let ws = null;
let gameOver = false;

function onConnectionError() {
	setTimeout(function() {
		document.querySelector("#connection-error").classList.remove("hidden");
	}, 500);
}

const Icon = {
	// num[0] is meaningless, zero-index is just used as padding so that num[1] is icon "1"
	num: Array.from(Array(9).keys()).map(String),
	flag: "flag",
};

const OutgoingMessage = {
	Reveal: function(x, y) {
		return `reveal\n${x}\n${y}`;
	},
	JoinRoom: function(id) {
		return `join\n${id}`;
	},
	Flag: function(x, y, on) {
		return `flag\n${x}\n${y}\n${on}`;
	},
	NewGame: function(width, height, num_mines) {
		return `newgame\n${width}\n${height}\n${num_mines}`;
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
		this.flagged = false;
		let elt = document.createElement("span");
		elt.classList.add("square");
		elt.classList.add("hidden-square");
		elt.addEventListener("click", function(e) {
			if (e.ctrlKey) {
				this.toggleFlag();
				return;
			}

			if (e.button != 0) {
				return;
			}

			this.reveal();
		}.bind(this));
		elt.addEventListener("mousedown", function(e) {
			if (gameOver || e.button != 0 || this.flagged || e.ctrlKey) {
				return;
			}

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
				if (e.button != 0 || e.ctrlKey) {
					return;
				}
				removeEventListener("mousemove", mousemoveHandler);
				if (e.target.classList.contains("square")) {
					e.target.click();
				}
			});
		}.bind(this));
		elt.addEventListener("contextmenu", function(e) {
			e.preventDefault();
			this.toggleFlag();
		}.bind(this));
		this.elt = elt;
	}

	toggleFlag() {
		if (this.elt.classList.contains("revealed-square") || gameOver) {
			return;
		}

		if (this.flagged) {
			this.flagOff();
		} else {
			this.flagOn();
		}
	}

	flagOn(broadcast = true) {
		this.flagged = true;
		if (broadcast) {
			ws.send(OutgoingMessage.Flag(this.x, this.y, true));
		}
		this.setIcon(Icon.flag);
	}

	flagOff(broadcast = true) {
		this.flagged = false;
		if (broadcast) {
			ws.send(OutgoingMessage.Flag(this.x, this.y, false));
		}
		this.clearIcon();
	}

	displayRevealedStyle() {
		eltDisplayRevealedStyle(this.elt);
	}

	displayHiddenStyle() {
		eltDisplayHiddenStyle(this.elt);
	}

	displayBoomedStyle() {
		this.elt.classList.remove("hidden-square");
		this.elt.classList.add("boom-mine");
	}

	// `setIcon(Icon.flag)` (displays a flag)
	// `setIcon(Icon.num[1])` (displays a "1")
	setIcon(iconName) {
		this.elt.style.backgroundImage = `url("/${iconName}.svg")`;
	}

	clearIcon() {
		this.elt.style.backgroundImage = "";
	}

	reveal() {
		if (gameOver) {
			return;
		}

		if (!this.flagged) {
			ws.send(OutgoingMessage.Reveal(this.x, this.y));
		}
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

function idealMineCount(area) {
	if (area == 30 * 16) {
		return 99;
	} else {
		return Math.round(
			(0.0002 * area * area) + (0.0938 * area) + 0.8937
		);
	}
}

function main() {
	addEventListener("dragstart", e => e.preventDefault());
	let roomCodeDisplay = document.querySelector("#your-room-code");
	let roomCodeInput = document.querySelector("#room-code-input");
	let roomCodeSubmit = document.querySelector("#room-code-submit");
	roomCodeInput.addEventListener("keypress", function(e) {
		if (e.key == "Enter") {
			roomCodeSubmit.click();
		}
	});
	roomCodeSubmit.addEventListener("click", function() {
		roomCodeInput.classList.remove("bad-input");
		if (roomCodeInput.value != "") {
			ws.send(OutgoingMessage.JoinRoom(roomCodeInput.value));
		}
	});
	let boardWidthInput = document.querySelector("#board-width");
	let boardHeightInput = document.querySelector("#board-height");
	let boardInputsElt = document.querySelector("#board-inputs");
	let numMinesInput = document.querySelector("#num-mines");
	let newGameButton = document.querySelector("#new-game");
	newGameButton.addEventListener("click", function() {
		boardInputsElt.classList.remove("bad-input");
		let width = Number(boardWidthInput.value);
		let height = Number(boardHeightInput.value);
		let numMines = Number(numMinesInput.value);
		if (!isNaN(width) && !isNaN(height) && !isNaN(numMines)) {
			ws.send(OutgoingMessage.NewGame(width, height, numMines));
		}
	});
	document.querySelector("#ideal-mines").addEventListener("click", function() {
		let width = Number(boardWidthInput.value);
		let height = Number(boardHeightInput.value);
		let area = width * height;
		if (!isNaN(width) && !isNaN(height)) {
			numMinesInput.value = idealMineCount(area);
		}
	});
	let boardElt = document.querySelector("#board");
	let field;
	let roomId = null;
	try {
		ws = new WebSocket(`ws://ws.${location.hostname}:12345`);
	} catch (e) {
		onConnectionError();
	}
	ws.addEventListener("close", onConnectionError);
	ws.addEventListener("error", onConnectionError);

	ws.addEventListener("message", function(m) {
		const message = JSON.parse(m.data);
		switch (message.t) {
			case "newgame": {
				document.querySelector("#loader").classList.add("hidden");
				document.querySelector("#wrapper").classList.remove("hidden");
				field = new Minefield(boardElt, message.width, message.height);
				boardWidthInput.value = message.width;
				boardHeightInput.value = message.height;
				numMinesInput.value = message.mines;
				gameOver = false;
				break;
			}
			case "reveal": {
				const numContent = Number(message.content);
				let square = field.getSquare(message.x, message.y);

				if (!isNaN(numContent) && numContent > 0 && numContent < 9) {
					square.displayRevealedStyle();
					square.setIcon(Icon.num[numContent]);
				} else if (numContent == 0) {
					square.displayRevealedStyle();
				} else if (message.content == "!") {
					square.displayBoomedStyle();
					gameOver = true;
				}
				break;
			}
			case "room": {
				roomCodeInput.value = "";
				roomId = message.id;
				roomCodeDisplay.innerText = roomId;
				break;
			}
			case "flag": {
				let square = field.getSquare(message.x, message.y);
				if (message.on) {
					square.flagOn(false);
				} else {
					square.flagOff(false);
				}
				break;
			}
			case "badboard": {
				boardInputsElt.classList.add("bad-input");
				break;
			}
			case "badcode": {
				roomCodeInput.classList.add("bad-input");
				break;
			}
			default:
				console.log("Unhandled message:", message);
		}
	});
}
addEventListener("load", main);

function prefetchImages() {
	const images = [
		"1.svg",
		"2.svg",
		"3.svg",
		"4.svg",
		"5.svg",
		"6.svg",
		"7.svg",
		"8.svg",
		"flag.svg",
		"mine.svg",
	];
	for (const i of images) {
		document.createElement("img").src = i;
	}
}
prefetchImages();
