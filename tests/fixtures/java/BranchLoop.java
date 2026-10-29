// SPDX-License-Identifier: GPL-3.0-only
// Original WP02 fixture: a conditional inside a counting loop.
public final class BranchLoop {
    public static int evaluate(int iterations) {
        int total = 0;
        int counter = 0;
        while (counter < iterations) {
            if (counter == 2) {
                total = total - 1;
            } else {
                total = total + counter;
            }
            counter++;
        }
        return total;
    }
}
