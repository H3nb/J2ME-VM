// SPDX-License-Identifier: GPL-3.0-only
// Original WP02-W6 fixture: nested same-class static calls and overloads.
public final class NestedStaticCalls {
    public static int evaluate(int input) {
        int first = adjust(input);
        int second = adjust(input + 1);
        return first + second - adjust();
    }

    private static int adjust(int value) {
        return increment(value) + 2;
    }

    private static int increment(int value) {
        return value + 3;
    }

    private static int adjust() {
        return 4;
    }
}
