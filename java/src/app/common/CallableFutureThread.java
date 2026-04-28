package app.common;

import java.util.concurrent.*;

public class CallableFutureThread {
    // Callable = 반환값 있음 (call())
    static class Worker implements Callable<Integer> {
        private final int id;

        public Worker(int id) {
            this.id = id;
        }

        // run()과 달리 call()은 값 반환 가능, 예외 전달 가능
        @Override
        public Integer call() throws Exception {
            int sum = 0;
            for (int i = 0; i < 3; i++) {
                sum += i;
                System.out.println("Thread " + id + " working...");
                Thread.sleep(500);
            }
            return sum;
        }
    }

    public static void run() throws Exception {
        ExecutorService executor = Executors.newFixedThreadPool(2);
        
        //  Thread Pool에서 스레드 하나 가져옴 -> Worker.call() 작업을 큐에 넣음 -> Future를 반환 (결과를 나중에 받기 위한 핸들)
        Future<Integer> f1 = executor.submit(new Worker(1));
        Future<Integer> f2 = executor.submit(new Worker(2));

        int result1 = f1.get(); // Future.get() = 결과 반환 + 필요하면 대기 (blocking)
        int result2 = f2.get();

        System.out.println("Result1 = " + result1);
        System.out.println("Result2 = " + result2);

        executor.shutdown();
    }

}