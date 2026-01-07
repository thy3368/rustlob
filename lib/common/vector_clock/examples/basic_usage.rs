use vector_clock::clock::vector_clock::VectorClock;

fn print_section(title: &str) {
    println!("\n{}", "=".repeat(60));
    println!("{}", title);
    println!("{}\n", "=".repeat(60));
}

fn print_subsection(title: &str) {
    println!("\n>>> {}", title);
    println!("{}", "-".repeat(60));
}

fn main() {
    print_section("向量时钟（Vector Clock）完整使用示例");

    example_1_message_chain();
    example_2_order_system();
    example_3_message_detection();
    example_4_concurrent_events();
    example_5_conflict_detection();
}

/// 场景1：三个节点的消息链 A -> B -> C
fn example_1_message_chain() {
    print_subsection("场景1：消息链传递 (A -> B -> C)");

    let nodes = &["A", "B", "C"];
    let mut clock_a = VectorClock::new("A", nodes);
    let mut clock_b = VectorClock::new("B", nodes);
    let mut clock_c = VectorClock::new("C", nodes);

    println!("初始状态:");
    println!("  A: {:?}", clock_a.counters());
    println!("  B: {:?}", clock_b.counters());
    println!("  C: {:?}", clock_c.counters());

    println!("\n【步骤1】A产生本地事件");
    clock_a.local_event();
    clock_a.local_event();
    println!("  A执行2次本地事件");
    println!("  A: {:?}", clock_a.counters());

    println!("\n【步骤2】A发送消息给B");
    let msg_ab = clock_a.prepare_message();
    println!("  消息携带时钟: {:?}", msg_ab);
    println!("  A发送后: {:?}", clock_a.counters());

    clock_b.receive_message(&msg_ab);
    println!("  B接收后: {:?}", clock_b.counters());
    println!("  说明：B知道了A的所有3个事件");

    println!("\n【步骤3】B发送消息给C");
    let msg_bc = clock_b.prepare_message();
    println!("  消息携带时钟: {:?}", msg_bc);
    println!("  B发送后: {:?}", clock_b.counters());

    clock_c.receive_message(&msg_bc);
    println!("  C接收后: {:?}", clock_c.counters());
    println!("  说明：C继承了A->B的因果链");

    println!("\n【步骤4】A和C独立产生事件（并发）");
    clock_a.local_event();
    println!("  A产生1个事件: {:?}", clock_a.counters());

    clock_c.local_event();
    println!("  C产生1个事件: {:?}", clock_c.counters());

    println!("\n【因果关系分析】");
    let relation = clock_a.compare(&clock_b);
    println!("  A vs B: {:?}", relation);
    println!("    说明：A的所有操作都被B看到了");

    let relation = clock_b.compare(&clock_c);
    println!("  B vs C: {:?}", relation);
    println!("    说明：B的所有操作都被C看到了");

    let relation = clock_a.compare(&clock_c);
    println!("  A vs C: {:?}", relation);
    println!("    说明：A的最后事件和C的事件是并发的（无通信）");
}

/// 场景2：分布式订单系统
fn example_2_order_system() {
    print_subsection("场景2：订单系统事务追踪");

    let services = &["OrderSvc", "PaymentSvc", "InventorySvc"];
    let mut order = VectorClock::new("OrderSvc", services);
    let mut payment = VectorClock::new("PaymentSvc", services);
    let mut inventory = VectorClock::new("InventorySvc", services);

    println!("初始状态:");
    println!("  Order:     {:?}", order.counters());
    println!("  Payment:   {:?}", payment.counters());
    println!("  Inventory: {:?}", inventory.counters());

    println!("\n【步骤1】OrderSvc创建订单");
    order.local_event();
    println!("  Order:     {:?}", order.counters());

    println!("\n【步骤2】OrderSvc请求PaymentSvc处理支付");
    let payment_req = order.prepare_message();
    println!("  请求消息: {:?}", payment_req);
    println!("  Order:     {:?}", order.counters());

    payment.receive_message(&payment_req);
    println!("  Payment:   {:?}", payment.counters());

    println!("\n【步骤3】PaymentSvc处理支付");
    payment.local_event();
    println!("  Payment:   {:?}", payment.counters());

    println!("\n【步骤4】PaymentSvc回复OrderSvc");
    let payment_resp = payment.prepare_message();
    println!("  回复消息: {:?}", payment_resp);
    println!("  Payment:   {:?}", payment.counters());

    order.receive_message(&payment_resp);
    println!("  Order:     {:?}", order.counters());

    println!("\n【步骤5】OrderSvc并发请求InventorySvc");
    let inventory_req = order.prepare_message();
    println!("  请求消息: {:?}", inventory_req);
    println!("  Order:     {:?}", order.counters());

    inventory.receive_message(&inventory_req);
    println!("  Inventory: {:?}", inventory.counters());

    println!("\n【步骤6】InventorySvc检查库存");
    inventory.local_event();
    println!("  Inventory: {:?}", inventory.counters());

    println!("\n【因果关系分析】");
    let rel1 = order.compare(&payment);
    let rel2 = payment.compare(&order);
    let rel3 = inventory.compare(&payment);

    println!("  OrderSvc vs PaymentSvc: {:?}", rel1);
    println!("    说明：OrderSvc的操作导致了PaymentSvc的响应");

    println!("  PaymentSvc vs OrderSvc: {:?}", rel2);
    println!("    说明：PaymentSvc操作基于OrderSvc的请求");

    println!("  InventorySvc vs PaymentSvc: {:?}", rel3);
    println!("    说明：两个服务独立操作，事件并发");
}

