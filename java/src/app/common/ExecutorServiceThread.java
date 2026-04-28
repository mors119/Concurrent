package app.common;

import java.util.concurrent.*;

public class ExecutorServiceThread {
    public static void run() throws InterruptedException {
        ExecutorService pool = Executors.newFixedThreadPool(3);

        for (int i = 0; i < 5; i++) {
            int id = i;
            // pool.submit(new Runnable() {@Override public void run() {작업내용}}); 와 아래는 동일
            // Worker 클래스 = 재사용/복잡한 로직, 람다 = 간단한 1회성 작업
            pool.submit(() -> {
                for (int j = 0; j < 2; j++) {
                    System.out.println("Task " + id + " - j=" + j);
                    try {
                        Thread.sleep(500);
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                        return;
                    }
                }
            });
        }

        // 스레드 풀은 계속 살아있음 -> 프로그램이 안 끝날 수도 있음
        pool.shutdown(); // 재출된 내용만 실행하고, 더 이상 새로운 작업 안 받음 (반드시 필요!)
        pool.awaitTermination(10, TimeUnit.SECONDS); // “모든 작업이 끝날 때까지 기다리되 (다음 작업을 대기), 최대 10초까지만 기다린다”

        System.out.println("All tasks finished");
    }
}