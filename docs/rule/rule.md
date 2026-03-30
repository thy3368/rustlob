# 设计原则

1. 领域api设计

handle(command) -> change_log

2. 将领域api发布成web(http), 事件（kafka，queue),等消费
   编解码（json,sbe,grpc等）
