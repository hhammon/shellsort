This project is insipired by this paper: [Best Increments for the Average Case of Shellsort](https://web.archive.org/web/20180923235211/http://sun.aei.polsl.pl/~mciura/publikacje/shellsort.pdf) by Marcin Ciura. It's not yet complete, but it should be able to come up with gap sequences similar the one created by Ciura. I'm looking specifically to find the best gap sequence for an entirely random array of a given length, since it's obvious from this paper that there is no universally best option.

In addition to Shellsort, I also analyze quicksort for comparison. Shellsort is a fascinating algorithm. With many advantages over quicksort. It guarantees O(1) auxiliary space complexity by not using recursion unlike quicksort's O(log n) average and O(n) worst case. It's also adaptive, performing well with partially sorted arrays, which are common in practice. The basis of Shellsort is insertion sort, which runs in O(n^2) time on average and O(1) space. However, insertion sort becomes an O(k \* n) algorithm when the array is already partially sorted, where k is the largest distance any element is from where it should be. Shellsort works to create such an array so that insertion sort can run where it shines.

Already, the output of this program running with these arguments:

```bash
cargo run --release -- -q -c 4,10,23,57,132,301,701,1750 -l 5000
```

is

> Sorting results on array of length 5000 for 100 round(s).  
> Shell sort performed with gap sequence: [1, 4, 10, 23, 57, 132, 301, 701, 1750]
>
> Shellsort Report:  
> Average comparisons: 86035.78  
> Std Dev comparisons: 470.05  
> Most comparisons: 87248  
> Fewest comparisons: 85054  
> Average moves: 88326.13  
> Std Dev moves: 473.84  
> Most moves: 89565  
> Fewest moves: 87306
>
> Quicksort Report:  
> Average Comparisons: 70483.17  
> Std Dev Comparisons: 3105.10  
> Most Comparisons: 81103  
> Fewest Comparisons: 65183  
> Average Swaps: 38906.33  
> Std Dev Swaps: 3111.31  
> Most Swaps: 47840  
> Fewest Swaps: 32896  
> Average Max Depth: 28.44  
> Std Dev Max Depth: 2.01  
> Highest Max Depth: 34  
> Lowest Max Depth: 24

I think this demonstrates promise. I'm excited to see if I can produce interesting results when I complete this project.
