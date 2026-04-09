use bdd::bdd_test;

#[bdd_test(
    feature = "购物车管理",
    scenario = "向空购物车添加商品",
    given = "[\"购物车为空\"]",
    when = "添加一件价格为100元的商品",
    then = "[\"购物车应包含该商品\", \"总价应为100元\"]",
    tags = "[\"cart\", \"add-item\"]",
    priority = "5"
)]
fn test_add_item_to_empty_cart() {
    assert!(true);
}

#[bdd_test(
    feature = "购物车管理",
    scenario = "添加重复商品",
    given = "[\"购物车中已有商品A\"]",
    when = "再次添加相同的商品A",
    then = "[\"商品数量应增加\", \"总价应相应计算\"]",
    tags = "[\"cart\", \"duplicate\"]",
    priority = "3"
)]
fn test_add_duplicate_item() {
    assert!(true);
}
