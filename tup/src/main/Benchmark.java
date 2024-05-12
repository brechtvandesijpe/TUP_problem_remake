package main;

import file.Reader;
import model.Game;
import model.Instance;

import java.io.File;
import java.io.IOException;
import java.lang.management.ManagementFactory;
import java.lang.management.ThreadMXBean;

import problem.Tree;

import static main.Config.*;

public class Benchmark {

    public static void main(String[] args) throws IOException {
        Benchmark benchmark = new Benchmark();

        System.out.println(purple +
                "/**\n" +
                " *  _______   ________  __    __   ______   __    __  __       __   ______   _______   __    __\n" +
                " * /       \\ /        |/  \\  /  | /      \\ /  |  /  |/  \\     /  | /      \\ /       \\ /  |  /  |\n" +
                " * $$$$$$$  |$$$$$$$$/ $$  \\ $$ |/$$$$$$  |$$ |  $$ |$$  \\   /$$ |/$$$$$$  |$$$$$$$  |$$ | /$$/\n" +
                " * $$ |__$$ |$$ |__    $$$  \\$$ |$$ |  $$/ $$ |__$$ |$$$  \\ /$$$ |$$ |__$$ |$$ |__$$ |$$ |/$$/ \n" +
                " * $$    $$< $$    |   $$$$  $$ |$$ |      $$    $$ |$$$$  /$$$$ |$$    $$ |$$    $$< $$  $$<  \n" +
                " * $$$$$$$  |$$$$$/    $$ $$ $$ |$$ |   __ $$$$$$$$ |$$ $$ $$/$$ |$$$$$$$$ |$$$$$$$  |$$$$$  \\ \n" +
                " * $$ |__$$ |$$ |_____ $$ |$$$$ |$$ \\__/  |$$ |  $$ |$$ |$$$/ $$ |$$ |  $$ |$$ |  $$ |$$ |$$  \\ \n" +
                " * $$    $$/ $$       |$$ | $$$ |$$    $$/ $$ |  $$ |$$ | $/  $$ |$$ |  $$ |$$ |  $$ |$$ | $$  |\n" +
                " * $$$$$$$/  $$$$$$$$/ $$/   $$/  $$$$$$/  $$/   $$/ $$/      $$/ $$/   $$/ $$/   $$/ $$/   $$/\n" +
                " */" + reset
        );

        System.out.println(blue + "------------------------------- 8 TEAMS -------------------------------" + reset);
        benchmark.testInstance("resources/umps8.txt", 4, 2, 34311);
        benchmark.testInstance("resources/umps8A.txt", 4, 2, 31490);
        benchmark.testInstance("resources/umps8B.txt", 4, 2, 32731);
        benchmark.testInstance("resources/umps8C.txt", 4, 2, 29879);
        System.out.println(blue + "------------------------------- 10 TEAMS -------------------------------" + reset);
        benchmark.testInstance("resources/umps10.txt", 5, 2, 48942);
        benchmark.testInstance("resources/umps10A.txt", 5, 2, 46551);
        benchmark.testInstance("resources/umps10B.txt", 5, 2, 45609);
        benchmark.testInstance("resources/umps10C.txt", 5, 2, 43149);
        System.out.println(blue + "------------------------------- 12 TEAMS -------------------------------" + reset);
        benchmark.testInstance("resources/umps12.txt", 7, 2, 86889);
        benchmark.testInstance("resources/umps12.txt", 5, 3, 93679);
        benchmark.testInstance("resources/umps12.txt", 4, 3, 89826);
        System.out.println(blue + "------------------------------- 14 TEAMS -------------------------------" + reset);
        benchmark.testInstance("resources/umps14.txt", 7, 2, 146656);
        benchmark.testInstance("resources/umps14.txt", 8, 3, 172177);
        benchmark.testInstance("resources/umps14.txt", 8, 2, 147824);
        benchmark.testInstance("resources/umps14.txt", 7, 3, 164440);
        benchmark.testInstance("resources/umps14B.txt", 6, 2, 138241);
        benchmark.testInstance("resources/umps14.txt", 6, 2, 145124);
        benchmark.testInstance("resources/umps14.txt", 6, 3, 158875);
        benchmark.testInstance("resources/umps14.txt", 5, 3, 154962);
        benchmark.testInstance("resources/umps14.txt", 5, 2, 143357);
        System.out.println(blue + "------------------------------- 16 TEAMS -------------------------------" + reset);
        benchmark.testInstance("resources/umps16.txt", 8, 2, -1);
    }

    private void testInstance(String instanceFileName, int Q1, int Q2, int expectedValue) throws IOException {
        Game.gameId = 0;

        Config.FILE_NAME = instanceFileName;
        Config.Q1 = Q1;
        Config.Q2 = Q2;

        ThreadMXBean threadMXBean = ManagementFactory.getThreadMXBean();
        Instance instance = Reader.read(new File(instanceFileName));
        Tree tree = new Tree(instance, 0, NUM_ROUNDS - 1, false);
        long startExecution = threadMXBean.getCurrentThreadCpuTime();
        tree.startGlobalTraversal();
        long endExecution = threadMXBean.getCurrentThreadCpuTime();

        int upperbound = tree.getUpperbound();
        String solutionMessage = (upperbound == Integer.MAX_VALUE) ? "Infeasible" : String.valueOf(upperbound);

        if (upperbound == expectedValue) {
            System.out.println(green + "[PASSED]  " + reset + instanceFileName + " {" + Q1 + "," + Q2 + "} : Expected " + expectedValue + ", Actual " + solutionMessage + " - " + (endExecution - startExecution) / 1_000_000_000.0 + " sec");
        } else {
            System.out.println(red + "[FAILED]  " + reset + instanceFileName + " {" + Q1 + "," + Q2 + "} : Expected " + expectedValue + ", Actual " + solutionMessage + " - " + (endExecution - startExecution) / 1_000_000_000.0 + " sec");
        }

    }
}