# Introduction

- Welcome
- This is me
- This is "UXStream"

- Topic: Rust for Graphics, Games, and GPUs

# Why

- What problems are we trying to solve?

  - Performance
    - Stops being about hacking around the language
      - The fact that in something python using a for loop is considered bad practice 
        is tragically ridiculous
        - Instead you have to make sure you use something like NumPy to make your 
          machine run as few lines of actual python as possible
      - With computing power moving towards servers, we don't have to focus on optimizing nanoseconds
        to fit our execution cycles into a 9 year old phone. But we can focus on using the boost Rust gives us
        to make our code do more fancy stuff
  - Correctness
    - I am using this an umbrella term for
      - Safety
      - Reliability
      - Predictability
      - Maintainability
      - Readability
      - everything else
    - Why?
      - Square plug go in square hole
      - The solid type system allows us to develop without the fear of breaking too much stuff
  - Flexibility
    - **Pluggable** systems
    - Despite enforcing decoupling from the business logic, Low Level access to the hardware is always possible
    - Expandible syntax
      - Don't want the users of your library to write boilerplate code?
        - Generate it for them


GLitch: https://www.shadertoy.com/view/4dXBW2

Seascape: https://www.shadertoy.com/view/Ms2SD1
Rust-GPU Port: https://github.com/Rust-GPU/rust-gpu-shadertoys

# Ecosystem

## Bevy
- The most mature Rust game Development Framework

## Rust-GPU

## WGPU

# Entity Component System

- A Data-Oriented design pattern that works amazingly well with Rust

## Demo

- Empty world
- Add an entity
  - Nothing happens
  - Empty axis gizmo

- Add a component
  - Stuff shows up
  - Sphere where the gizmo is

- Add a system
  - Stuff moves around
  - The sphere moves around 