/// 场景3：检测消息丢失
fn example_3_message_detection() {
    print_subsection("场景3：消息丢失检测");

    let mut producer = VectorClock::new("Producer", &["Producer", "Consumer"]);
    let mut consumer = VectorClock::new("Consumer", &["Producer", "Consumer"]);

    println!("【正常消息流】\n");

    println!("发送消息1");
    let msg1 = producer.prepare_message();
    println!("  消息时钟: {:?}", msg1);
    consumer.receive_message(&msg1);
    println!("  消费者接收: {:?}\n", consumer.counters());

    println!("发送消息2");
    let msg2 = producer.prepare_message();
    println!("  消息时钟: {:?}", msg2);
    consumer.receive_message(&msg2);
    println!("  消费者接收: {:?}\n", consumer.counters());

    println!("【消息丢失模拟】\n");

    println!("生产者发送消息3（但消费者没收到！）");
    let msg3 = producer.prepare_message();
    println!("  消息时钟: {:?}", msg3);
    println!("  消费者 [丢失] \n");

    println!("生产者继续发送消息4");
    let msg4 = producer.prepare_message();
    println!("  消息时钟: {:?}", msg4);
    consumer.receive_message(&msg4);
    println!("  消费者接收: {:?}", consumer.counters());

    println!("\n【问题检测】");
    let producer_counter = msg4.get("Producer").copied().unwrap_or(0);
    let consumer_counter = consumer.counters().get("Producer").copied().unwrap_or(0);
    let gap = producer_counter - consumer_counter;

    println!("  生产者消息中的计数: {}", producer_counter);
    println!("  消费者认知的计数:   {}", consumer_counter);
    println!("  差距: {}", gap);

    if gap > 1 {
        println!("  ⚠️  检测到消息丢失！应该最多相差1，实际相差{}", gap);
        println!("  缺失的消息序号范围: {} - {}", consumer_counter + 1, producer_counter - 1);
    }
}

/// 场景4：并发事件检测
fn example_4_concurrent_events() {
    print_subsection("场景4：并发事件检测与排序");

    let nodes = &["N1", "N2", "N3"];
    let mut n1 = VectorClock::new("N1", nodes);
    let mut n2 = VectorClock::new("N2", nodes);
    let mut n3 = VectorClock::new("N3", nodes);

    println!("模拟两个并发分支：");
    println!("  分支A: N1 -> N2");
    println!("  分支B: N1 -> N3\n");

    println!("【初始】所有节点从N1开始");
    n1.local_event();
    println!("N1执行事件: {:?}", n1.counters());

    let msg_to_n2 = n1.prepare_message();
    n2.receive_message(&msg_to_n2);
    println!("N2收到: {:?}", n2.counters());

    let msg_to_n3 = n1.prepare_message();
    n3.receive_message(&msg_to_n3);
    println!("N3收到: {:?}\n", n3.counters());

    println!("【分支A：N2继续处理】");
    n2.local_event();
    n2.local_event();
    println!("N2执行2个事件: {:?}\n", n2.counters());

    println!("【分支B：N3继续处理】");
    n3.local_event();
    println!("N3执行1个事件: {:?}\n", n3.counters());

    println!("【合并检测】");
    let rel = n2.compare(&n3);
    println!("N2 vs N3: {:?}", rel);
    println!("  N2: {:?}", n2.counters());
    println!("  N3: {:?}", n3.counters());

    println!("\n【N2如果收到N3的消息】");
    let msg_from_n3 = n3.prepare_message();
    println!("N3消息: {:?}", msg_from_n3);
    n2.receive_message(&msg_from_n3);
    println!("N2合并后: {:?}", n2.counters());
    println!("  N2现在看到了整个系统的操作历史");

    println!("\n【对比：事件排序能力】");
    let events = vec![
        ("事件1 by N1", vec![("N1", 1), ("N2", 0), ("N3", 0)]),
        ("事件2 by N2", vec![("N1", 1), ("N2", 1), ("N3", 0)]),
        ("事件3 by N3", vec![("N1", 1), ("N2", 0), ("N3", 1)]),
    ];

    println!("根据向量时钟排序事件:");
    for (name, counters) in events.iter() {
        println!("  {} -> {:?}", name, counters);
    }
}

