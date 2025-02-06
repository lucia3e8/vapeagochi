/**
  ******************************************************************************
  * @file    GPIO/GPIO_IOToggle/Src/main.c
  * @author  MCD Application Team
  * @brief   This example describes how to configure and use GPIOs through
  *          the STM32F0xx HAL API.
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2016 STMicroelectronics.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by ST under BSD 3-Clause license,
  * the "License"; You may not use this file except in compliance with the
  * License. You may obtain a copy of the License at:
  *                        opensource.org/licenses/BSD-3-Clause
  *
  ******************************************************************************
  */

/* Includes ------------------------------------------------------------------*/
#include "main.h"

I2C_HandleTypeDef hi2c1;

/** @addtogroup STM32F0xx_HAL_Examples
  * @{
  */

/** @addtogroup GPIO_IOToggle
  * @{
  */

/* Private typedef -----------------------------------------------------------*/
/* Private define ------------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private variables ---------------------------------------------------------*/
static GPIO_InitTypeDef  GPIO_InitStruct;

/* Private function prototypes -----------------------------------------------*/
static void SystemClock_Config(void);
static void Error_Handler(void);

/* Private functions ---------------------------------------------------------*/

/**
  * @brief  Main program
  * @param  None
  * @retval None
  */
int main(void)
{
  __HAL_RCC_I2C1_CLK_ENABLE();  // In your initialization code
  /* This sample code shows how to use GPIO HAL API to toggle LED3, LED4, LED5 and LED6 IOs
    in an infinite loop. */

  /* STM32F0xx HAL library initialization:
       - Configure the Flash prefetch
       - Systick timer is configured by default as source of time base, but user
         can eventually implement his proper time base source (a general purpose
         timer for example or other time source), keeping in mind that Time base
         duration should be kept 1ms since PPP_TIMEOUT_VALUEs are defined and
         handled in milliseconds basis.
       - Low Level Initialization
     */
  HAL_Init();

  /* Configure the system clock to 48 MHz */
  SystemClock_Config();

  /* -1- Enable each GPIO Clock (to be able to program the configuration registers) */
  LED3_GPIO_CLK_ENABLE();
  LED4_GPIO_CLK_ENABLE();
  LED5_GPIO_CLK_ENABLE();
  LED6_GPIO_CLK_ENABLE();

  /* -2- Configure IOs in output push-pull mode to drive external LEDs */
  GPIO_InitStruct.Mode  = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull  = GPIO_PULLUP;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_HIGH;

  GPIO_InitStruct.Pin = LED3_PIN;
  HAL_GPIO_Init(LED3_GPIO_PORT, &GPIO_InitStruct);

  GPIO_InitStruct.Pin = LED4_PIN;
  HAL_GPIO_Init(LED4_GPIO_PORT, &GPIO_InitStruct);

  GPIO_InitStruct.Pin = LED5_PIN;
  HAL_GPIO_Init(LED5_GPIO_PORT, &GPIO_InitStruct);

  GPIO_InitStruct.Pin = LED6_PIN;
  HAL_GPIO_Init(LED6_GPIO_PORT, &GPIO_InitStruct);

  /* -3- Toggle IOs in an infinite loop */
  while (1)
  {
     // Reset OLED
    ssd1306_Reset();

    // Wait for the screen to boot
    // HAL_Delay(100);

    // Init OLED
    // ssd1306_SetDisplayOn(1);
    

    // ssd1306_WriteCommand(0x20); //Set Memory Addressing Mode
    // ssd1306_WriteCommand(0x00); // 00b,Horizontal Addressing Mode; 01b,Vertical Addressing Mode;
    //                             // 10b,Page Addressing Mode (RESET); 11b,Invalid

    // ssd1306_WriteCommand(0xB0); //Set Page Start Address for Page Addressing Mode,0-7

    // ssd1306_WriteCommand(0xC8); //Set COM Output Scan Direction
    
    // ssd1306_WriteCommand(0x00); //---set low column address
    // ssd1306_WriteCommand(0x10); //---set high column address

    // ssd1306_WriteCommand(0x40); //--set start line address - CHECK

    // ssd1306_SetContrast(0xFF);

    // ssd1306_WriteCommand(0xA1); //--set segment re-map 0 to 127 - CHECK
    // ssd1306_WriteCommand(0xA6); //--set normal color
    // ssd1306_WriteCommand(0xA8); //--set multiplex ratio(1 to 64) - CHECK
    // ssd1306_WriteCommand(0x3F); //

    // ssd1306_WriteCommand(0xA4); //0xa4,Output follows RAM content;0xa5,Output ignores RAM content

    // ssd1306_WriteCommand(0xD3); //-set display offset - CHECK
    // ssd1306_WriteCommand(0x00); //-not offset

    // ssd1306_WriteCommand(0xD5); //--set display clock divide ratio/oscillator frequency
    // ssd1306_WriteCommand(0xF0); //--set divide ratio

    // ssd1306_WriteCommand(0xD9); //--set pre-charge period
    // ssd1306_WriteCommand(0x22); //

    // ssd1306_WriteCommand(0xDA); //--set com pins hardware configuration - CHECK
    // ssd1306_WriteCommand(0x12);

    // ssd1306_WriteCommand(0xDB); //--set vcomh
    // ssd1306_WriteCommand(0x20); //0x20,0.77xVcc

    // ssd1306_WriteCommand(0x8D); //--set DC-DC enable
    // ssd1306_WriteCommand(0x14); //
    // ssd1306_SetDisplayOn(1); //--turn on SSD1306 panel

    // // Clear screen
    // memset(SSD1306_Buffer, 0x00, sizeof(SSD1306_Buffer));
    
    // // Flush buffer to screen
    // for(uint8_t i = 0; i < 8; i++) {
    //     ssd1306_WriteCommand(0xB0 + i); // Set the current RAM page address.
    //     ssd1306_WriteCommand(0x00);
    //     ssd1306_WriteCommand(0x10);
    //     ssd1306_WriteData(&SSD1306_Buffer[128*i],128);
    // }
    
    // // Set default values for screen object
    // SSD1306.CurrentX = 0;
    // SSD1306.CurrentY = 0;
    
    // SSD1306.Initialized = 1;

    
    // HAL_GPIO_TogglePin(LED3_GPIO_PORT, LED3_PIN);
    // /* Insert delay 100 ms */
    // HAL_Delay(1000);
    // HAL_GPIO_TogglePin(LED4_GPIO_PORT, LED4_PIN);
    // /* Insert delay 100 ms */
    // HAL_Delay(100);
    // HAL_GPIO_TogglePin(LED5_GPIO_PORT, LED5_PIN);
    // /* Insert delay 100 ms */
    // HAL_Delay(100);
    // HAL_GPIO_TogglePin(LED6_GPIO_PORT, LED6_PIN);
    // /* Insert delay 100 ms */
    // HAL_Delay(100);
  }
}

