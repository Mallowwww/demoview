# Overview
We're trying to create an easier way to view demo files from Source 1 games like TF2 and HL2. This project is nowhere near usable. 
# Build
This project uses Sveltekit for the frontend and Rust for the backend with Tauri. 
## Prerequisites
In order to build Demoview, you will need to install the Visual Studio C++ build tools from the Visual Studio installer; you will also need to install Rust. On Windows 11, the easiest way to do this is to run this command:
```ps
winget install --id Rustlang.Rustup
```
## Steps
To run the app locally, enter the following commands:
```
$ npm i
$ npm run tauri dev
```
To build the project, run:
```
$ npm run tauri build
```
# Roadmap
To make this program work, we need it to be able to:
- [ ] Read and display server messages in a human-readable format
- [ ] Render the map of the demo in-app
- [ ] Render entities in the map as the demo plays
# Community
We have a discord server! Come join the community!
https://discord.gg/pwDKJcA8