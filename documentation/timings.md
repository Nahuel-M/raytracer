# Summary of the timings of several renders.

## Used to track the change in speed several optimizations bring

### Progress
| **Improvement**            | **Time before (s)** | **Time after (s)** | **Speed increase factor**                 |
|----------------------------|---------------------|--------------------|-------------------------------------------|
| Improved polygon intersect | 86                  | 15                 | 5.73                                      |
| Added precompute to poly   | 15                  | 9                  | 1.66                                      |
| Implement BVH              | 12.4                | 5.2                | 2.4                                       |