/// 场景5：冲突检测和版本管理
fn example_5_conflict_detection() {
    print_subsection("场景5：冲突检测与版本管理");

    let nodes = &["Client1", "Client2", "Server"];
    let mut client1 = VectorClock::new("Client1", nodes);
    let mut client2 = VectorClock::new("Client2", nodes);
    let mut server = VectorClock::new("Server", nodes);

    println!("\n【场景】两个客户端并发修改同一条记录\n");

    println!("【初始】服务器初始化");
    server.local_event();
    println!("Server: {:?}\n", server.counters());

    // Client1 获取数据
    println!("【步骤1】Client1 向 Server 请求数据");
    let init_msg = server.prepare_message();
    client1.receive_message(&init_msg);
    println!("Client1 初始版本: {:?}", client1.counters());

    // Client2 获取数据
    println!("\n【步骤2】Client2 向 Server 请求数据");
    let init_msg2 = server.prepare_message();
    client2.receive_message(&init_msg2);
    println!("Client2 初始版本: {:?}", client2.counters());

    // Client1 修改数据
    println!("\n【步骤3】Client1 修改数据（并发）");
    client1.local_event();
    println!("Client1 修改后: {:?}", client1.counters());

    // Client2 也修改数据
    println!("\n【步骤4】Client2 修改数据（并发）");
    client2.local_event();
    println!("Client2 修改后: {:?}", client2.counters());

    println!("\n【冲突检测】");
    println!("Client1 vs Client2:");

    // 检测是否冲突
    if client1.may_conflict(&client2) {
        println!("  ⚠️  检测到冲突！两个修改是并发的");

        // 找出冲突的节点
        let diff_nodes = client1.concurrent_diff(&client2);
        println!("  冲突来自节点: {:?}", diff_nodes);

        println!("\n【冲突分析】");
        println!("  Client1 版本: {:?}", client1.counters());
        println!("  Client2 版本: {:?}", client2.counters());

        println!("\n  两个版本都有自己独立的修改：");
        println!("    - Client1 有自己的修改");
        println!("    - Client2 有自己的修改");
        println!("    - 无法自动合并，需要人工解决");
    } else {
        println!("  无冲突");
    }

    // Client1 提交给 Server
    println!("\n【步骤5】Client1 提交修改给 Server");
    let update1 = client1.prepare_message();
    server.receive_message(&update1);
    println!("Server 收到 Client1: {:?}", server.counters());

    // Client2 也提交
    println!("\n【步骤6】Client2 尝试提交修改给 Server");
    let update2 = client2.prepare_message();
    println!("Client2 的版本: {:?}", update2);
    println!("Server 当前版本: {:?}", server.counters());

    // 检查 Client2 是否是 Server 的直接后继
    if client2.is_newer_than(&server) {
        println!("\n  ❌ Server 版本已经是最新的，Client2 的修改过时了");
        println!("  需要 Client2 重新获取最新数据再修改");
    }

    println!("\n【总结】");
    println!("冲突检测方法：");
    println!("  1. may_conflict() - 检测是否有并发修改");
    println!("  2. concurrent_diff() - 找出冲突的节点");
    println!("  3. is_newer_than() - 检查版本新旧");
    println!("  4. is_direct_successor() - 检查是否直接后继");

    println!("\n【应用场景】");
    println!("  ✓ 分布式数据库 (CouchDB, Riak)");
    println!("  ✓ Git 版本控制系统");
    println!("  ✓ 多人协作编辑 (Figma, Google Docs)");
    println!("  ✓ 分布式缓存 (Redis Cluster)");
}
