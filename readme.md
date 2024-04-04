This project is insipired by this paper: [Best Increments for the Average Case of Shellsort](https://web.archive.org/web/20180923235211/http://sun.aei.polsl.pl/~mciura/publikacje/shellsort.pdf) by Marcin Ciura. It's not yet complete, but it should be able to come up with gap sequences similar the one created by Ciura. I'm looking specifically to find the best gap sequence for an entirely random array of a given length, since it's obvious from this paper that there is no universally best option.

In addition to Shellsort, I also analyze quicksort for comparison. Shellsort is a fascinating algorithm. With many advantages over quicksort. It guarantees O(1) auxiliary space complexity by not using recursion unlike quicksort's O(log n) average and O(n) worst case. It's also adaptive, performing well with partially sorted arrays, which are common in practice. The basis of Shellsort is insertion sort, which runs in O(n^2) time on average and O(1) space. However, insertion sort becomes an O(k \* n) algorithm when the array is already partially sorted, where k is the largest distance any element is from where it should be. Shellsort works to create such an array so that insertion sort can run where it shines.

Already, the output of this program running with these arguments:

```bash
cargo run --release -- -l 1000000 -q
```

is

> Sorting results on array of length 1000000 for 100 round(s).  
> Shell sort performed with gap sequence: [1, 4, 9, 20, 45, 102, 230, 516, 1158, 2599, 5831, 13082, 29351, 65853, 147748, 331490, 743735]
>
> Shellsort Report:  
> Average comparisons: 31954418.22  
> Std Dev comparisons: 18439.27  
> Most comparisons: 32005651  
> Fewest comparisons: 31896832  
> Average moves: 32429146.81  
> Std Dev moves: 18605.96  
> Most moves: 32480327  
> Fewest moves: 32371866
>
> Quicksort Report:  
> Average Comparisons: 25356610.00  
> Std Dev Comparisons: 681667.07  
> Most Comparisons: 28422334  
> Fewest Comparisons: 24341364  
> Average Swaps: 24689906.01  
> Std Dev Swaps: 681687.29  
> Most Swaps: 27755948  
> Fewest Swaps: 23674755  
> Average Max Depth: 49.93  
> Std Dev Max Depth: 2.29  
> Highest Max Depth: 57  
> Lowest Max Depth: 46

I think this shows promise. I'm excited to see if I can produce interesting results when I complete this project.
