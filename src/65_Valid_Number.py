# 65_Valid_Number
# https://leetcode.cn/problems/valid-number/

# 有效数字（按顺序）可以分成以下几个部分：

# 一个 小数 或者 整数
# （可选）一个 'e' 或 'E' ，后面跟着一个 整数

# 小数（按顺序）可以分成以下几个部分：
# （可选）一个符号字符（'+' 或 '-'）
# 下述格式之一：
# 至少一位数字，后面跟着一个点 '.'
# 至少一位数字，后面跟着一个点 '.' ，后面再跟着至少一位数字
# 一个点 '.' ，后面跟着至少一位数字

# 整数（按顺序）可以分成以下几个部分：
# （可选）一个符号字符（'+' 或 '-'）
# 至少一位数字

# 部分有效数字列举如下：["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]
# 部分无效数字列举如下：["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]

# 给你一个字符串 s ，如果 s 是一个 有效数字 ，请返回 true 。

# 示例 1：
# 输入：s = "0"
# 输出：true

# 示例 2：
# 输入：s = "e"
# 输出：false

# 示例 3：
# 输入：s = "."
# 输出：false
import re
class Solution:
    def isNumber(self, s: str) -> bool:
        return re.match(r"^[+-]?(\.\d+|\d+\.?\d*)([eE][+-]?\d+)?$", s) is not None
    
# 这个问题可以通过编写一个正则表达式来解决。正则表达式是一种用于匹配字符串模式的工具。在这个问题中，我们需要匹配的模式是有效的数字。

# 根据你给出的规则，有效数字的正则表达式可以写成如下形式：


# ^[+-]?(\.\d+|\d+\.?\d*)([eE][+-]?\d+)?$
# 解释一下这个正则表达式：

# ^ 表示字符串的开始。
# [+-]? 表示可选的 '+' 或 '-' 符号。
# (\.\d+|\d+\.?\d*) 匹配小数。它可以是 ".数字" 的形式，或者 "数字.数字" 的形式，或者 "数字." 的形式。
# ([eE][+-]?\d+)? 匹配可选的科学记数法部分。它是 'e' 或 'E' ，后面跟着一个可选的 '+' 或 '-' 符号，然后是至少一位数字。
# $ 表示字符串的结束。




# 有限状态机（Finite State Machine，FSM）是一种可以用来解决这类问题的算法。在这个问题中，我们可以定义一个状态机，
# 其中每个状态表示字符串中的一个位置，每个转移表示消耗掉一个或多个字符并从一个状态移动到另一个状态。

# 我们可以定义以下状态：
    # 开始状态
    # 符号状态（可选）
    # 整数部分状态
    # 小数点状态
    # 小数部分状态
    # 'e' 或 'E' 状态
    # 指数符号状态（可选）
    # 指数部分状态
    # 结束状态
# 然后，我们可以定义从一个状态到另一个状态的转移。例如，从开始状态，我们可以移动到符号状态（如果有 '+' 或 '-'），或者直接移动到整数部分状态。

# 在处理字符串的过程中，我们从开始状态开始，然后根据当前字符和当前状态来决定下一个状态。如果我们可以从最后一个字符移动到结束状态，那么字符串就是一个有效的数字。

# 这是一个在 Python 中的实现示例：
class Solution:
    def isNumber(self, s: str) -> bool:
        # Finite state machine
        state = [
            {'blank': 0, 'sign': 1, 'digit': 2, '.': 3},  # 0. start
            {'digit': 2, '.': 3},  # 1. sign before integer part
            {'digit': 2, '.': 4, 'e': 5, 'blank': 8},  # 2. integer part
            {'digit': 4},  # 3. left dot (no digit yet)
            {'digit': 4, 'e': 5, 'blank': 8},  # 4. decimal part
            {'sign': 6, 'digit': 7},  # 5. 'e' or 'E'
            {'digit': 7},  # 6. sign after 'e' or 'E'
            {'digit': 7, 'blank': 8},  # 7. exponential part
            {'blank': 8}  # 8. end with blank
        ]
        p = 0
        for c in s:
            if '0' <= c <= '9': condition = 'digit'
            elif c in "+-": condition = 'sign'
            elif c in "eE": condition = 'e'
            elif c in ".": condition = '.'
            elif c in " ": condition = 'blank'
            else: condition = '?'
            if condition not in state[p]: return False
            p = state[p][condition]
        return p in (2, 4, 7, 8)

