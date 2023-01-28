# Summary of the timings of several renders.

## Used to track the change in speed several optimizations bring

| **Improvement**            | **Time before (s)** | **Time after (s)** | **Speed increase factor**                 |
|----------------------------|---------------------|--------------------|-------------------------------------------|
| Bounding box for model     | 120                 | 87                 | Highly dependent on model position (1.27) |
| Improved polygon intersect | 86                  | 15                 | 5.73                                      |
| Added precompute to poly   | 15                  | 9                  | 1.66                                      |