SHELL = /bin/bash
C_FLAGS := -g -Wall -Wextra -Wno-unused-parameter
TST_FLAGS := -D TEST_MODE
CC := gcc
RM := rm
LINKFLAGS :=
TARGET := weezie
TST_TARGET := weezie_tst
INC_PATH := -I ./include

SRC_DIR := ./src
OBJ_DIR := ./obj
TST_DIR := ./tst
MOD_DIR := ./mod

TST_FILES := $(wildcard $(TST_DIR)/*.c)
SRC_FILES := $(wildcard $(SRC_DIR)/*.c)
MOD_FILES := $(wildcard $(MOD_DIR)/*.c)

MOD_OBJ_FILES := $(patsubst $(MOD_DIR)/%.c,$(OBJ_DIR)/%.o,$(MOD_FILES))
OBJ_FILES := $(patsubst $(SRC_DIR)/%.c,$(OBJ_DIR)/%.o,$(SRC_FILES))
TST_OBJ_FILES := $(patsubst $(TST_DIR)/%.c,$(OBJ_DIR)/%.o,$(TST_FILES)) $(patsubst $(SRC_DIR)/%.c,$(OBJ_DIR)/%_tst.o,$(SRC_FILES))

default: $(TARGET)
	./$(TARGET)

test: $(TST_TARGET)
	valgrind --leak-check=full -s ./$(TST_TARGET)

$(TARGET): $(OBJ_FILES) $(MOD_OBJ_FILES)
	@echo [INFO] Creating Binary Executable [$(TARGET)]
	@$(CC) -o $@ $^

$(TST_TARGET): $(TST_OBJ_FILES)
	@echo [INFO] Creating TEST Executable [$(TST_TARGET)]
	@$(CC) -o $@ $^
	@chmod +x $(TST_TARGET)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	@echo [CC] $<
	@$(CC) $(C_FLAGS)  -o $@ -c $< $(INC_PATH)

$(OBJ_DIR)/%.o: $(TST_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	@echo [CC] $<
	@$(CC) $(C_FLAGS) -o $@ -c $< $(INC_PATH)

$(OBJ_DIR)/%.o: $(MOD_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	@echo [CC] $<
	@$(CC) $(C_FLAGS) -o $@ -c $< $(INC_PATH)

$(OBJ_DIR)/%_tst.o: $(SRC_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	@echo [CC] $<
	@$(CC) $(C_FLAGS) $(TST_FLAGS) -o $@ -c $< $(INC_PATH)



clean:
	@echo "[Cleaning]"
	$(RM) -rfv $(OBJ_DIR)/*
	$(RM) -rfv $(TARGET)
	$(RM) -rfv $(TST_TARGET)

FORCE:
