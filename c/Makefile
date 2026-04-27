CC = gcc
CFLAGS = -Wall -Wextra -std=c17 -Iinclude

SRC_DIR = src
BUILD_DIR = build
TARGET = app

SRCS = $(shell find $(SRC_DIR) -name "*.c")
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(OBJS) -o $(TARGET)

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

run: all
	./$(TARGET) $(ARGS)

clean:
	rm -rf $(BUILD_DIR) $(TARGET)

.PHONY: all run clean