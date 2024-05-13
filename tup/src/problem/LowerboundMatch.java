package problem;

import model.Instance;

import static main.Config.NUM_UMPIRES;

public class LowerboundMatch {
    private final int[][] costArray;
    private final Instance instance;
    
    public LowerboundMatch(Instance instance) {
        this.instance = instance;
        this.costArray = new int[NUM_UMPIRES][NUM_UMPIRES];
    }


}
