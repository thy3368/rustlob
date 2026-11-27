/// line的处理方式 在编排每个pipe,
/// 会根据前面pipe的执行结果，决定后面哪些pipe执行，所以result的设计需要能表达这些信息，
/// line自己的result基本是pipe的组合
pub mod place_spot_order_pipeline_v2;
