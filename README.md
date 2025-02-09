## 学习Rust记录

- [类型和值](#类型和值)
- [控制流基础](#控制流基础)
- [元组和数组](#元组和数组)
- [引用](#引用)
- [用户定义类型](#用户定义类型)
- [模式匹配](#模式匹配)
- [方法和特征](#方法和特征)

#### [类型和值](src/example/type_value)

|                          页面                           |   说明    |
|:-----------------------------------------------------:|:-------:|
|      [hello.rs](src/example/type_value/hello.rs)      |  输出控制台  |
|   [variable.rs](src/example/type_value/variable.rs)   |   变量    |
|        [val.rs](src/example/type_value/val.rs)        | 字面量值的语法 |
|  [operation.rs](src/example/type_value/operation.rs)  |  函数算数   |
| [derivation.rs](src/example/type_value/derivation.rs) |  类型推到   |
|  [fibonacci.rs](src/example/type_value/fibonacci.rs)  |   练习题   |

#### [控制流基础](src/example/control_flow_basics)

|                                      页面                                      |       说明       |
|:----------------------------------------------------------------------------:|:--------------:|
|                [if.rs](src/example/control_flow_basics/if.rs)                |     if表达式      |
|             [cycle.rs](src/example/control_flow_basics/cycle.rs)             |      循环控制      |
| [break_or_continue.rs](src/example/control_flow_basics/break_or_continue.rs) | break&continue |
|            [labels.rs](src/example/control_flow_basics/labels.rs)            |     标签参数循环     |
|             [scope.rs](src/example/control_flow_basics/scope.rs)             |   代码块和作用域、遮蔽   |
|               [fun.rs](src/example/control_flow_basics/fun.rs)               |       函数       |
|             [macro.rs](src/example/control_flow_basics/macro.rs)             |       宏        |
|             [koraz.rs](src/example/control_flow_basics/koraz.rs)             |      练习题       |

#### [元组和数组](src/example/tup_arr)

|                          页面                          |  说明  |
|:----------------------------------------------------:|:----:|
|         [arr.rs](src/example/tup_arr/arr.rs)         |  数组  |
|         [tup.rs](src/example/tup_arr/tup.rs)         |  元组  |
|     [arr_for.rs](src/example/tup_arr/arr_for.rs)     | 数组迭代 |
| [deconstruct.rs](src/example/tup_arr/deconstruct.rs) |  结构  |
|      [nested.rs](src/example/tup_arr/nested.rs)      | 练习题  |

#### [引用](src/example/quote)

|                       页面                       |  说明  |
|:----------------------------------------------:|:----:|
|    [shared.rs](src/example/quote/shared.rs)    | 共享引用 |
| [exclusive.rs](src/example/quote/exclusive.rs) | 独占引用 |
|    [slices.rs](src/example/quote/slices.rs)    |  切片  |
|  [geometry.rs](src/example/quote/geometry.rs)  | 练习题  |

#### [用户定义类型](src/example/customize_type)

|                                   页面                                    |   说明    |
|:-----------------------------------------------------------------------:|:-------:|
|           [structs.rs](src/example/customize_type/structs.rs)           |   结构体   |
|       [tup_structs.rs](src/example/customize_type/tup_structs.rs)       |  元组结构体  |
|             [enums.rs](src/example/customize_type/enums.rs)             |   枚举    |
|           [statics.rs](src/example/customize_type/statics.rs)           |  静态变量   |
|            [consts.rs](src/example/customize_type/consts.rs)            | **常量**  |
|        [type_alias.rs](src/example/customize_type/type_alias.rs)        |  类型别名   |
| [elevator_incident.rs](src/example/customize_type/elevator_incident.rs) | **练习题** |

#### [模式匹配](src/example/pattern_matching)

|                                        页面                                         |     说明      |
|:---------------------------------------------------------------------------------:|:-----------:|
|       [matching_values.rs](src/example/pattern_matching/matching_values.rs)       |     匹配值     |
|        [deconstruct.rs](src/example/pattern_matching/deconstruct_match.rs)        |     解构      |
|      [let_control_flow.rs](src/example/pattern_matching/let_control_flow.rs)      | **let 控制流** |
| [expression_evaluation.rs](src/example/pattern_matching/expression_evaluation.rs) |   **练习题**   |

#### [方法和特征](src/example/method_traits)

|                                      页面                                      |     说明     |
|:----------------------------------------------------------------------------:|:----------:|
|              [methods.rs](src/example/method_traits/methods.rs)              |     方法     |
|               [traits.rs](src/example/method_traits/traits.rs)               |    实现特征    |
|            [traits.rs](src/example/method_traits/more_traits.rs)             |    更多特征    |
|            [traits.rs](src/example/method_traits/shared_types.rs)            |    共享类型    |
|           [traits.rs](src/example/method_traits/derived_traits.rs)           |    派生特征    |
|          [traits_logs.rs](src/example/method_traits/traits_logs.rs)          |    练习题     |
|          [traits_logs.rs](src/example/method_traits/generics_fn.rs)          |    泛型函数    |
|         [traits_logs.rs](src/example/method_traits/generics_type.rs)         |    泛型类型    |
|           [traits_logs.rs](src/example/method_traits/generics.rs)            |     泛型     |
|       [traits_logs.rs](src/example/method_traits/feature_boundary.rs)        |    特征边界    |
| [generics_impl_traits.rs](src/example/method_traits/generics_impl_traits.rs) | impl Trait |
|               [min_fn.rs](src/example/method_traits/min_fn.rs)               |    练习题     |

----

**认真看，多练习**