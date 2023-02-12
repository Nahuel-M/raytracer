# This file keeps track of ongoing plans, derivations and resources on Physically Based Rendering.
The current goal is the implementation of a (hopefully) general BSDF shader.

## Snell's law:
https://graphicscompendium.com/raytracing/10-reflection-refraction

## BRDF

Before taking into account refraction, first focus on diffuse and specular reflection.
Light emitted from surface (angle, position) = Material Luminance + sum ( BRDF * )