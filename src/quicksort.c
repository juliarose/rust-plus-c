#include <stdio.h>
#include "quicksort.h"

void quicksort(int *numbers, int first, int last) {
    int i, j, pivot, temp;
    
    if (first < last) {
        pivot = first;
        i = first;
        j = last;
        
        while (i < j) {
            while (numbers[i] <= numbers[pivot] && i < last) {
                i++;
            }
            
            while (numbers[j] > numbers[pivot]) {
                j--;
            }
            
            if (i < j) {
                temp = numbers[i];
                numbers[i] = numbers[j];
                numbers[j] = temp;
            }
        }
        
        temp = numbers[pivot];
        numbers[pivot]=numbers[j];
        numbers[j] = temp;
        quicksort(numbers, first, j - 1);
        quicksort(numbers, j + 1, last);
    }
}