
Swift Cmake Examples: [https://github.com/compnerd/swift-build-examples](https://github.com/compnerd/swift-build-examples)

Build:

```
cmake -G Ninja -B build -DCMAKE_BUILD_TYPE=RelWithDebInfo -DBUILD_TESTING=YES
cd build
ninja
ninja test
```