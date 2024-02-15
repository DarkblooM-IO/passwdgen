import java.io.*;
import java.security.SecureRandom;

public class passwdgen {
  public static final SecureRandom RAND = new SecureRandom();
  public static final Console CONSOLE = System.console();
  
  static int randint(int min, int max) {
    return min + RAND.nextInt((max - min) + 1);
  }

  static int getLength() {
    int val;
    try {
      val = Integer.parseInt(CONSOLE.readLine("Password length (leave blank for default): "));
    } catch (NumberFormatException e) {
      val = 16;
    }
    return val > 0 ? val : 16;
  }

  public static void main(String[] args) {
    int length = getLength();
    for (int i = 0; i < length; i++)
      System.out.print((char) randint(33, 126));
    System.out.println("");
  }
}
