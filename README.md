# lsty.rs
<img src="https://github.com/isemae/lsty/assets/55517023/2a227eb5-dff4-44a3-9c8d-934fd7d3fc66" height="250">



### No more messy download folder!


User-rule-based file manager.


See Commands below for more info!

## Installation
Download binary from releases and move it to your user binary or system binary path.



## Commands


```shell
# Adds a new keyword-target rule.
# Your working directory on shell will be the source path.
lsty add(-a) KEYWORD TARGET_PATH

# You can use any words for keywords, and file extensions too, such as .hwp, .alz, .egg...
```

```shell
# Removes a rule associated with a specified KEYWORD.
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

---


<details markdown="1">
<summary>etc</summary> 


## 'nah why dont ya just make batch files'?
but datz no COOL


<img width="800" alt="took_2_yrs_20240331023409" src="https://github.com/isemae/lsty/assets/55517023/fec0deb1-ceb1-4db0-ab4e-edcd8aaaeb2b">

and not forgetting a goal is COOL

## GUI?

not planned for the moment but


<img src="https://github.com/isemae/lusty.rs/assets/55517023/a9d92a2d-8e9e-49cd-9921-03baa85ef800" alt="fl_or_tr" width="500">

</details>
