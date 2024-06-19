/*
** EPITECH PROJECT, 2024
** epi
** File description:
** Description du projet
*/

#include <stdio.h>
#include "my_print_alpha.h"

int my_print_alpha(void)
{
    for (int letter = 97; letter < 123; letter++) {
        printf("%c", letter);
    }
    return 0;
}
