作业要求:  
实现一个记事本合约应用，并限制内容长度  
1.超出长度报错  
2.添加权限，控制谁可以修改  
3.报错信息通过自定义错误来实现

作业解答:    
一、工程介绍    
notepad主要包括program和client两个package，分别定义了智能合约和客户端测试的代码逻辑  

1.program  
* lib.rs声明相关module  
* entrypoint.rs智能合约的程序入口  
* instructions.rs定义了记事本的操作指令，注册NoteCreate、更新NoteUpdate、删除NoteDelete逻辑，以及数据结构NotepadInstructionPayload用于传递指令参数  
* processor.rs定义数据解析以及指令路由的处理逻辑，包括入口函数  process_instruction()，记事本创建、更新、删除函数note_create()、note_update()、note_delete()  
* state.rs定义记事本需要存储的状态信息，包括内容contents和权限pubkey  
* error.rs自定义文本长度错误和权限错误，NotepadError::InvalidContentsLen，NotepadError::InvalidPubkey    

2.client  
main.rs依次对记事本功能进行单元测试，包括：   
* 笔记创建、更新、删除测试：note_create_test()、note_update_test()、note_delete_test()  
* 异常情况测试，输入长度超限、权限不匹配测试：note_invalid_contents_len_test()、note_invalid_pubkey_test()

二、实现结果截图展示  
1.智能合约编译及部署  
<img width="1493" alt="合约编译" src="https://github.com/lihuineo/Notepad/assets/161575076/d19b2a12-9d06-4c5c-ace4-66225216b547">
<img width="1445" alt="合约部署" src="https://github.com/lihuineo/Notepad/assets/161575076/b7bc8677-3110-4bb5-b580-3202d9d95286">

2.基本功能验证：账户创建、更新、删除  
笔记创建
https://solscan.io/tx/4JLZcJHNFQ4WhuMRKKA7obxVsFLfoaRj2SPBQWosZZqMWfQpGJCmswgwJ6SaruLJoFDfpddXqTsvFbL2yKVznhkK?cluster=devnet
<img width="1395" alt="笔记创建-指令" src="https://github.com/lihuineo/Notepad/assets/161575076/74239cec-b8a2-4728-8162-1a90024023ac">
<img width="1395" alt="笔记创建-内容" src="https://github.com/lihuineo/Notepad/assets/161575076/2e1815f0-fb74-4f17-bcaf-3c9957ead810">

笔记更新
https://solscan.io/tx/67Sn3jz5PSd7W2Xkw31WeXFRgWU8jBBxBZn9s3ow4afMASjaCQVrRFmLepa8pKrGJjvkKuk6rjkhgkd3wcSphnd3?cluster=devnet
<img width="1418" alt="笔记更新-详情" src="https://github.com/lihuineo/Notepad/assets/161575076/18faf7cd-8246-456a-bc90-e4b907e1c450">
<img width="1399" alt="笔记更新-内容" src="https://github.com/lihuineo/Notepad/assets/161575076/b7280a48-ee51-4ed8-8166-2b4afd10ed32">

笔记删除  
https://solscan.io/tx/67Sn3jz5PSd7W2Xkw31WeXFRgWU8jBBxBZn9s3ow4afMASjaCQVrRFmLepa8pKrGJjvkKuk6rjkhgkd3wcSphnd3?cluster=devnet
<img width="1395" alt="笔记删除-指令" src="https://github.com/lihuineo/Notepad/assets/161575076/71f79a3c-f0c3-4aa4-87a0-00497495ddfb">
<img width="1432" alt="笔记删除-内容" src="https://github.com/lihuineo/Notepad/assets/161575076/e4cbc888-0d05-488a-8218-03f1186419ba">


3.异常情况（内容长度超限、权限不匹配）的验证，以及自定义日志展示  
权限不匹配
<img width="1434" alt="权限异常测试" src="https://github.com/lihuineo/Notepad/assets/161575076/7a64d667-8c76-47ac-8fd8-b7ee5397ebbb">

内容长度超限
<img width="1450" alt="输入异常测试" src="https://github.com/lihuineo/Notepad/assets/161575076/1309a106-3747-4758-a1e7-1a575bb05b7c">
