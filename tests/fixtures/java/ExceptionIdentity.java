// SPDX-License-Identifier: GPL-3.0-only
// Original WP02-W9 diagnostic fixture: local catch and caller catch after throw.
public final class ExceptionIdentity {
    public static int evaluate() {
        Throwable expected = new Throwable();

        try {
            throw expected;
        } catch (Throwable caught) {
            if (caught != expected) return 1;
        }

        try {
            rethrow(expected);
            return 2;
        } catch (Throwable caught) {
            if (caught != expected) return 3;
        }

        return 0;
    }

    private static void rethrow(Throwable value) throws Throwable {
        throw value;
    }
}
