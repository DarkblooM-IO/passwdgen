fun main() {
	print("Password length (leave blank for default): ")
	val length = maxOf(readLine()?.toIntOrNull() ?: 16, 1)

	for (i in 1..length) {
		print((33..126).random().toChar())
	}
	println("")
}
