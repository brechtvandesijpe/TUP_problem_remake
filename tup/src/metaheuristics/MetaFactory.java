package metaheuristics;

public class MetaFactory {

    /**
     * The MetaFactory class is responsible for creating instances of metaheuristics based on the provided type.
     * It provides a method to create an instance of a metaheuristic specified by the MetaType enum.
     */

    public static Meta createMetaheuristic(MetaType mt) {
        //System.out.println("Chose " + mt);
        return switch (mt) {
            case STEEPEST_DESCENT -> new SteepestDescent();
            case SIMULATED_ANNEALING -> new SimulatedAnnealing();
        };
    }

    @Override
    public String toString() {
        return "MatchFactory{}";
    }
}