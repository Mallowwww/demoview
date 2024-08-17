# Overview
We're trying to create an easier way to view demo files from Source 1 games like TF2 and HL2. This project is nowhere near usable. 
# Build
This project uses Sveltekit for the frontend and Rust for the backend with Tauri. To run the app locally, enter the following commands:
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