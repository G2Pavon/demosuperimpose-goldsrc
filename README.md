# dem2pointfile
Command Line Tool for convert player movements from a `.dem` file into a pointfile `.pts` to load in JACK/Trenchbroom

Original implementation: https://github.com/khanghugo/demosuperimpose-goldsrc/blob/master/src/demo_doer/trenchbroom_player_point.rs
Forked to bring JACK support

#### Usage:

windows
```
dem2pointfile.exe your_demo.dem
```

linux
```
./dem2pointfile your_demo.dem
```

#### Output:

`your_demo.pts`

#### Load:

| JACK | Trenchbroom |
|------|-------------|
| ![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/ed138680-3a27-4d90-be52-418a58d1a40b)| ![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/e19a6f51-7c1a-4a20-904d-400b9613f191)|



