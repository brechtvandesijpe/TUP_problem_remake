package localsearch;

import main.Config;
import metaheuristics.MetaType;

/*
 * Local Search hasn't been implemented yet. 
 */

public class LocalSearch {
    private MetaType metaheuristic;

    public LocalSearch() {
        this.metaheuristic = Config.META_HEURISTIC;
    }


     private int[][] findBestNeighbor(int[][] c) {
        return null;
    }

    public void perturbate() {
        
    }

    private double evaluate(int[][] e) {
        return 0.0; 
    }
}
