package problem;

import model.Instance;

public class Tree {
    private final Instance instance;
    private final int startRoundIndex;
    private final int endRoundIndex;
    private final boolean isSub;

    public Tree(Instance instance, int startRoundIndex, int endRoundIndex, boolean isSub) {
        this.isSub = isSub;
        this.instance = instance;
        this.startRoundIndex = startRoundIndex;
        this.endRoundIndex = endRoundIndex;
    }

    public void startGlobalTraversal() {
        System.out.println("e");
    }

}
