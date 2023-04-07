# Summary of the timings of several renders.

## Used to track the change in speed several optimizations bring

### Progress
| **Improvement**            | **Time before (s)** | **Time after (s)** | **Speed increase factor** |
|----------------------------|---------------------|--------------------|---------------------------|
| Improved polygon intersect | 86                  | 15                 | 5.73                      |
| Added precompute to poly   | 15                  | 9                  | 1.66                      |
| Implement BVH              | 12.4                | 5.2                | 2.4                       |
| Improved BVH               | 24.32               | 9.14               | 2.66                      |
| Re-improved BVH            | 9.14                | 6.41 - 5.59        | 1.43                      |
| SIMD for bounding_box      | 5.59                | 3.30               | 1.69                      |
