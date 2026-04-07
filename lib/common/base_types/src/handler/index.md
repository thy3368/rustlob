# 设计组件约束

## CmdHandlerForUpdate，每个command的处理器

## EventHandler，对应一个事件的处理器

- place_order_event_handler
- new_trade_event_handler

## EventActor，对应一个线程，可以绑核

- place_order_actor,match_actor等
- http_gateway_actor

# 调用路径

- actor---->event_handler----->command_handler, 如kafka_actor 接收事件，并调用事件处理handler
- actor---->command_handler， 如http_command_actor 直接调用command_handler

##
- 定义所有的command/query
- 定义command_handler 里的state,changed_state
- 定义command_handler的pipe line
- 用单测验收command_handler
