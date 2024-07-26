# Ask-cli
A simple command line tool to communicate with the Google Gemini AI REST API and get responses directly in the terminal with markdown format.

> [!NOTE]
> You need to get a Gemini API key in order to use this tool, more information at [https://ai.google.dev/aistudio](https://ai.google.dev/aistudio)

## Features
- Usage of the Gemini AI model directly in the terminal
- System prompts to change the responses
- An execute option to ask the AI for commands and execute them directly

## Installation

Download from [releases](https://github.com/gg0074x/ask-cli/releases)

## Usage

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

