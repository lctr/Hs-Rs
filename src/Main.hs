import FLib
import Foreign.C.String (newCString)

main :: IO ()
main = do
  -- print 6
  print $ doubleInput 3
  -- newCString doesn't need the \0, but it was found that Rust consumed the last character regardless of value, likely since Rust Strings aren't null-terminated ?? hence the explicit null-termination
  str <- newCString "Hello World\0"
  printString str