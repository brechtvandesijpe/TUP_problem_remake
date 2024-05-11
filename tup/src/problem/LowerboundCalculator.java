package problem;

import model.Instance;

import static main.Config.NUM_ROUNDS;

public class LowerboundCalculator {
    private final Instance instance;
    private final Tree tree;
    private final int[][] roundLBs;

    public LowerboundCalculator(Tree tree) {
        this.tree = tree;
        this.instance = tree.getInstance();
        this.roundLBs = new int[NUM_ROUNDS][NUM_ROUNDS];
    }

    // Algorithm 2.2: Lower bounds computation algorithm
    public void calculateLBs() {

    }

}
