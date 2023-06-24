# flood
=================================================

[![License](https://img.shields.io/badge/License-CC0-lightgray.svg?style=flat-square)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Latest release](https://img.shields.io/github/v/release/mhucka/readmine.svg?style=flat-square&color=b44e88)](https://github.com/mhucka/readmine/releases)
[![DOI](http://img.shields.io/badge/DOI-10.22002%20%2f%20D1.20173-blue.svg?style=flat-square)](https://data.caltech.edu/records/20173)


Table of contents
-----------------

* [Introduction](#introduction)
* [Usage](#usage)

Introduction
------------

This project is based on a Stanford Nifty Assignment called Rising Tides (http://nifty.stanford.edu/2023/schwarz-rising-tides/).

The maps in "terrain" are from the Nifty assignment, however, you can create your own 

Usage
-----

You can run this project by using 
```bash 
cargo run
``` 

### Basic operation


<img src="./stuff/useage.gif" />

When you run the program, you will be prompted to select which map to load. Select the map by the index in the list and you will then be able to enter the water level.

To create a terrain map, create a new file ending with .terrain and place it in the terrain folder. The format of the folder should be as follows:

```
local
[rows]
[cols]
[water sources]
[water source row]
[water source col]
[MAP]
```

NOTE: As of now, the number placed in rows, cols, and water sources does not matter. Only one water source works as of now.

