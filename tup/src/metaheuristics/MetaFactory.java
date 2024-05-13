package metaheuristics;

public class MetaFactory {

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