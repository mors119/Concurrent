JAVAC = javac
JAVA = java

SRC_DIR = src
BUILD_DIR = out
MAIN_CLASS = app.Main

SRCS = $(shell find $(SRC_DIR) -name "*.java")

all:
	@mkdir -p $(BUILD_DIR)
	$(JAVAC) -d $(BUILD_DIR) $(SRCS)

run: all
	$(JAVA) -cp $(BUILD_DIR) $(MAIN_CLASS) $(ARGS)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all run clean