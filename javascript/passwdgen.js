const prompt = require("prompt-sync")({ sigint: true })
console.clear()

function rand(min, max) {
  return Math.random() * (max - min) + min
}

const input  = parseInt(prompt("Password length (leave blank for default): "))
const length = !isNaN(input) && input > 0 ? input : 16

let password = ""

for (let x = 0; x < length; x++)
  password = password + String.fromCharCode(rand(33, 126))

console.log(password)
