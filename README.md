# Rotational Spectrum Simulator

## Purpose
Develop a spectrum simulator for symmetric and asymmetric rotational molecules in Rust.

Initially, development will focus on creating a CLI version.

If a GUI is to be developed, it will be created using JS/TS to develop both web and desktop versions (using tools like Tauri).

## Current Progress

Currently working on the simulator for symmetric rotational molecules.

## Memo
Python
I wanted to run matplotlib interactively in a poetry environment.

However, installing matplotlib with poetry add matplotlib didn't work properly with the GUI backend's PyQt5 related packages and didn't run.

So, I installed PyQt6 instead of PyQt5 and then installed matplotlib.

It worked properly in this case.

```bash
$ poetry add PyQt6
$ poetry add matplotlib
```
