Intuition: if the prices are sorted, the minimum loss is given by two adjacent prices. We just need to check if the pair is “valid”, that is: if the sell price is actually following the purchase (the sell is in the future).

Thus, we can duplicate our data and add an index for each price. The index is ascending and it just represents then history.

For example, for: 14 7 8 2 5

We have:
(0, 14) (1, 7) (2, 8) (3, 2) (4, 5)

Now we can sort the data and the history won’t be lost:

(3, 2) (4, 5) (1, 7) (2, 8) (0, 14)

We can finally process adjacent pairs. Since we sell at a loss:

- the right pair (price is bigger) represents the purchase price
- the left pair (price is lower) represents the sell price
