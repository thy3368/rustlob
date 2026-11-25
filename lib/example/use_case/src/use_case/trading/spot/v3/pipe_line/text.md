只创建不冻结（只创建）

冻结(state,冻结成功，冻结失败)

match/maker/taker/(rest,reject,filled,)

取消（cancel,成交取消)

结算（）

# 流程

## 客户端下单action

- 下达命令 place
- 下太空命令 cancel

## block1执行

- 执行place命令，改变状态， 记录成功信息resp
- 执行cancel命令，改变状态，记录成功信息resp
- block1执行后的状态
- 持久化block1的状态
- command是唯一真相，resp是派生值
- command,resp,state 三个都要持久化？
- command{state,resp}
- command{state{ok/error}} ->resp

- 每条command及对应的result
- 整个block的result

## 客户端拿到resp

- 返回place的resp
- 返回cancel的resp
-

## block use case= single use case + executor

计算（state+working）=updated的对象
