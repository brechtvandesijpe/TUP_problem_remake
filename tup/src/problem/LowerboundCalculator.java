package problem;

import model.Instance;

import static main.Config.DEBUG_LOWERBOUND_CALCULATOR;
import static main.Config.NUM_ROUNDS;

public class LowerboundCalculator {
    private final Instance instance;
    private final int[][] roundLBs;
    private final LowerboundMatch lowerboundMatch;

    public LowerboundCalculator(Tree tree) {
        this.instance = tree.getInstance();
        this.roundLBs = new int[NUM_ROUNDS][NUM_ROUNDS];
        this.lowerboundMatch = new LowerboundMatch(instance);
    }

    // Algorithm 2.2: Lower bounds computation algorithm
    public void calculateLBs() {
       // System.out.println("test");
        lowerboundMatch.generateCostArray(1);
        // Solving subproblems with size [2, R-1]
        for (int k = 1; k <= NUM_ROUNDS - 1; k++) {
            int r = NUM_ROUNDS - 1 - k;
            Tree tree = new Tree(instance, r, r + k, true);
            tree.startSubTraversal(this);
            int solutionValue = tree.getTotalDistance();
            for (int r1 = r; r1 >= 0; r1--) {
                for (int r2 = r + k; r2 <= NUM_ROUNDS - 1; r2++) {
                    roundLBs[r1][r2] = Math.max(roundLBs[r1][r2], roundLBs[r1][r] + solutionValue + roundLBs[r + k][r2]);
                    if (DEBUG_LOWERBOUND_CALCULATOR) {
                        System.out.println("Updated {" + r1 + "," + r2 + "} to: " + roundLBs[r1][r2]);
                    }
                }
            }
        }
    }

    public int getLBOfRounds(int round, int endRound) {
        return roundLBs[round][endRound];
    }

}
