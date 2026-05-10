use std::sync::{Arc, Mutex};
use std::thread;

// Banker’s Algorithm (은행원 알고리즘)
// - 자원을 요청할 때 "이 요청을 허용해도 전체 시스템이 안전하게 종료 가능한가?"
//   를 검사한 뒤, 안전할 때만 자원을 할당한다.

// Dining Philosophers Problem 해결 예제
// - philosopher0, philosopher1이 포크(자원) 2개를 서로 다른 순서로 요청
// - Banker가 unsafe state(데드락 가능 상태)를 차단하여 데드락을 방지

/*
구조:

    philosopher0 ─┐
                  ├── Banker ── Mutex ── Resource
    philosopher1 ─┘

Banker
- 여러 스레드가 공유하는 자원 관리자

Mutex
- Banker의 내부 장부(Resource)를 보호

Resource
- available / allocation / max 상태 관리
*/

// NRES: 자원 종류 수
// NTH : 스레드(프로세스) 수
struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],         // 현재 사용 가능한 자원 수
    allocation: [[usize; NRES]; NTH], // 각 스레드가 현재 보유한 자원 수
    max: [[usize; NRES]; NTH],        // 각 스레드가 최대로 필요로 하는 자원 수
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

    // 현재 상태가 safe state인지 검사

    // safe state:
    // - 어떤 순서로든 모든 스레드가 필요한 자원을 얻고
    //   작업을 완료할 수 있는 상태
    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH];
        let mut work = self.available.clone();

        loop {
            let mut found = false;
            let mut num_true = 0;

            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                // need = max - allocation
                // 앞으로 추가로 필요한 자원 수
                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);

                // 현재 work(가용 자원)만으로 완료 가능한가?
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);

                if is_avail {
                    // 이 스레드는 완료 가능하다고 가정
                    finish[i] = true;
                    found = true;

                    // 작업이 끝나면 현재 보유 자원을 반환한다고 가정
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a;
                    }

                    break;
                }
            }

            // 모든 스레드가 완료 가능하면 safe state
            if num_true == NTH {
                return true;
            }

            // 더 이상 완료 가능한 스레드가 없으면 unsafe state
            if !found {
                break;
            }
        }

        false
    }

    // id번 스레드가 resource번 자원 1개 요청
    fn take(&mut self, id: usize, resource: usize) -> bool {
        // 잘못된 요청이거나 자원이 없으면 실패
        if id >= NTH || resource >= NRES || self.available[resource] == 0 {
            return false;
        }

        // 일단 자원을 할당해 본다
        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        // safe state이면 확정
        if self.is_safe() {
            true
        } else {
            // unsafe state이면 원복
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    // id번 스레드가 resource번 자원 1개 반환
    fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    // Arc
    // - 여러 스레드가 같은 Banker 상태를 공유

    // Mutex
    // - Resource 장부를 한 번에 하나의 스레드만 수정하도록 보호
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Banker {
            resource: Arc::new(Mutex::new(Resource::new(available, max))),
        }
    }

    // 자원 요청
    fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    // 자원 반환
    fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource)
    }
}

const NUM_LOOP: usize = 100000;

pub fn run() {
    // Banker::<2, 2>
    // - 자원 종류 2개 (포크 2개)
    // - 스레드 2개 (철학자 2명)

    // available = [1, 1]
    // - 포크 0은 1개
    // - 포크 1은 1개

    // max = [[1, 1], [1, 1]]
    // - 각 철학자는 포크 0과 포크 1을 각각 1개씩 필요
    let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
    let banker0 = banker.clone();

    // philosopher0:
    // - 포크 0 → 포크 1 순서로 요청
    let philosopher0 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker0.take(0, 0) {
                // busy waiting 완화
                thread::yield_now();
            }

            while !banker0.take(0, 1) {
                thread::yield_now();
            }

            println!("0: eating");

            banker0.release(0, 0);
            banker0.release(0, 1);
        }
    });

    // philosopher1:
    // - 포크 1 → 포크 0 순서로 요청

    // 일반 Mutex만 쓰면 데드락 가능
    // Banker가 unsafe state를 차단하여 데드락 방지
    let philosopher1 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker.take(1, 1) {
                thread::yield_now();
            }

            while !banker.take(1, 0) {
                thread::yield_now();
            }

            println!("1: eating");

            banker.release(1, 1);
            banker.release(1, 0);
        }
    });

    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
}
