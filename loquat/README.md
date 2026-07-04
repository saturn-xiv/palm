# Usage

```bash
cmake -Wno-dev -DCMAKE_BUILD_TYPE=Release -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF -B build -S . -G Ninja
cmake --build build
```
