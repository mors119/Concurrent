package app;

public class Main {
    public static void main(String[] args) {
        if (args.length < 2) {
            System.out.println("사용법: make run ARGS=\"<category> <number>\"");
            return;
        }
        try {
            Matcher.run(args);
        } catch (InterruptedException e) {
            System.out.println("인터럽트 발생!");
        } catch (Exception e) {
            System.out.println("예외 발생!");
        }
    }
}