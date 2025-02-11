# Pi Aier

## Intro

Let's stand the tech peak at the first time.


### TODO

- [ ]Prompt前端基本实现
- [ ]基于一个Prompt进行聊天
- [ ] 快捷键 绑定对应的Prompt
- [ ] 使用命令启动对应Prompt作为窗口


- [ ] 支持Lua脚本嵌入Prompt，使用Lua脚本扩展Prompt
    Lua脚本，支持联网，支持打开文件，读取文件，执行有限相关的系统命令
- [ ] 写一个自然语言转Lua的Prompt
- [ ] Prompt用AI分析出 提问中的提示词，再解析成Lua，执行后，再提交给AI
  比如：把桌面上的 a.txt读出来，然后给我总结一下
  一阶段：用AI分析出命令：打开 桌面上 a.txt
  二阶段：Open $HOME/Desktop/a.txt, 转化成Lua脚本
  三阶段：并执行，得到内容，并最终提交
