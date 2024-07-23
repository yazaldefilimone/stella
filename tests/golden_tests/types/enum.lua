enum Color {
  Red = 1,
  Green = 2,
  Blue
}

enum Result {
  Ok = "ok",
  Err = "error"
}

local color: Color = Color.Red
color = Color.Green
color = Color.Blue

if color == Color.Red then
  print("Red")
elseif color == Color.Green then
  print("Green")
elseif color == Color.Blue then
  print("Blue")
else
  print("Unknown color")
end
