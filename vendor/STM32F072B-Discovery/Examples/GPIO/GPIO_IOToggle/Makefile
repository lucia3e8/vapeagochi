# Toolchain definitions
CC = arm-none-eabi-gcc
AS = arm-none-eabi-as
OBJCOPY = arm-none-eabi-objcopy
SIZE = arm-none-eabi-size

# Project name
TARGET = firmware

# Paths
BUILD_DIR = build
SRC_DIR = Src
INC_DIR = Inc
STARTUP_DIR = SW4STM32
CUBE_DIR = /home/bmc/Downloads/STM32Cube_FW_F0_V1.11.0
HAL_SRC_DIR = $(CUBE_DIR)/Drivers/STM32F0xx_HAL_Driver/Src
LINKER_SCRIPT = $(STARTUP_DIR)/STM32F072B-Discovery/STM32F072RBTx_FLASH.ld

# MCU flags
CPU = -mcpu=cortex-m0
MCU = $(CPU) -mthumb

# Compiler flags
CFLAGS = $(MCU)
CFLAGS += -Wall -Wextra
CFLAGS += -O2
CFLAGS += -DSTM32F072xB
CFLAGS += -DUSE_HAL_DRIVER
CFLAGS += -I$(INC_DIR)
CFLAGS += -I$(CUBE_DIR)/Drivers/STM32F0xx_HAL_Driver/Inc
CFLAGS += -I$(CUBE_DIR)/Drivers/CMSIS/Device/ST/STM32F0xx/Include
CFLAGS += -I$(CUBE_DIR)/Drivers/CMSIS/Include
CFLAGS += -I$(CUBE_DIR)/Drivers/BSP/STM32F072B-Discovery

# Linker flags
LDFLAGS = $(MCU)
LDFLAGS += -T$(LINKER_SCRIPT)
LDFLAGS += --specs=nano.specs
LDFLAGS += -Wl,--gc-sections

# Source files
SRC = $(wildcard $(SRC_DIR)/*.c)
ASM = $(STARTUP_DIR)/startup_stm32f072xb.s

# HAL source files
HAL_SRC = $(HAL_SRC_DIR)/stm32f0xx_hal.c
HAL_SRC += $(HAL_SRC_DIR)/stm32f0xx_hal_cortex.c
HAL_SRC += $(HAL_SRC_DIR)/stm32f0xx_hal_gpio.c
HAL_SRC += $(HAL_SRC_DIR)/stm32f0xx_hal_rcc.c
HAL_SRC += $(HAL_SRC_DIR)/stm32f0xx_hal_rcc_ex.c

# Object files
OBJ = $(SRC:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)
OBJ += $(HAL_SRC:$(HAL_SRC_DIR)/%.c=$(BUILD_DIR)/%.o)
OBJ += $(BUILD_DIR)/startup_stm32f072xb.o

# Build rules
all: $(BUILD_DIR)/$(TARGET).elf $(BUILD_DIR)/$(TARGET).hex $(BUILD_DIR)/$(TARGET).bin

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(BUILD_DIR)
	$(CC) -c $(CFLAGS) $< -o $@

$(BUILD_DIR)/%.o: $(HAL_SRC_DIR)/%.c
	@mkdir -p $(BUILD_DIR)
	$(CC) -c $(CFLAGS) $< -o $@

$(BUILD_DIR)/startup_stm32f072xb.o: $(ASM)
	@mkdir -p $(BUILD_DIR)
	$(AS) $(MCU) $< -o $@

$(BUILD_DIR)/$(TARGET).elf: $(OBJ)
	$(CC) $(OBJ) $(LDFLAGS) -o $@
	$(SIZE) $@

$(BUILD_DIR)/$(TARGET).hex: $(BUILD_DIR)/$(TARGET).elf
	$(OBJCOPY) -O ihex $< $@

$(BUILD_DIR)/$(TARGET).bin: $(BUILD_DIR)/$(TARGET).elf
	$(OBJCOPY) -O binary $< $@

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean
