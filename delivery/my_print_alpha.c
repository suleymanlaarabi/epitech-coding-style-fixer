/*
** EPITECH PROJECT, 2024
** epi
** File description:
** Description du projet
*/

#include "my_print_alpha.h"
#include <stdio.h>

int my_print_alpha(void)
{
    for (int letter = 97; letter < 123; letter++) {
        printf("%c", letter);
    }
    return 0;
}
