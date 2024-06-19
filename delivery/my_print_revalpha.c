/*
** EPITECH PROJECT, 2024
** epi
** File description:
** Description du projet
*/

#include "my_print_revalpha.h"
#include <stdio.h>

int my_print_revalpha(void)
{
    for (int letter = 122; letter > 96; letter--) {
        printf("%c", letter);
    }
    return 0;
}
