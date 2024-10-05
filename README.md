# ✨ Ask-cli
A simple command line tool to communicate with the Google Gemini AI REST API and get responses directly in the terminal with markdown format.

> [!NOTE]
> You need to get a Gemini API key in order to use this tool, more information at [Google AI Studio](https://ai.google.dev/aistudio)

## 🤖 Features
- Usage of the Gemini AI model directly in the terminal  
- System prompts to change the responses  
- An execute option to ask the AI for commands and execute them directly  

## 💾 Installation

Download from [releases](https://github.com/gg0074x/ask-cli/releases)

## 🗒️ Usage

After running from the first time the program will tell you that you haven't specified an API key  
You can specify this key by going to your config folder:  
- `~/.config/ask_config` in Linux
- `C:\Users\username\AppData\Roaming` in Windows
- `/Users/username/Library/Application Support`

And add the following to the `config.toml` file:  
`GEMINI_TOKEN=YOUR_API_KEY_HERE`  
Alternatively you can pass your API key as an environment variable by executing:  
`export GEMINI_TOKEN=YOUR_API_KEY_HERE`  

```sh
Usage: ask-cli [OPTIONS] --prompt <PROMPT>

Options:
  -t, --shell <SHELL>    
        Specify the shell used when running with the execute option
  -x, --execute          
        Ask the AI to provide you with just a command and then ask you to execute it
  -s, --system <SYSTEM>  
        Specify an optional system prompt to change the AI generation results
  -p, --prompt <PROMPT>  
        The prompt that the AI will respond to
  -h, --help             Print help
  -V, --version          Print version
```

## 🔨 Contributing

- [ ] Code refactor
- [ ] Use commands instead of flags
- [ ] Support different AI models
- [ ] Support other AI providers
