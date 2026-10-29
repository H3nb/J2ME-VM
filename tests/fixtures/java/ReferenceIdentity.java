// SPDX-License-Identifier: GPL-3.0-only
// Original WP02-W9 diagnostic fixture: reference identity and null.
public final class ReferenceIdentity {
    public static int evaluate() {
        Object first = new Object();
        Object alias = echo(first);
        Object second = new Object();
        Object none = echo(null);

        if (first != alias) return 1;
        if (first == second) return 2;
        if (none == first) return 3;
        if (none == second) return 4;
        if (none != null) return 5;
        return 0;
    }

    private static Object echo(Object value) {
        return value;
    }
}
