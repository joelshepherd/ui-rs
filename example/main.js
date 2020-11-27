import init, {
  Alignment,
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

  const username = new Stream("");
  const password = new Stream("");

  const app = new Stack()
    .child(
      new Stack()
        .child(new Text("Username:"))
        .child(new TextField(username))
        .align(Alignment.Center)
        .orient(Orientation.Horizontal)
    )
    .child(
      new Stack()
        .child(new Text("Password:"))
        .child(new TextField(password).secure())
        .align(Alignment.Center)
        .orient(Orientation.Horizontal)
    )
    .child(new Button("Login").action(() => alert("Nice try!")))
    .child(
      new Stack()
        .child(new Text("username:"))
        .child(new Text().stream(username))
        .orient(Orientation.Horizontal)
        .spacing(10)
    )
    .child(
      new Stack()
        .child(new Text("password:"))
        .child(new Text().stream(password))
        .orient(Orientation.Horizontal)
        .spacing(10)
    )
    .orient(Orientation.Vertical);

  // Manually pass in next value
  username.next("hello");
  password.next("world");

  document.body.appendChild(app.body);
}

main().catch(console.error);
