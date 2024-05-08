package problem;

import java.util.HashSet;

public class Pruner {
    private final Tree tree;
    private HashSet<Integer> prunedGames;

    public Pruner(Tree tree) {
        this.tree = tree;
    }

    public void initPruner(int currentRoundIndex) {
        this.prunedGames = new HashSet<>();
    }

    public HashSet<Integer> pruneGames(int umpire, int currentRoundIndex) {
        initPruner(currentRoundIndex);

        return prunedGames;
    }


}
