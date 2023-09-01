<?php
$input    = (int)readline("Password length (leave blank for default): ");
$length   = $input > 0 ? $input : 16;
$password = "";

for ($x = 0; $x < $length; $x++) {
  $password = $password.chr(rand(33, 126));
}

echo $password."\n";
