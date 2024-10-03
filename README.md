<div align="center">
    <h1> rustic_ml </h1>
    <p><i>A machine learning library created from scratch</i></p>
    <p><strong>Created by Kjetil Indrehus</strong></p>

   <br/>

   <!-- Badges Section -->

   <p>
      <img alt="Rust" src="https://img.shields.io/badge/rust-1.74-orange?logo=rust" />
      <img alt="version" src="https://img.shields.io/crates/v/rustic_ml" />
      <img alt="Downloads" src="https://img.shields.io/crates/dv/rustic_ml" />
   </p>

   <h4>Status</h4>
   <p>
      <img alt="Build" src="https://github.com/KjetilIN/rustic_ml/actions/workflows/test.yml/badge.svg" />
      <img alt="docs passing ci" src="https://img.shields.io/docsrs/rustic_ml" />
   </p>
</div>


## Summary

`rustic_ml` is a machine learning library designed to be easy to use, and give the developer a flexible API to work with.
This library is built of first principles, and the goal is to avoid any dependencies. 


> ⚠️ This library is in the prototype stage. Breaking changes can happen. 


## Table of content <!-- omit in toc -->

- [Summary](#summary)
- [Feature list](#feature-list)
- [Usage](#usage)
- [Use Cases](#use-cases)
  - [Binary classification](#binary-classification)
- [Macros](#macros)
- [Feature Flags](#feature-flags)
- [Deeper Reading](#deeper-reading)


## Feature list 

The library includes the following key features:
- `Matrix` implementation
- `Dataframe` implementation 
- `Perceptron` binary classifier

## Usage

`rustic_ml` has documentation on docs.rs. It will be very usefull to read it through
https://docs.rs/rustic_ml/latest/rustic_ml/

Run the following Cargo command in your project directory:
```
cargo add rustic_ml
```
Or add it to the Cargo manifest. Make sure to pick the newest version:

```toml
[dependencies]
rustic_ml = "0.0.2"
```
Also see the [./examples/](examples/) folder for different examples. 
See also the specific use cases in the next section of the README file.

## Use Cases

### Binary classification 

`rustic_ml` has implemented the `Percetpron`. It works well when you know your data is linearly seperable.
In the example below, we use a Jupyter Notebook with Rust kernal. This makes it easy to build up models with Rust:

![image](https://github.com/user-attachments/assets/29ef6f0c-ab6f-46f9-bc2b-1b748c34e039)

(See the full demo [examples/notebook_binary_classification.ipynb](examples/notebook_binary_classification.ipynb)


## Macros

> Comming soon!

## Feature Flags

> Comming soon!

## Deeper Reading

> Comming soon!
