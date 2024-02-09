# dem2pointfile
Command Line Tool to convert player movements from a `.dem` file into a pointfile `.pts` to load in JACK/Trenchbroom

Original implementation: https://github.com/khanghugo/demosuperimpose-goldsrc/blob/master/src/demo_doer/trenchbroom_player_point.rs

Forked to bring JACK support

---
#### Usage:

download: https://github.com/G2Pavon/demosuperimpose-goldsrc/releases

linux
```
./dem2pointfile your_demo.dem
```

windows
```
dem2pointfile.exe your_demo.dem
```

#### Output:

`your_demo.pts`

---
#### Load:

| JACK | Trenchbroom |
|------|-------------|
| ![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/ed138680-3a27-4d90-be52-418a58d1a40b)| ![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/e19a6f51-7c1a-4a20-904d-400b9613f191)|

---
#### Example:

- Trenchbroom: bkz_goldbhop_promax_0127.68.pts
![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/92d2a64a-f2d3-484c-8cd1-ab4b71a3acfd)
![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/3e26f8ba-eb4e-46c3-9662-69bd7069cf5b)
![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/987de0a0-8062-4b3a-a260-51b881d4106e)
![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/fc91036d-a685-4f21-a797-ea21561cfa3d)



---
- JACK: slide_kei_nefarious_x_BloLy_0109.84.pts
![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/85a9166d-27cd-44d6-9b11-32f66b99a92e)
![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/b75c7a7d-9c69-408c-806c-dad397363ed2)
![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/4495a592-e291-44fc-adab-3b343d2810c5)


# Fly along path (TrenchBroom)

After you open the point file, move the camera to the first point with `View->Camera->Move Camera to Next Point`, and continue hitting `.` hotkey to fly along path.

![image](https://github.com/G2Pavon/demosuperimpose-goldsrc/assets/14117486/a48b2ac9-01f2-460f-b660-0eb920e1d45b)

Video:


![](https://www.youtube.com/watch?v=-1Mhm3EiSpA)

[![](https://img.youtube.com/vi/-1Mhm3EiSpA/0.jpg)](https://www.youtube.com/watch?v=-1Mhm3EiSpA)
----