/**
  * @brief  System Clock Configuration
  *         The system Clock is configured as follow :
  *            System Clock source            = PLL (HSI48)
  *            SYSCLK(Hz)                     = 48000000
  *            HCLK(Hz)                       = 48000000
  *            AHB Prescaler                  = 1
  *            APB1 Prescaler                 = 1
  *            HSI Frequency(Hz)              = 48000000
  *            PREDIV                         = 2
  *            PLLMUL                         = 2
  *            Flash Latency(WS)              = 1
  * @param  None
  * @retval None
  */
static void SystemClock_Config(void)
{
  RCC_ClkInitTypeDef RCC_ClkInitStruct;
  RCC_OscInitTypeDef RCC_OscInitStruct;

  /* Select HSI48 Oscillator as PLL source */
  RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSI48;
  RCC_OscInitStruct.HSI48State = RCC_HSI48_ON;
  RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
  RCC_OscInitStruct.PLL.PLLSource = RCC_PLLSOURCE_HSI48;
  RCC_OscInitStruct.PLL.PREDIV = RCC_PREDIV_DIV2;
  RCC_OscInitStruct.PLL.PLLMUL = RCC_PLL_MUL2;
  if (HAL_RCC_OscConfig(&RCC_OscInitStruct)!= HAL_OK)
  {
    Error_Handler();
  }

  /* Select PLL as system clock source and configure the HCLK and PCLK1 clocks dividers */
  RCC_ClkInitStruct.ClockType = (RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_PCLK1);
  RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_PLLCLK;
  RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
  RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV1;
  if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_1)!= HAL_OK)
  {
    Error_Handler();
  }
}

/**
  * @brief  This function is executed in case of error occurrence.
  * @param  None
  * @retval None
  */
static void Error_Handler(void)
{
  /* User may add here some code to deal with this error */
  while(1)
  {
  }
}

#ifdef  USE_FULL_ASSERT

/**
  * @brief  Reports the name of the source file and the source line number
  *         where the assert_param error has occurred.
  * @param  file: pointer to the source file name
  * @param  line: assert_param error line source number
  * @retval None
  */
void assert_failed(uint8_t *file, uint32_t line)
{
  /* User can add his own implementation to report the file name and line number,
     ex: printf("Wrong parameters value: file %s on line %d\r\n", file, line) */

  /* Infinite loop */
  while (1)
  {
  }
}
#endif

/**
  * @}
  */

/**
  * @}
  */

/************************ (C) COPYRIGHT STMicroelectronics *****END OF FILE****/
