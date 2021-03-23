# Maze Generator

This maze generator is a rewrite of a college data structures assignment.

The goal is to create a random maze with only one correct path between entrance and exit, utilizing a Disjoint Set.


## Build and run


```bash
cargo build
```

run the executable with arguments for width and height
```
.\target\debug\mazegen.exe --width 20 --height 20
 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
|_ _ _       _|_   _|  _ _       _   _  |
|  _|  _|_|_  |_  |_ _ _ _| |_|_ _|_| | |
|  _ _|_   _ _|_   _  | |_     _| |     |
|_|  _| |   |_   _|_   _ _ _| |    _|_|_|
|_  |  _| |_|_     _|_ _| |  _|_|_| |_  |
  |   |  _  | | |  _|   |_ _|_ _  |  _ _|
|_  |_| |  _| | |  _|_|_   _| | |_   _  |
|    _ _|_|  _| |_|_ _  | | |  _ _    | |
|_|_  |_   _    |     |   |  _  |_  | | |
|  _ _| | | |_|_|_|_|_  |_|_|   |_ _| |_|
|_    |  _  |_ _ _    |_ _   _|_ _| | | |
|_  |   |_  |_ _ _ _|  _ _ _ _ _| |_  |
|_  | |   |  _ _| |_ _|    _| |  _| | | |
|_  | |_|_| | |_        |_|_ _  |  _|   |
| |_| |_ _ _| |_ _| |_| |_   _ _   _ _|_|
|  _ _| | |  _|  _   _|_| | | |  _ _  | |
| |_  |  _ _|_ _  |_ _|_  | |_ _ _ _|_| |
|_ _   _|_  |_   _|      _| |  _ _ _|  _|
|  _| |_ _   _| | | |_|  _ _|_   _ _ _| |
|_ _ _ _ _ _ _ _ _ _ _|_ _ _ _ _ _ _ _ _|


```

## License
[MIT](https://choosealicense.com/licenses/mit/)
