# LL(1)语法分析程序

# 实验内容

编写LL(1)语法分析程序,实现对算术表达式的语法分析。要求所分析算数表达式由如下的文法产生:

```
E -> E+T | E-T | T
T -> T*F | T/F | F
F -> (E) | num
```

# 程序设计与实现

**使用方式**：运行`.\ll1-parser.exe "<表达式>"`

1. 生成FIRST和FOLLOW集合`fn get_first(&mut self)` `fn get_follow(&mut self)`
    
    ![Untitled](images/Untitled.png)
    
    ![Untitled](images/Untitled%201.png)
    
    ![Untitled](images/Untitled%202.png)
    
2. 编程实现算法 4.2,为给定文法自动构造预测分析表。`fn get_table(&mut self)`
    
    ![Untitled](images/Untitled%203.png)
    
    生成的预测分析表：
    
    ![Untitled](images/Untitled%204.png)
    
3. 编程实现算法 4.1,构造 LL(1)预测分析程序 。
    
    ![Untitled](images/Untitled%205.png)
    
    程序运行效果如下：
    
    ![Untitled](images/Untitled%206.png)
    
    由于输出较长，故摘取部分输出。
    

# 错误处理

当不能识别输入的表达式时，会抛出错误，程序终止。例如运行`.\ll1-parser.exe "123+456-789*25.2/(7-3))"`：

![Untitled](images/Untitled%207.png)

# 程序输出

执行分析`.\ll1-parser.exe "123+456-789*25.2/(7-3)"`，对表达式**123+456-789*25.2/(7-3)**进行分析的程序输出结果见`output.txt`。

# 源程序

在目录`src/`下。

# 可执行程序

Windows下运行`.\ll1-parser.exe "<表达式>"`，其中`"<表达式>"`为用引号括起来的待分析表达式字符串。

Linux下可运行`./ll1-parser "<表达式>"`。