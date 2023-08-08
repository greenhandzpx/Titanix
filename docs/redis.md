### redis测试时会卡死

查看syscall发现server在第一次创建socket之后收到client的connect之后accept一次，但是立马又创建新的socket进行listen和accept的等待，但是在accept之前通过setsockopt设置socket
为非阻塞状态，所以在第二个socket accept的时候会返回一个EAGAIN表示资源暂时不可用，只需要实现了非阻塞accept就可以成功启动redis客户端了！！！

### redis benchmark 提前abort

在测试benchmark的时候会在跑SET的测试的时候提前退出，但是如果多输出一些TCP的日志就可能顺利跑完，该问题可能是TCP close的问题，待修复

### 成果

![](fig/redis.png)