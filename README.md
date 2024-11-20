## Quote Calculator (CLI)

This is a fast and minimal command-line tool written in F# to calculate a repair quote based on listed part costs.

### Features

* User-friendly interface with prompts for part costs.
* Supports simple math expressions for dynamic calculations.
* Displays a table with individual part costs and the final quote.
* Calculates the total quote and minimum deposit (50% of total).

**Note:** This is an in-house tool and is not affiliated with Casphone company, or the author. It is provided "as is" without warranty of any kind.

### Usage

1. Run the application from the command line.
2. Enter the cost of each part on a separate line. You can use simple math expressions like "10 + 7.5".
3. Press Enter twice or enter a non-numerical line to finalize the quote calculation.

### Example

```
Welcome to quote calculator.
Simply enter the cost of the parts in each lines.
Entering a non-neumerical value or pressing Enter twice will prompt me to calculate the total cost.
Part    1: $17.36
Part    2: $7.87
Part    3: $15.11 + 34.98
Part    4: $
╭-------+-----------+--------╮
| #     | Part cost | Quote  |
+============================+
| 1     | 17.36     | 157.36 |
|-------+-----------+--------|
| 2     | 7.87      | 147.87 |
|-------+-----------+--------|
| 3     | 50.09     | 190.09 |
|-------+-----------+--------|
| Total |           | 495.32 |
╰-------+-----------+--------╯
Minimum deposit: $247.66
```

### License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0). See the LICENSE file for details.

### Copyright

Copyright (C) pubnoconst 2024.  All rights reserved.
