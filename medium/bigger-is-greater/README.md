1. Find the largest index i such that the character at position i is smaller than the character at position i + 1.
2. If no such index is found, the word is already the greatest possible, and we return “no answer”.
3. Find the smallest index j > i such that the character at position j is greater than the character at position i.
4. Swap characters at positions i and j.
5. Reverse the subarray of characters starting from position i + 1 to the end of the array.