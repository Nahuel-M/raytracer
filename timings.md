# Summary of the timings of several renders.

## Used to track the change in speed several optimizations bring

### Progress
| **Improvement**            | **Time before (s)** | **Time after (s)** | **Speed increase factor**                 |
|----------------------------|---------------------|--------------------|-------------------------------------------|
| Bounding box for model     | 120                 | 87                 | Highly dependent on model position (1.27) |
| Improved polygon intersect | 86                  | 15                 | 5.73                                      |
| Added precompute to poly   | 15                  | 9                  | 1.66                                      |
| Implement BVH              | 12.4                | 5.2                | 2.4                                       |

### Current time division of several parts
Only specular:  12.88 seconds
Only diffuse:   13.16 seconds
Both:           18.30 seconds
