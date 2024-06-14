package problem;

import model.Game;
import model.Instance;
import subproblem.match.Match;

import java.util.BitSet;

import static main.Config.*;
import static model.Instance.*;
import static subproblem.match.MatchFactory.createMatchAlgorithm;

/**
 * 1. A subgraph is derived containing:
 * • the games of round r − 1 of umpires not yet allocated in round r,
 * • the games of round r with allocations pending,
 * • the edges connecting games of these two sets.
 * 2. A matching problem defined by the subgraph is solved.
 */

public class Matcher {

    private Instance instance;
    private int branchStart;
    private int[][] subGraph;
    private int subGraphSize = 0;

    public Matcher(Instance instance){
        this.instance = instance;
    }

    public int calculatePartialMatchingCost(BitSet vec, int subGraphSize, int roundIndex) {
        this.branchStart = NUM_UMPIRES * roundIndex;
        this.subGraphSize = subGraphSize;
        return - 1;
    }


    @Override
    public String toString() {
        return "Matcher{" +
                "instance=" + instance +
                '}';
    }
}
