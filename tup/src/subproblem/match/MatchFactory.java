package subproblem.match;

import subproblem.cost.Backtracking;
import subproblem.cost.Hungarian;
import subproblem.cost.JonkerVolgenant;
import subproblem.cost.Solver;

/**
 * The MatchFactory class is responsible for creating instances of matching algorithms based on the provided type.
 * It provides a method to create an instance of a matching algorithm specified by the MatchType enum.
 */

public class MatchFactory {

    public static Match createMatchAlgorithm(MatchType mt) {
        //System.out.println("Chose " + mt);
        return switch (mt) {
            case HUNGARIAN -> new Hungarian();
            case BACK_TRACKING -> new Backtracking();
            case JONKER_VOLGENANT -> new JonkerVolgenant();
            case SOLVER -> new Solver();
        };
    }

    @Override
    public String toString() {
        return "MatchFactory{}";
    }
}
