// SPDX-License-Identifier: GPL-3.0-only
// Original WP02-W6 fixture: bounded-test recursive static invocation.
public final class RecursiveStaticCalls {
    public static int evaluate(int depth) {
        int base = 0;
        if (depth <= base) {
            return 0;
        }
        return evaluate(depth - 1) + 1;
    }
}
