Project Name: gpt

Project Description

This is a command line tool written in Rust that is used to submit queries to chatgpt and get the response automatically.

Configuration File

Before using this tool, you are required to create a configuration file with the following key-value pairs under $HOME/.haigpt/config.toml :

• url: This is the url where the query requests will be sent.
• key: This is the API key, which might include multiple items separated by commas.
• prompt: This is your system message that will be configured as your question submitted to chatgpt.

For example:

url = "Put your URL here"
key = "Put your API key here"
prompt = "Put your system message here"

Examples

Here are some examples of how you can use this tool:

1. Querying by command line argument:

$ gpt -m "Hello chatgpt"

1. Querying by file:

$ gpt -f query.txt

1. Querying by pipe:

$ echo "Hello chatgpt" | gpt
