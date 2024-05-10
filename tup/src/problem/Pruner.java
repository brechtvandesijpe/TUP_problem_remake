package problem;

import main.Config;

import java.util.HashSet;

public class Pruner {
    private final Tree tree;
    private HashSet<Integer> prunedGames;
    private int startRoundForQ1Constraint;
    private int startRoundForQ2Constraint;

    public Pruner(Tree tree) {
        this.tree = tree;
    }

    public void initPruner(int currentRoundIndex) {
        this.startRoundForQ1Constraint = Math.max(currentRoundIndex + 1 - Config.Q1, tree.getStartRoundIndex());
        this.startRoundForQ2Constraint = Math.max(currentRoundIndex + 1 - Config.Q2, tree.getStartRoundIndex());
        this.prunedGames = new HashSet<>();
    }

    public HashSet<Integer> pruneGames(int umpire, int currentRoundIndex) {
        initPruner(currentRoundIndex);
        return prunedGames;
    }

}
