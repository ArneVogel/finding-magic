# finding-magic
Finding Magic-Squares trough bruteforce

# Usage
call  the program with a config file

./finding-magic example.txt

## example config
Theres a example config in the repo. The first line is the configuration of the search. In the second line the program saves its progress. This can progress can get very huge and some text editors might fail to load some times.

Example config:

3;1;1;false;magic;add;1,2

this config searches for a 3x3 magic square, starting from 1, with the minimum number 1, not using negative numbers, that is additive, and using the powers 1 and 2

Explanation of the values:
 * 1: the dimension of the square nxn
 * 2: the number to start from/that to what it currently has checked to
 * 3: the minimum number the square should have. Ignored if negatives are allowed
 * 4: If negative values are allowed in the square
 * 5: Values: semi or magic: specifies if searching for semi-magic or magic-squares
 * 6: The operation the square should satisfie. Values add, mul, gcd, lcm. Multiple values allowed.
 * 7: The powers the square should allow. Comma seperated. 1,2 means the numbers are first checked with power 1, then with power 2.
 
# Compiling
Clone the repo, make sure rust is installed. 

cargo build --release 
