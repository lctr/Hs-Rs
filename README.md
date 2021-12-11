# Haskell-Rust FFI 

Small explorations in Haskell-Rust interop.

## Notes
1. Declare in `Cargo.toml` the name of the package, here `rusty` as a statically linked library. 
    ```toml
    [lib]
    name = "rusty"
    crate-type = ["staticlib"]
    ```
    The name of the package will be used in the corresponding `hsrs.cabal` project file. In particular, with the FFI as a separate Haskell library (from the (Haskell) `Main` module in *Main.hs*). 
    
    Both the main executable and the FFI library sections of the *rshs.cabal* file (which should be in the same directory as the *Cargo.toml* file), require the `rusty` package (along with another, `pthread`) to be added.

    ```cabal
    executable hsrs-exec
        main-is:             Main.hs
        hs-source-dirs:      src
        build-depends:       base >= 4.7 && < 5
        default-language:    Haskell2010
        other-modules:       FLib
        extra-libraries:     rusty
                           , pthread
        extra-lib-dirs:      target/release

    library
        hs-source-dirs:      src
        exposed-modules:     FLib
        other-extensions:    ForeignFunctionInterface
        build-depends:       base >= 4.7 && < 5
        default-language:    Haskell2010
        extra-libraries:     rusty
                           , pthread
        extra-lib-dirs:      target/release
    ```

Sources:
* [1](https://blog.mgattozzi.dev/haskell-rust/)