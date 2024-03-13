# lusty.rs

## Tidy Up Your Download Folder!

![Download Folder](다운로드폴더이미지.jpg)

- Move all the directories you want to sort by just one command line!

Each keyword-target pair is avaliable only for its source, which is the current path on the shell. So, you'll be working on your shell path always!

- Commands
  - `lsty add(-a) keyword /target_dir`
    - adds a new keyword-target directory pair
  - `del(-d) keyword`
    - deletes the pair which has the keyword in its pair
  - `move(-m) <keyword>`
    - move all files that have <keyword> in its filename, 
  - `move(-m)`
    - or you can just move all the files without keyword