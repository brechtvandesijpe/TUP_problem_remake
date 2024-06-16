package problem;

import main.Config;
import model.Instance;
import subproblem.match.MatchType;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.text.DecimalFormat;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.stream.IntStream;

import static main.Config.*;

public class LowerboundCalculator {
    private final Instance instance;
    public int[][] roundLBs;
    private final LowerboundMatch lowerboundMatch;
    private final Tree tree;

    public LowerboundCalculator(Tree tree) {
        this.tree = tree;
        this.instance = tree.getInstance();
        this.roundLBs = new int[NUM_ROUNDS][NUM_ROUNDS];
        this.lowerboundMatch = new LowerboundMatch();
    }

    /**
     * Algorithm 2.2: Lower bounds computation algorithm
     * PART 1: Calculate initial lowerbounds
     * PART 2: Strengthening lower bounds : increment subproblem size
     * PART 3: Propagation
     */

    public void calculateLBs() {
        // PART 1: Calculate initial lower bounds for all pairs of rounds using the values of the matchings between every two consecutive rounds
        if (MATCH_LOWERBOUND) {
            if (Config.LB_MATCH == LowerboundMatchType.MATCH_ALGORITHM) {
                //System.out.println("Chose MATCH_ALGORITHM");
                IntStream.range(0, NUM_ROUNDS - 1).forEach(roundIndex -> {
                    int newLowerBoundValue = lowerboundMatch.calculateRoundMatching(roundIndex);
                    int nextRound = roundIndex + 1;
                    IntStream.rangeClosed(0, roundIndex).forEach(i -> IntStream.rangeClosed(nextRound, NUM_ROUNDS - 1).forEach(j -> roundLBs[i][j] = Math.max(roundLBs[i][j], roundLBs[i][roundIndex] + newLowerBoundValue + roundLBs[roundIndex][j])));
                });
            } else if (Config.LB_MATCH == LowerboundMatchType.BRANCH_AND_BOUND_2_DEEP) {
                for (int roundIndex = 0; roundIndex < NUM_ROUNDS - 1; roundIndex++) {
                    Tree tree = new Tree(instance, roundIndex, roundIndex + 1, true);
                    tree.startSubTraversal(this);
                    int newLowerBoundValue = tree.getTotalDistance();
                    int nextRound = roundIndex + 1;
                    for (int i = 0; i <= roundIndex; i++) {
                        for (int j = nextRound; j < NUM_ROUNDS; j++) {
                            roundLBs[i][j] = Math.max(roundLBs[i][j], roundLBs[i][roundIndex] + newLowerBoundValue + roundLBs[roundIndex][j]);
                        }
                    }
                }
            }else {

            }
        }

        // PART 2: Solving subproblems with size [2, R-1]
        for (int k = 1; k <= NUM_ROUNDS - 1; k++) {
            int r = NUM_ROUNDS - 1 - k;
            Tree tree = new Tree(instance, r, r + k, true);
            tree.startSubTraversal(this);
            int solutionValue = tree.getTotalDistance();
            for (int r1 = r; r1 >= 0; r1--) {
                for (int r2 = r + k; r2 <= NUM_ROUNDS - 1; r2++) {
                    printDebugInfo();
                    // PART 3: Lowerbound propagation
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

    public void timeAndLogLBMatchAlgorithms() {
        String csvFilePath = "LBMatchDurations.csv";
        int measurements = 100;
        
        boolean fileExists = new File(csvFilePath).exists();
        
        StringBuilder dataBuilder = new StringBuilder();
        if (!fileExists) {
            String header = "file,HUNGARIAN,JONKERVOLGENANT,BRANCH_AND_BOUND_2_DEEP\n";
            dataBuilder.append(header);
        }
        
        double hungarianTotalDuration = 0;
        double jonkervolgenantTotalDuration = 0;
        double branchAndBound2DeepTotalDuration = 0;
        
        for (int i = 0; i < measurements; i++) {
            MATCH_TYPE = MatchType.HUNGARIAN;
            hungarianTotalDuration += timeLBMatchAlgorithm(LowerboundMatchType.MATCH_ALGORITHM);
            
            MATCH_TYPE = MatchType.JONKER_VOLGENANT;
            jonkervolgenantTotalDuration += timeLBMatchAlgorithm(LowerboundMatchType.MATCH_ALGORITHM);
            
            branchAndBound2DeepTotalDuration += timeLBMatchAlgorithm(LowerboundMatchType.BRANCH_AND_BOUND_2_DEEP);
        }
        
        double hungarianMeanDuration = hungarianTotalDuration / measurements;
        double jonkervolgenantMeanDuration = jonkervolgenantTotalDuration / measurements;
        double branchAndBound2DeepMeanDuration = branchAndBound2DeepTotalDuration / measurements;
        
        dataBuilder.append(Config.FILE_NAME).append("_").append(Q1).append("_").append(Q2).append(",")
                    .append(hungarianMeanDuration).append(",")
                    .append(jonkervolgenantMeanDuration).append(",")
                    .append(branchAndBound2DeepMeanDuration).append("\n");
        
        try (FileWriter fileWriter = new FileWriter(csvFilePath, true)) { // Enable appending
            fileWriter.write(dataBuilder.toString());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private double timeLBMatchAlgorithm(LowerboundMatchType algorithmType) {
        clearLBs();
        Config.LB_MATCH = algorithmType;
        long startTime = System.nanoTime();
        if (MATCH_LOWERBOUND) {
            if (Config.LB_MATCH == LowerboundMatchType.MATCH_ALGORITHM) {
                //System.out.println("Chose MATCH_ALGORITHM");
                IntStream.range(0, NUM_ROUNDS - 1).forEach(roundIndex -> {
                    int newLowerBoundValue = lowerboundMatch.calculateRoundMatching(roundIndex);
                    int nextRound = roundIndex + 1;
                    IntStream.rangeClosed(0, roundIndex).forEach(i -> IntStream.rangeClosed(nextRound, NUM_ROUNDS - 1).forEach(j -> roundLBs[i][j] = Math.max(roundLBs[i][j], roundLBs[i][roundIndex] + newLowerBoundValue + roundLBs[roundIndex][j])));
                });
            } else if (Config.LB_MATCH == LowerboundMatchType.BRANCH_AND_BOUND_2_DEEP) {
                for (int roundIndex = 0; roundIndex < NUM_ROUNDS - 1; roundIndex++) {
                    Tree tree = new Tree(instance, roundIndex, roundIndex + 1, true);
                    tree.startSubTraversal(this);
                    int newLowerBoundValue = tree.getTotalDistance();
                    int nextRound = roundIndex + 1;
                    for (int i = 0; i <= roundIndex; i++) {
                        for (int j = nextRound; j < NUM_ROUNDS; j++) {
                            roundLBs[i][j] = Math.max(roundLBs[i][j], roundLBs[i][roundIndex] + newLowerBoundValue + roundLBs[roundIndex][j]);
                        }
                    }
                }
            }else {
                // todo
            }
        }
        long endTime = System.nanoTime();
        return (endTime - startTime) / 1_000_000_000.0;
    }

    public void clearLBs() {
        roundLBs = new int[NUM_ROUNDS][NUM_ROUNDS];
    }
}
