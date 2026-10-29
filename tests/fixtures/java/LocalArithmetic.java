// SPDX-License-Identifier: GPL-3.0-only
// Original WP02 fixture: argument and int local-variable arithmetic.
public final class LocalArithmetic {
    public static int evaluate(int input) {
        int expanded = input + 7;
        int adjusted = expanded - 3;
        return adjusted + (input - 2);
    }
}
