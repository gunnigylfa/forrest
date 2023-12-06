## Binary Merkle Trees

Given a binary tree:

```mermaid
    flowchart TD
        subgraph root
            0(["0"])
        end
        subgraph intermediate nodes
            1(["1"])
            2(["2"])
            3(["3"])
            4(["4"])
            5(["5"])
            6(["6"])
        end
        subgraph leaves
            7(["7"])
            8(["8"])
            9(["9"])
            10(["10"])
            11(["11"])
            12(["12"])
            13(["13"])
            14(["14"])
        end
        0 --> 1
        0 --> 2
        1 --> 3
        1 --> 4
        2 --> 5
        2 --> 6
        3 --> 7
        3 --> 8
        4 --> 9
        4 --> 10
        5 --> 11
        5 --> 12
        6 --> 13
        6 --> 14
```

The single top node is called the root, the lowest nodes are called leaves and the remaining nodes are called intermediate nodes. Layers are counted zero based from the root, called the depth. The number of layers is called the depth of the tree. The nodes are numbered zero-based left-to-right and top-to-bottom called their index.

## Index calculus

In addition to the index, we can also identify the nodes by their depth and offset from the left:

```mermaid
flowchart TD
    0(["(0,0)"])
    1(["(1,0)"])
    2(["(1,1)"])
    3(["(2,0)"])
    4(["(2,1)"])
    5(["(2,2)"])
    6(["(2,3)"])
    0 --> 1
    0 --> 2
    1 --> 3
    1 --> 4
    2 --> 5
    2 --> 6
```
