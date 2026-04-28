package app.common;  // 하위 폴더는 패키지 선언 필요

public class BasicThread {

    // Thread = 실행 엔진
    // Runnable = 실행할 작업 (Runnable 인터페이스가 표준)
    static class Worker implements Runnable { // extends Thread도 가능하지만 다른 클래스 상속 중이면 사용 못함.
        private final int id;

        public Worker(int id) {
            this.id = id;
        }

        // Thread는 “run()을 호출하도록 설계되어 있음”
        @Override
        public void run() {
            for (int i = 0; i < 3; i++) {
                System.out.println("Thread " + id + " - i=" + i);
                try {
                    Thread.sleep(500);
                } catch (InterruptedException e) { // 예외 = 스레드 내부에서 발생한 (sleep) 예외 처리
                    Thread.currentThread().interrupt(); // 인터럽트 상태(신호) 복구 (필수 패턴)
                    // “인터럽트가 왔던 사실”이 사라짐 -> “누군가 이 스레드 중단 요청했음을 상위/외부에도 알림”
                    return;
                }
            }
        }
    }

    // Thread = 실행 컨테이너
    // join()으로 종료 대기
    public static void run() throws InterruptedException { // 예외 = join에서 발생한 예외 처리
        Thread t1 = new Thread(new Worker(1));
        Thread t2 = new Thread(new Worker(2));

        t1.start();  // 실행
        t2.start();

        t1.join();   // 종료 대기
        t2.join();

        System.out.println("All done");
    }
}