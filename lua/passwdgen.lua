math.randomseed(math.floor(os.clock()*1000000))

function get_length()
  io.write("Password length (leave blank for default): ")
  local uinput = io.read()
  return tonumber(uinput) or 16
end

local password = ""
local length = get_length()

for n = 1, length do
  password = password .. string.char(math.random(33, 126))
end

print(password)
