import init, {
  Alignment,
  Button,
  Orientation,
  Stack,
  Stream,
  Text,
} from "../pkg/ui_rs.js";

async function main() {
  // Init wasm
  await init();

  const player1 = new Stream("1");
  const player2 = new Stream("2");

  const winner = new Stream("");

  // Let's do this better
  let p1Value;
  let p2Value;
  player1.subscribe((x) => (p1Value = Number(x)));
  player2.subscribe((x) => (p2Value = Number(x)));
  const checkWinner = () =>
    winner.next(
      p1Value > p2Value
        ? "Player 1 wins"
        : p2Value > p1Value
        ? "Player 2 wins"
        : "It's a draw!"
    );
  player1.subscribe(checkWinner);
  player2.subscribe(checkWinner);

  const app = new Stack()
    .child(new Text("Dice Game"))
    .child(
      new Stack()
        .child(
          new Stack()
            .child(new Text("Player 1"))
            .child(
              new Stack()
                .child(new Text("Score:"))
                .child(new Text().stream(player1))
                .orient(Orientation.Horizontal)
                .spacing(5)
            )
            .orient(Orientation.Vertical)
            .spacing(10)
        )
        .child(
          new Stack()
            .child(new Text("Player 2"))
            .child(
              new Stack()
                .child(new Text("Score:"))
                .child(new Text().stream(player2))
                .orient(Orientation.Horizontal)
                .spacing(5)
            )
            .orient(Orientation.Vertical)
            .spacing(10)
        )
        .align(Alignment.Center)
        .orient(Orientation.Horizontal)
        .spacing(10)
    )
    .child(
      new Button("Roll").action(() => {
        const number1 = Math.ceil(Math.random() * 6);
        const number2 = Math.ceil(Math.random() * 6);

        player1.next(String(number1));
        player2.next(String(number2));
      })
    )
    .child(new Text().stream(winner))
    .align(Alignment.Center)
    .orient(Orientation.Vertical)
    .spacing(20);

  document.body.appendChild(app.body);
}

main().catch(console.error);
