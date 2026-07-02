# LL(1) 分析报告

## 原始文法

- 1. Expr -> Term ExprP
- 2. ExprP -> + Term ExprP
- 3. ExprP -> ε
- 4. Term -> Factor TermP
- 5. TermP -> * Factor TermP
- 6. TermP -> ε
- 7. Factor -> ( Expr )
- 8. Factor -> id

## 符号说明

- $：输入结束符，在教材中通常表示输入串已经读完，也常作为分析栈的栈底标记
- (：终结符 (，表示左括号，是输入串中可直接匹配的符号
- )：终结符 )，表示右括号，是输入串中可直接匹配的符号
- *：终结符 *，表示乘法运算符，是输入串中可直接匹配的符号
- +：终结符 +，表示加法运算符，是输入串中可直接匹配的符号
- Expr：表达式（expression），表示这一套文法里最外层的运算结构，也是当前样例里的主要开始分析对象
- ExprP：表达式后续部分，通常表示在一个项之后，还可以继续接上加法一类的递归扩展
- Factor：因子（factor），表示最基础的可参与运算单元，例如标识符或括号括起来的子表达式
- Term：项（term），表示表达式里优先级更高的一层结构，通常先于加法层被识别
- TermP：项的后续部分，通常表示在一个因子之后，还可以继续接上乘法一类的递归扩展
- id：终结符 id，通常表示标识符，是可以被分析器直接匹配的输入记号
- ε：空串，在教材中表示该产生式可以不消耗任何输入符号而直接推出空

## FIRST 集

- Expr = {(, id}
- ExprP = {+, ε}
- Factor = {(, id}
- Term = {(, id}
- TermP = {*, ε}

## FOLLOW 集

- Expr = {$, )}
- ExprP = {$, )}
- Factor = {$, ), *, +}
- Term = {$, ), +}
- TermP = {$, ), +}

## 预测分析表

- M[Expr, (] = 1
- M[Expr, id] = 1
- M[ExprP, $] = 3
- M[ExprP, )] = 3
- M[ExprP, +] = 2
- M[Factor, (] = 7
- M[Factor, id] = 8
- M[Term, (] = 4
- M[Term, id] = 4
- M[TermP, $] = 6
- M[TermP, )] = 6
- M[TermP, *] = 5
- M[TermP, +] = 6

## 预测分析结果

### 输入串: id + id * id

- 结论: 接受

| 步骤 | 分析栈 | 剩余输入 | 动作 |
| --- | --- | --- | --- |
| 1 | Expr $ | id + id * id $ | expand Expr -> Term ExprP |
| 2 | Term ExprP $ | id + id * id $ | expand Term -> Factor TermP |
| 3 | Factor TermP ExprP $ | id + id * id $ | expand Factor -> id |
| 4 | id TermP ExprP $ | id + id * id $ | match id |
| 5 | TermP ExprP $ | + id * id $ | expand TermP -> ε |
| 6 | ExprP $ | + id * id $ | expand ExprP -> + Term ExprP |
| 7 | + Term ExprP $ | + id * id $ | match + |
| 8 | Term ExprP $ | id * id $ | expand Term -> Factor TermP |
| 9 | Factor TermP ExprP $ | id * id $ | expand Factor -> id |
| 10 | id TermP ExprP $ | id * id $ | match id |
| 11 | TermP ExprP $ | * id $ | expand TermP -> * Factor TermP |
| 12 | * Factor TermP ExprP $ | * id $ | match * |
| 13 | Factor TermP ExprP $ | id $ | expand Factor -> id |
| 14 | id TermP ExprP $ | id $ | match id |
| 15 | TermP ExprP $ | $ | expand TermP -> ε |
| 16 | ExprP $ | $ | expand ExprP -> ε |
| 17 | $ | $ | accept |

### 输入串: ( id + id ) * id

- 结论: 接受

| 步骤 | 分析栈 | 剩余输入 | 动作 |
| --- | --- | --- | --- |
| 1 | Expr $ | ( id + id ) * id $ | expand Expr -> Term ExprP |
| 2 | Term ExprP $ | ( id + id ) * id $ | expand Term -> Factor TermP |
| 3 | Factor TermP ExprP $ | ( id + id ) * id $ | expand Factor -> ( Expr ) |
| 4 | ( Expr ) TermP ExprP $ | ( id + id ) * id $ | match ( |
| 5 | Expr ) TermP ExprP $ | id + id ) * id $ | expand Expr -> Term ExprP |
| 6 | Term ExprP ) TermP ExprP $ | id + id ) * id $ | expand Term -> Factor TermP |
| 7 | Factor TermP ExprP ) TermP ExprP $ | id + id ) * id $ | expand Factor -> id |
| 8 | id TermP ExprP ) TermP ExprP $ | id + id ) * id $ | match id |
| 9 | TermP ExprP ) TermP ExprP $ | + id ) * id $ | expand TermP -> ε |
| 10 | ExprP ) TermP ExprP $ | + id ) * id $ | expand ExprP -> + Term ExprP |
| 11 | + Term ExprP ) TermP ExprP $ | + id ) * id $ | match + |
| 12 | Term ExprP ) TermP ExprP $ | id ) * id $ | expand Term -> Factor TermP |
| 13 | Factor TermP ExprP ) TermP ExprP $ | id ) * id $ | expand Factor -> id |
| 14 | id TermP ExprP ) TermP ExprP $ | id ) * id $ | match id |
| 15 | TermP ExprP ) TermP ExprP $ | ) * id $ | expand TermP -> ε |
| 16 | ExprP ) TermP ExprP $ | ) * id $ | expand ExprP -> ε |
| 17 | ) TermP ExprP $ | ) * id $ | match ) |
| 18 | TermP ExprP $ | * id $ | expand TermP -> * Factor TermP |
| 19 | * Factor TermP ExprP $ | * id $ | match * |
| 20 | Factor TermP ExprP $ | id $ | expand Factor -> id |
| 21 | id TermP ExprP $ | id $ | match id |
| 22 | TermP ExprP $ | $ | expand TermP -> ε |
| 23 | ExprP $ | $ | expand ExprP -> ε |
| 24 | $ | $ | accept |

### 输入串: id + + id

- 结论: 拒绝
- 错误: 预测分析表缺少条目: (Term, +)

| 步骤 | 分析栈 | 剩余输入 | 动作 |
| --- | --- | --- | --- |
| 1 | Expr $ | id + + id $ | expand Expr -> Term ExprP |
| 2 | Term ExprP $ | id + + id $ | expand Term -> Factor TermP |
| 3 | Factor TermP ExprP $ | id + + id $ | expand Factor -> id |
| 4 | id TermP ExprP $ | id + + id $ | match id |
| 5 | TermP ExprP $ | + + id $ | expand TermP -> ε |
| 6 | ExprP $ | + + id $ | expand ExprP -> + Term ExprP |
| 7 | + Term ExprP $ | + + id $ | match + |
| 8 | Term ExprP $ | + id $ | error: no rule for Term with + |

