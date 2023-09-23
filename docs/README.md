# Calander

Terminal calander for linux written in rust.

This is a command line app which can be used to see the calander within the terminal. It can be used to see the calander of any year between 0 and 4294967295.

![Calander](/docs/screenshot.webp)

## Features

- Interactive
- Simple UI
- Written using rust
- Command line interface

## Usage

Up and down keys can be used to change month and left and right keys can be used to change year

Press **q** to quit and **t** to go to current year's calander

Month and year can be passed as command line arguements in respective order to see the specified calander

## Run

Clone the repository and build the project using cargo

Clone the repository

```
git clone https://github.com/youaremagic/calander
```
Change directory
```
cd calander
```
To build and run the release version in single step
```
cargo run --release
```
