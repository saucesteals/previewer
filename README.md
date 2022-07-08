<img src="https://socialify.git.ci/saucesteals/previewer-rs/image?font=Inter&logo=https%3A%2F%2Fgithub.com%2Fsaucesteals%2Fpreviewer-rs%2Fblob%2Fmain%2Fassets%2Flogo.png%3Fraw%3Dtrue&owner=1&pattern=Solid&theme=Dark" alt="previewer" width="1040" height="320"  />

<h2 align="center">
<b>🎵 Welcome to Previewer 🎵</b>
<p>Advanced Link Previews in Discord!<p>
</h2>

## **⚙️ Installation**

### **Public Bot**

- [Invite our publicly hosted bot](https://discord.com/oauth2/authorize?client_id=866304561017913354&scope=bot&permissions=388176)

### **Heroku**

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/saucesteals/previewer-rs)

1. Click the [Deploy to Heroku](https://heroku.com/deploy?template=https://github.com/saucesteals/previewer-rs) button above
2. Enter your discord bot's token as prompted
3. Click `Deploy app`, then wait until it's finished
4. Enjoy :)

### **Manual**

Previewer requires [Rust](https://www.rust-lang.org/tools/install) to run.

Install the dependencies.

Create and configure an `.env` file from the following template:

```sh
# Your discord bot's token
DISCORD_TOKEN=
```

Start it!

```sh
cargo run
```

## **✅ Supported Websites**

- [Tiktok](https://www.tiktok.com/) - Replies with an in-app attachment of the tiktok/video

```
https://www.tiktok.com/@vscode/video/6932506933505166598
```

- ... More Soon **(Requests are welcome and encouraged)**

## **🕹️ Usage**

1. Invite the bot
2. Make sure the bot has the following permissions in the channels you want it to work in:

   - View Channel
   - Send Messages
   - Embed Links
   - Attach Files
   - Read Message History

3. Send a link matching one of the supported websites

## **🤝 Contributing**

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## **📝 License**

Distributed under the MIT License. See `LICENSE` for more information.
