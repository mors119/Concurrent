package app;

public class Main {
    public static void main(String[] args) {
        if (args.length < 2) {
            System.out.println("사용법: make run ARGS=\"<category> <number>\"");
            return;
        }
        Matcher.run(args);
    }
}