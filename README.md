# Concurrent Programming (C / Java / Rust)

이 프로젝트는 C, Java, Rust를 통해 동시성과 병행제어를 학습하기 위해 만들었습니다.
Concurrent Programming 도서를 기본으로 동일 내용을 좀 더 심도있게 학습하고자 합니다.

각 언어별 실행 방식을 `Makefile`을 통해 통합하여 다음과 같은 형태로 사용할 수 있습니다.

###### **common/chapter1 실행**

```bash
make run-java ARGS="common 1"
make run-c ARGS="common 1"
make run-rust ARGS="common 1"
```
