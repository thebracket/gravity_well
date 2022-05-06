# Gravity Well

[Play online here](https://bfnightly.bracketproductions.com/gravity_well/)

> When I'm working on book proposals, I typically prototype a lot of projects. Not all of them make the cut. This is a project from the cutting room floor.

![image](https://user-images.githubusercontent.com/14896751/167146456-39aba57d-4993-408a-983e-d44b9b694e76.png)

*Gravity Well* is a simple two-player (shared keyboard) game. Fly around a gravity well, collecting salvage. Bounce one another, accelerate and rotate. Whomever collects the most salvage wins.

This isn't intended to be a finished game; I'm hoping that someone finds the source code useful/instructive. Elements in the code:

* **Assets**: a basic asset manager to specify your sprite atlases up-front (with the builder pattern), and combine them into a single `Assets` resource.
* **Menu Framework**: a simple framework for displaying a main menu and game over screen, as well as a "Loading, please wait" message while your assets finish loading.
* **Physics**: some very primitive collision detection, velocity and gravity code.
* **Centered Text**: I added a bundle for centered text, just because I find the normal Bevy code verbose.
* **Particles**: a very simple particle system. Not at all optimized.
