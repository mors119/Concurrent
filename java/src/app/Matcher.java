package app;

public class Matcher {
    public static void run(String[] args) throws InterruptedException, Exception {
        String category = args[0];
        String number = args[1];

        if (category.equals("common")) {
            if (number.equals("1")) {
                app.common.BasicThread.run();
                return;
            }
            if (number.equals("2")) {
                app.common.CallableFutureThread.run();
                return;
            }
            if (number.equals("3")) {
                app.common.ExecutorServiceThread.run();
                return;
            }
            System.out.println("common 카테고리의 해당 번호가 없습니다.");
            return;
        }
        System.out.println("알 수 없는 category 입니다.");
    }
}