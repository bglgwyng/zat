module Sample where

import Data.Map (Map)

greet :: String -> String
greet name = "Hello, " ++ name ++ "!"

add :: Int -> Int -> Int
add a b = a + b

data Color
  = Red
  | Green
  | Blue

newtype Name = Name String

type Mapping = Map String String

class Printable a where
  display :: a -> String
  label :: a -> String

instance Printable Color where
  display Red = "red"
  display Green = "green"
  display Blue = "blue"
  label _ = "color"

infixl 6 `add`
