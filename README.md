# lsty.rs
<img src="https://github.com/isemae/lsty/assets/55517023/aac9dba7-00bc-4a2e-95d9-dc0efe9a6bce" height="250">

### No more messy download folder!


A Keyword-based file manager.


See Commands below for more info!


### !USE AT YOUR OWN RISK!
This binary is not guaranteed to work on all environments, operating systems or machines.
Therefore it can produce any unexpected or destructive results. Please use it at your own discretion.



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

</details>
