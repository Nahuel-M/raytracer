Basic raytracer written completely in Rust.

To do:
- [x] Make camera rotate with quaternion around stable axis (without roll)
- [x] Increase efficiency of polygon hit calculation
- [x] Normals for vertices, interpolate normal for polygon
- [x] Add precompute function for shape to speed up rendering
- [x] Basic Bounded Volume Hierarchy
- [x] Replace Rayon for multithreading
- [x] Improve BVH - https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/
- [x] SIMD for BVH - Research and implementation
- [ ] Complete basic shader for Physically Based Rendering
- [ ] Multiple Importance Sampling - http://shihchinw.github.io/2015/06/implementing-ggx-brdf-in-arnold-with-multiple-importance-sampling.html 
- [ ] Consider definitive image and display pipeline
- [x] Transform linear color space to sRGB in final image
- [ ] Extend / rewrite world::parser to accept all .obj / .mtl files, and support textures for materials
- [ ] Extend materials to allow for texture maps