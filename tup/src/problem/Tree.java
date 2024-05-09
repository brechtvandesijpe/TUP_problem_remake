package problem;

import model.Instance;

import java.util.Arrays;
import java.util.HashSet;

public class Tree {
    private final Instance instance;
    private final Pruner pruner;
    private final int startRoundIndex;
    private final int endRoundIndex;
    private final boolean isSub;

    private HashSet<Integer> prunedGames;

    public Tree(Instance instance, int startRoundIndex, int endRoundIndex, boolean isSub) {
        this.isSub = isSub;
        this.instance = instance;
        this.startRoundIndex = startRoundIndex;
        this.endRoundIndex = endRoundIndex;

        this.pruner = new Pruner(this);
    }

    public void startGlobalTraversal() {
        System.out.println(Arrays.deepToString(Instance.roundStadium));
    }

    public int[] getFeasibleAllocations(int umpire, int currentRoundIndex) {
        prunedGames = pruner.pruneGames(umpire, currentRoundIndex);
        return new int[0];
    }
}
