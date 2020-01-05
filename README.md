# vector-graphics

The aim of this project is having a high-performance vector library that can interpollate vector objects, then it could be used by OpenScisor project for vector edditing purposes and also another project related to physics for animations.

The build target is WebAssembly and all codes should be written in **Rust** and **unit tests should be included**. 

at the end of this project it will supports: 

### The vector objects calcuations such as: 
1. scale, rotate, move, convert
2. location, size, area, volume, angle(s), edge(s)
3. hit_test, intersection, merge, union, ...
4. trace_bitmap

### on the following objects:
#### Basics
1. Point
2. Line
3. Arc
4. Ellipse
5. Rect

#### Advanced objects *(shape/path)*
1. Bézier curve
2. Polygon
3. Path
4. Compund objects
5. GroupedObjects
6. Symetric objects
7. Text (in a sub project)
