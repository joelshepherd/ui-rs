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
    .child(new Text("Login Form"))
    .child(
      new Stack()
        .child(new Text("Username:"))
        .child(new TextField(username))
        .child(new Text().stream(username))
        .align(Alignment.Center)
        .orient(Orientation.Horizontal)
        .spacing(5)
    )
    .child(
      new Stack()
        .child(new Text("Password:"))
        .child(new TextField(password).secure())
        .child(new Text().stream(password))
        .align(Alignment.Center)
        .orient(Orientation.Horizontal)
        .spacing(5)
    )
    .child(new Button("Login").action(() => alert("Nice try!")))
    .align(Alignment.Leading)
    .orient(Orientation.Vertical)
    .spacing(10);

  // Manually pass in next value
  username.next("user@example.com");
  password.next("password");

  document.body.appendChild(app.body);
}

main().catch(console.error);