# 在这个函数中，state 是一个列表，表示状态机的所有状态。
# 每个状态都是一个字典，键是条件，值是下一个状态的索引。
# 然后，我们遍历输入的字符串，根据当前字符和当前状态来决定下一个状态。
# 如果我们不能根据当前字符和当前状态找到下一个状态，那么我们就返回 False。
# 最后，如果我们的最后一个状态是 2（整数部分）、4（小数部分）、7（指数部分）或 8（结束状态），那么我们就返回 True，否则返回 False。

# 预备知识

# 确定有限状态自动机（以下简称「自动机」）是一类计算模型。它包含一系列状态，这些状态中：

# 有一个特殊的状态，被称作「初始状态」。
# 还有一系列状态被称为「接受状态」，它们组成了一个特殊的集合。其中，一个状态可能既是「初始状态」，也是「接受状态」。
# 起初，这个自动机处于「初始状态」。随后，它顺序地读取字符串中的每一个字符，并根据当前状态和读入的字符，
# 按照某个事先约定好的「转移规则」，从当前状态转移到下一个状态；当状态转移完成后，它就读取下一个字符。
# 当字符串全部读取完毕后，如果自动机处于某个「接受状态」，则判定该字符串「被接受」；否则，判定该字符串「被拒绝」。

# 注意：如果输入的过程中某一步转移失败了，即不存在对应的「转移规则」，此时计算将提前中止。在这种情况下我们也判定该字符串「被拒绝」。

# 一个自动机，总能够回答某种形式的「对于给定的输入字符串 S，判断其是否满足条件 P」的问题。在本题中，条件 P 即为「构成合法的表示数值的字符串」。

# 自动机驱动的编程，可以被看做一种暴力枚举方法的延伸：它穷尽了在任何一种情况下，对应任何的输入，需要做的事情。

# 自动机在计算机科学领域有着广泛的应用。在算法领域，它与大名鼎鼎的字符串查找算法「KMP 算法」有着密切的关联；在工程领域，它是实现「正则表达式」的基础。

# 在 C++ 文档 中，描述了一个合法的数值字符串应当具有的格式。具体而言，它包含以下部分：

# 符号位，即 +、− 两种符号
# 整数部分，即由若干字符 0−9 组成的字符串
# 小数点
# 小数部分，其构成与整数部分相同
# 指数部分，其中包含开头的字符 e（大写小写均可）、可选的符号位，和整数部分
# 在上面描述的五个部分中，每个部分都不是必需的，但也受一些额外规则的制约，如：

# 如果符号位存在，其后面必须跟着数字或小数点。
# 小数点的前后两侧，至少有一侧是数字。

# 思路与算法

# 根据上面的描述，现在可以定义自动机的「状态集合」了。那么怎么挖掘出所有可能的状态呢？
# 一个常用的技巧是，用「当前处理到字符串的哪个部分」当作状态的表述。根据这一技巧，不难挖掘出所有状态：

# 初始状态
# 符号位
# 整数部分
# 左侧有整数的小数点
# 左侧无整数的小数点（根据前面的第二条额外规则，需要对左侧有无整数的两种小数点做区分）
# 小数部分
# 字符 e
# 指数部分的符号位
# 指数部分的整数部分

# 下一步是找出「初始状态」和「接受状态」的集合。
# 根据题意，「初始状态」应当为状态 0，而「接受状态」的集合则为状态 2、状态 3、状态 5 以及状态 8。
# 换言之，字符串的末尾要么是空格，要么是数字，要么是小数点，但前提是小数点的前面有数字。
# 最后，需要定义「转移规则」。结合数值字符串应当具备的格式，将自动机转移的过程以图解的方式表示出来：

