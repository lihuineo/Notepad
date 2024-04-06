作业要求:  
实现一个记事本合约应用，并限制内容长度  
1.超出长度报错  
2.添加权限，控制谁可以修改  
3. 报错信息通过自定义错误来实现

作业解答:    
一、工程主要包括program和client两个package，分别定义了智能合约和客户端测试的代码逻辑  
1.program  
lib.rs声明相关module  
entrypoint.rs智能合约的程序入口  
instructions.rs定义了记事本的操作指令，注册NoteCreate、更新NoteUpdate、删除NoteDelete逻辑，以及数据结构NotepadInstructionPayload用于传递指令参数  
processor.rs定义数据解析以及指令路由的处理逻辑，包括入口函数  process_instruction()，记事本创建、更新、删除函数note_create()、note_update()、note_delete()  
state.rs定义记事本需要存储的状态信息，包括内容contents和权限pubkey  
error.rs自定义文本长度错误和权限错误，NotepadError::InvalidContentsLen，NotepadError::InvalidPubkey    
2.client
lib.rs依次对记事本功能进行单元测试。基本创建、更新、删除测试：note_create_test()、note_update_test()、note_delete_test(); 输入长度超限、权限不匹配测试：note_invalid_contents_len_test()、note_invalid_pubkey_test()

二、实现结果截图展示  
1.智能合约编译及部署  
2.基本功能验证：账户创建、更新、删除  
笔记创建  
https://solscan.io/tx/3aw7gaRpGj2z2UMfKENzfiMkBGFp8NPZTSx4A1GLRP2jRZ5dCvu9TVQeGyingJvhjhnhmoDyU2mojUR8Mn7tFnUq?cluster=devnet

笔记更新
https://solscan.io/tx/2TJTkMBKDFShmFNTEibskdvHVzKyrVhoW61dvxxJHf1iAugmzTZwZK5rg2yVbRr6sRyvRKj9JBuFred76ZTah1c3?cluster=devnet

笔记删除
3.异常情况（内容长度超限、权限不匹配）的验证，以及自定义日志展示
权限不匹配

内容长度超限
