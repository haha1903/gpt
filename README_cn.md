项目名称: gpt

项目介绍

这是一款用Rust编写的命令行工具，可用于提交查询至chatgpt并获取结果。

配置文件

在使用这个工具之前，需要在位置$HOME/.haigpt/config.toml创建一个配置文件，其中包含以下键值对：

• url：这是提交查询请求的URL地址。
• key：这是API秘钥，可能包括多个以逗号分隔的项目。
• prompt：这是系统消息，会被配置成你提交给chatgpt的问题。

例如：

url = "这里放置你的URL"
key = "这里放置你的API秘钥"
prompt = "这里放置你的系统消息"

示例

以下是如何使用此工具的一些例子:

1. 使用命令行参数提交查询:

$ gpt -m "你好chatgpt"

1. 使用文件提交查询:

$ gpt -f query.txt

1. 通过管道提交查询：

$ echo "你好chatgpt" | gpt