# 比较上图与「预备知识」一节中对自动机的描述，可以看出有一点不同：

# 我们没有单独地考虑每种字符，而是划分为若干类。由于全部 10个数字字符彼此之间都等价，因此只需定义一种统一的「数字」类型即可。对于正负号也是同理。
# 在实际代码中，我们需要处理转移失败的情况。为了处理这种情况，我们可以创建一个特殊的拒绝状态。
# 如果当前状态下没有对应读入字符的「转移规则」，我们就转移到这个特殊的拒绝状态。一旦自动机转移到这个特殊状态，我们就可以立即判定该字符串不「被接受」。

from enum import Enum

class Solution:
    def isNumber(self, s: str) -> bool:
        State = Enum("State", [
            "STATE_INITIAL",
            "STATE_INT_SIGN",
            "STATE_INTEGER",
            "STATE_POINT",
            "STATE_POINT_WITHOUT_INT",
            "STATE_FRACTION",
            "STATE_EXP",
            "STATE_EXP_SIGN",
            "STATE_EXP_NUMBER",
            "STATE_END"
        ])
        Chartype = Enum("Chartype", [
            "CHAR_NUMBER",
            "CHAR_EXP",
            "CHAR_POINT",
            "CHAR_SIGN",
            "CHAR_ILLEGAL"
        ])

        def toChartype(ch: str) -> Chartype:
            if ch.isdigit():
                return Chartype.CHAR_NUMBER
            elif ch.lower() == "e":
                return Chartype.CHAR_EXP
            elif ch == ".":
                return Chartype.CHAR_POINT
            elif ch == "+" or ch == "-":
                return Chartype.CHAR_SIGN
            else:
                return Chartype.CHAR_ILLEGAL
        
        transfer = {
            State.STATE_INITIAL: {
                Chartype.CHAR_NUMBER: State.STATE_INTEGER,
                Chartype.CHAR_POINT: State.STATE_POINT_WITHOUT_INT,
                Chartype.CHAR_SIGN: State.STATE_INT_SIGN
            },
            State.STATE_INT_SIGN: {
                Chartype.CHAR_NUMBER: State.STATE_INTEGER,
                Chartype.CHAR_POINT: State.STATE_POINT_WITHOUT_INT
            },
            State.STATE_INTEGER: {
                Chartype.CHAR_NUMBER: State.STATE_INTEGER,
                Chartype.CHAR_EXP: State.STATE_EXP,
                Chartype.CHAR_POINT: State.STATE_POINT
            },
            State.STATE_POINT: {
                Chartype.CHAR_NUMBER: State.STATE_FRACTION,
                Chartype.CHAR_EXP: State.STATE_EXP
            },
            State.STATE_POINT_WITHOUT_INT: {
                Chartype.CHAR_NUMBER: State.STATE_FRACTION
            },
            State.STATE_FRACTION: {
                Chartype.CHAR_NUMBER: State.STATE_FRACTION,
                Chartype.CHAR_EXP: State.STATE_EXP
            },
            State.STATE_EXP: {
                Chartype.CHAR_NUMBER: State.STATE_EXP_NUMBER,
                Chartype.CHAR_SIGN: State.STATE_EXP_SIGN
            },
            State.STATE_EXP_SIGN: {
                Chartype.CHAR_NUMBER: State.STATE_EXP_NUMBER
            },
            State.STATE_EXP_NUMBER: {
                Chartype.CHAR_NUMBER: State.STATE_EXP_NUMBER
            },
        }

        st = State.STATE_INITIAL
        for ch in s:
            typ = toChartype(ch)
            if typ not in transfer[st]:
                return False
            st = transfer[st][typ]
        
        return st in [State.STATE_INTEGER, State.STATE_POINT, State.STATE_FRACTION, State.STATE_EXP_NUMBER, State.STATE_END]