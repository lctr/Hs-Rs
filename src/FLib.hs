module FLib where

import Foreign.C.String
import Foreign.C.Types

-- call the rusty function, treating i32 as CInt
foreign import ccall "double_input" doubleInput :: CInt -> CInt

foreign import ccall unsafe "print_string" printString :: CString -> IO ()
