## 1. Cross-compilation setup for a mac machine

- Create a `.cargo/config.toml` file in the root of your rust project that will expose the JNI for use on Android
- Add the necessary linker to the `.cargo/config.toml` file.
   ```sh
   [target.aarch64-linux-android]
    linker = "$ANDROID_HOME/ndk/29.0.14033849/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android30-clang"
    [target.armv7-linux-androideabi]
    linker = "$ANDROID_HOME/ndk/29.0.14033849/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi30-clang"
    [target.i686-linux-android]
    linker = "$ANDROID_HOME/ndk/29.0.14033849/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android30-clang"
    [target.x86_64-linux-android]
    linker = "$ANDROID_HOME/ndk/29.0.14033849/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android30-clang"
   ```
> [!IMPORTANT]
>  You will need to replace `29.0.14033849` with the version of the NDK you want to use. You can list them with `ls $ANDROID_HOME/ndk/`

## 2. Install Android Targets
- Install `rustup` with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` if you don't have it already installed. Make sure you install the official rust from the offical website and not from homebrew. See challenges for more info
- Run the command `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android` all the android targets
- After running the command, you can verify they were correctly installed by running `rustup target list --installed`. It should list the targets below if they were correctly installed
   ```sh
   aarch64-apple-darwin
   aarch64-linux-android
   armv7-linux-androideabi
   i686-linux-android
   x86_64-linux-android
   ```

## 3. Build the rust JNI library
Run the following commands to build the different targets:
```sh
cargo build --target aarch64-linux-android --release && \
cargo build --target armv7-linux-androideabi --release && \
cargo build --target i686-linux-android --release && \
cargo build --target x86_64-linux-android --release
```

## 4. Integrate it with the android code base
Create JNI libraries directories for the different ABIs
```sh
android/app/src/main/jniLibs
├── arm64-v8a
├── armeabi-v7a
├── x86
└── x86_64
```

## 5. Symlink or copy the .so files 
```sh
ln -s target/aarch64-linux-android/release/librusty_todo_jni.so android/app/src/main/jniLibs/arm64-v8a/
ln -s target/armv7-linux-androideabi/release/librusty_todo_jni.so android/app/src/main/jniLibs/armeabi-v7a/
ln -s target/i686-linux-android/release/librusty_todo_jni.so android/app/src/main/jniLibs/x86/
ln -s target/x86_64-linux-android/release/librusty_todo_jni.so android/app/src/main/jniLibs/x86_64/
```

## Challenges

See: https://users.rust-lang.org/t/compile-failed-with-target-aarch64-linux-android/92439