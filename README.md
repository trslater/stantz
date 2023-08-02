# Stantz

> What? What just popped in there?

Named after the "heart of the Ghostbusters," Ray Stantz, **Stantz** is a path tracer.

## Requirements

- Eigen3
- SDL2
- CMake
- Make

## Setup

This project uses CMake and requires a build directory, usually just called `build`. All instructions from here on will assume that the build directory exists, and that it is the current working directory.

## Building

To configure the build (should only need to be done once):

```
$ cmake ..
```

To build the project:

```
$ cmake --build .
```

## Usage

```
$ ./test <width> <height>
```

`width` and `height` are the output image size in pixels.

## Roadmap

- [ ] C++ Performance
  - Switching to C++ greatly degraded performance
- [ ] Modernize C++
- [ ] Path tracing
- [ ] General performance
  - More efficient
  - Multithreading? Multiprocessing?
- [ ] Triangle meshes
