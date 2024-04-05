# lsty.rs
<img src="https://github.com/isemae/lsty/assets/55517023/c53f2594-c293-4646-99d3-4ab3bef91085" height="200">


### No More Messy Download Folder!



"lsty" simplifies file management by source-keyword-target triplets. The source path is a shell path when its keyword-target rule is added.
Because your current shell path will always be one of the source paths, means you don't have to bother inputting source, target paths every time. 

Just command, keyword, one target path(if you need).



## Commands


```shell
# Adds a new keyword-target rule.
lsty add(-a) KEYWORD TARGET_PATH

# You can use any words for keywords, and file extensions too, such as .hwp, .alz, .egg...
```

```shell
# Removes the rule associated with the specified KEYWORD.
# Shows available keywords if the input KEYWORD is not found.
lsty del(-d) KEYWORD
```

```shell
# Updates a rule that has KEYWORD as its key.
lsty edit(-e) KEYWORD KEYWORD_OR_PATH
```

```shell
# Moves all entries with <keyword> in their names.
lsty move(-m) <KEYWORD>
# Moves all entries from a source path to each respective target path without <KEYWORD> input.
     move(-m)
```

```shell
# Scans current path and shows entries that satisfy any rules of the current path, or for a specified rule by <KEYWORD>.
# You can move entries to their target pathes by giving additional input 'm'.
# 'q' or 'ESC' to cancel.
lsty scan(-s) <KEYWORD>
```

```shell
# Sets <keyword> as an alias for current path.
# The alias can be used for specifying the source path to import rules from. 
lsty alias(-al) KEYWORD
```

```shell
# Imports rules from another path to the current path.
# You can specify the path to import from by <ALIAS> or <SOURCE_PATH>.
lsty import(-i) ALIAS_OR_PATH
```

## Installation

macOS

```shell
# Homebrew
```

Windows

```shell
# Chocolatey
```

<details markdown="1">
<summary>etc</summary> 


## 'nah why dont ya just make batch files'?
but datz no COOL


<img width="800" alt="took_2_yrs_20240331023409" src="https://github.com/isemae/lsty/assets/55517023/0c3469c3-6af7-48c4-a079-ab1dab45b784">

and not forgetting a goal is COOL

## GUI?
<img src="https://github.com/isemae/lusty.rs/assets/55517023/a9d92a2d-8e9e-49cd-9921-03baa85ef800" alt="fl_or_tr" width="500">

</details>
