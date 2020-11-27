import init, {
  Button,
  Orientation,
  Stack,
  Stream,
  Text,
  TextField,
} from "../pkg/ui_rs.js";

async function main() {
  // Init wasm
  await init();

  const stream1 = new Stream("");
  const stream2 = new Stream("");

  const app = new Stack()
    .child(
      new Stack()
        .child(new Text("Hello:"))
        .child(new TextField(stream1))
        .child(new Text().stream(stream1))
        .orient(Orientation.Horizontal)
    )
    .child(
      new Stack()
        .child(new Text("World:"))
        .child(new TextField(stream2))
        .child(new Text().stream(stream2))
        .orient(Orientation.Horizontal)
    )
    .child(new Button("Hello World").action(() => alert("Hello World!")))
    .orient(Orientation.Vertical);

  stream1.next("hello");
  stream2.next("world");

  document.body.appendChild(app.body);
}

main().catch(console.error);
