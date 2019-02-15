# Flow
> 一种脚本语言，用于处理流式数据（包括音频，视频，自动控制信息，Midi和一些其它的流式数据）。

[English Document](README.md)

现有的音视频处理工具（包括DAW，编辑器，效果器等）依旧采用的传统的算法来进行处理与分析数据。而随着大数据与AI数据的发展，我们已经拥有了更加好用的方式来处理这些数据数据来取得更好的效果，但现有工具的设计难以将其与AI新的算法与机制结合。Flow的目标就是创建一种新的用于处理音视频，自动控制，Midi序列等流式数据的工具，同时也能够对接传统的插件机制（VST，OpenFX等）与现有的大数据与AI处理机制（Tensorflow，Coffee等）。

## 设计原理

现有的音频，视频，自动控制信号，Midi等本质上属于一种流式数据。在Flow的设计中，数据就像是水流一样沿着Wire移动，不断的经过一个又一个模块。每经过一个模块数据都有可能被经过处理计算等。Flow采用FlowScript来表示这些模块，以及这些模块间的连接关系。

### 模块

模块是构成Flow的最基础单位，模块用于处理流式数据。流式数据流入模块，经过模块处理之后流出模块。模块可以被表示为一个图，图的节点是子模块或接口，每个节点可以流入流出数据；而图的边则是描述了节点间的连接关系，表示数据的流动。

一个模块由两部分组成，分别为Variables与Wires。

#### Variables

Variables是模块的基础组成部分，表示的是模块图的节点。

Variables可以是输入接口，输出接口，子模块等。

#### Wires

Wires表示的是模块图中的边。

Wire只能在Variable的输入输出接口之间进行连接。其中数据只能从模块的输入指向模块的输出。

### 钩子

钩子是模块用于调用外部系统的接口，

### 脚本

### FlowScript

FlowScript是以`flow`为后缀的脚本语言。

#### 注释

FlowScript使用`//`标记单行注释

```
// This is a comment.
```

#### 版本需求声明

版本需求声明表达了此脚本执行所需要的Flow版本。

```
version <require>;
```

其中version是关键字，后跟版本号声明。以分号结束。例如：

```
version ^0.1.0; //Flow version must greater than 0.1.0
version 0.1.0; //Flow version must equal 0.1.0
```

#### 导入语句

导入语句负责向当前FlowScript中导入资源，这些资源包括模块，钩子，脚本等内容。

```
import <package> [as <alias>];
```

其中import是必须的的关键字，后面跟上包名。也可以使用as字句来对这个包其别名。

例如：

```
import Flow; //import package named flow.
import Flow.Port as Port //import Port under Flow and alias it Port.
```

#### 模块声明

模块声明记录了如何构造一个模块，以及这个模块中各个子模块之间的Wires连接关系。

```
module <name> {
    <variable_name>: <type>; //Variable declare.
    <wire_name>: <variable_name_1> -> <variable_name_2> //Wire declare.
}
```

其中module为关键字，后跟模块名。模块声明内容写在花括号中。

模块声明中每一行则是一个声明语句。每一个声明语句由`:`分割，分号前是Variables或Wires名称，分号后是类型。名称可以使用任意UTF-8字符，但不能以数字开头，其中ASCII码范围内的符号只能存在_。

若类型是一个单独的类型，则声明的是Variable。可以利用逗号分割多个变量名，实现同时声明多个变量。

若类型是一个使用`->`连接的两个变量，则声明的是一个Wire，两个变量必须类型相同。声明的Wire同时只能声明一个，如果不需要为此Wire起名，则可以使用`_`代替名称。

例如：

```
module Test {
    blockdoor: Flow.Blockdoor; // Declare a variable called blackdoor, type is Flow.Blockdoor.
    bypass,mute: Flow.Port.Bool; // Declare a variable called mute, type is Flow.Port.Bool.
    in: Flow.Port.Audio;
    no_meaning: bypass -> mute; // Declare a wire called no_meaning.
    _: in -> blockdoor.in; // Declare a wire without name.
}
```

#### 钩子声明

#### 脚本声明

## 使用方式

## 参考

### FlowScript API

#### Package Flow

Package Flow是FlowScript提供的核心API，这些API包括端口与一些默认的模块。

- Flow.Port 包含所有默认类型。
- Flow.Core 包含默认的模块。

##### Flow.Port

Flow.Port包含如下的hook:

- onChange - 当值发生变化时，调用此hook。
- onData - 当有新值产生时，调用此hook。

##### Flow.Port.Bool

继承自Flow.Port，可以取值True或False。

Bool中包含如下hook：

- onTrue - 当值为True时，调用此hook。
- onFalse - 当值为False时，调用此hook。




### C API

## 许可证

本项目在LGPL-3.0下发布。
