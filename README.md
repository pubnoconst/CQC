## Quote Calculator (CLI)

This is a fast and minimal command-line tool written in F# to calculate a repair quote based on listed part costs.

### Features

* User-friendly interface with prompts for part costs.
* Supports simple math expressions for dynamic calculations.
* Displays a table with individual part costs and the final quote.
* Calculates the total quote and minimum deposit (50% of total).

**Note:** This is an in-house tool and is not affiliated with Casphone company, or the author. It is provided "as is" without warranty of any kind.

### Requirements

* .NET runtime 6.0 or later

### Usage

1. Run the application from the command line.
2. Enter the cost of each part on a separate line. You can use simple math expressions like "10 + 7.5".
3. Press Enter twice or enter a non-numerical line to finalize the quote calculation.

### Example

```
Welcome to Quote calculator. Simply list the parts cost in each line.
Pressing Enter twice will or entering a non-numerical line prompt me to compute the quote.
Part 1 cost: 42.1
Part 2 cost: 10.4 + 0.15
Part 3 cost: 
END

 #     │ Part cost │ Quote
═══════╪═══════════╪════════
 1     │ 42.10     │ 182.10
 2     │ 10.55     │ 150.55
 Total │           │ 332.65

Minimum deposit: 166.33
```

### License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0). See the LICENSE file for details.

### Copyright

Copyright (C) pubnoconst 2024.  All rights reserved.
