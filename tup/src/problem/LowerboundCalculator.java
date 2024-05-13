package problem;

import main.Config;
import model.Instance;
import java.text.DecimalFormat;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.stream.IntStream;

import static main.Config.*;

public class LowerboundCalculator {
    private final Instance instance;
    private final int[][] roundLBs;
    private final LowerboundMatch lowerboundMatch;
    private final Tree tree;

    public LowerboundCalculator(Tree tree) {
        this.tree = tree;
        this.instance = tree.getInstance();
        this.roundLBs = new int[NUM_ROUNDS][NUM_ROUNDS];
        this.lowerboundMatch = new LowerboundMatch();
    }

    // Algorithm 2.2: Lower bounds computation algorithm
    public void calculateLBs() {
        // System.out.println("test");
        // Calculate initial lower bounds for all pairs of rounds using the values of the matchings between every two consecutive rounds
        if (MATCH_LOWERBOUND) {
            IntStream.range(0, NUM_ROUNDS - 1).forEach(roundIndex -> {
                int newLowerBoundValue = lowerboundMatch.calculateRoundMatching(roundIndex);
                int nextRound = roundIndex + 1;
                IntStream.rangeClosed(0, roundIndex).forEach(i -> IntStream.rangeClosed(nextRound, NUM_ROUNDS - 1).forEach(j -> roundLBs[i][j] = Math.max(roundLBs[i][j], roundLBs[i][roundIndex] + newLowerBoundValue + roundLBs[roundIndex][j])));
            });
        }

        lowerboundMatch.generateCostArray(1);
        // Solving subproblems with size [2, R-1]
        for (int k = 1; k <= NUM_ROUNDS - 1; k++) {
            int r = NUM_ROUNDS - 1 - k;
            Tree tree = new Tree(instance, r, r + k, true);
            tree.startSubTraversal(this);
            int solutionValue = tree.getTotalDistance();
            for (int r1 = r; r1 >= 0; r1--) {
                for (int r2 = r + k; r2 <= NUM_ROUNDS - 1; r2++) {
                    printDebugInfo();
                    roundLBs[r1][r2] = Math.max(roundLBs[r1][r2], roundLBs[r1][r] + solutionValue + roundLBs[r + k][r2]);
                    if (DEBUG_LOWERBOUND_CALCULATOR) {
                        System.out.println("Updated {" + r1 + "," + r2 + "} to: " + roundLBs[r1][r2]);
                    }
                }
            }
        }
    }

    public void printDebugInfo() {
        if (PRINT_GAP) {
            DecimalFormat df = new DecimalFormat("0.00%");
            double gapPercentage = (double) (tree.getUpperbound() - roundLBs[0][NUM_ROUNDS - 1]) / tree.getUpperbound();
            SimpleDateFormat dateFormat = new SimpleDateFormat("dd-MM-yyyy HH:mm:ss");
            String currentTimeStamp = dateFormat.format(new Date());
            System.out.println(lightGrey + "[" + currentTimeStamp + "]" + reset + " GAP: " + df.format(gapPercentage) + ", LB: " + roundLBs[0][NUM_ROUNDS - 1] + ", UB: " + tree.getUpperbound() + yellow + " [LB ↑]" + Config.reset);
        }
    }

    public int getLBOfRounds(int round, int endRound) {
        return roundLBs[round][endRound];
    }

    public int[][] getRoundLBs() {
        return roundLBs;
    }
}
