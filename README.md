# Programming Languages Coding Games

Group Members : Tuğba Güler / Fatma Erkan 

Code4Life - Roche Game

Game Link: 
https://www.codingame.com/ide/puzzle/code4life

The game has been implemented in Scala and Rust languages which are functional programming languages.


#Game Information and Our Strategy

The game is played with two players. Players have a robot and they control these robots. There are 3 basic modules in the code. Diagnostics, molecule, and laboratory.  Robots can transfer two types of elements between modules: samples and molecules. That's why robot movements are optimized. 
- Robots collect sample data files from the cloud in the DIAGNOSIS module.
- Collects the necessary molecules for medicines in the MOLECULES module.
- Medicine is produced in the LABORATORY module and health scores are collected. Sample data are researched to increase players' health score. 
The starting points of the robots that players control is the same. A robot can carry up to 3 samples and 10 molecules.  The player can move his robots from one module to another with the console (“GOTO” …) statement. While the robot is in the interface of a module, it connects to it with the CONNECT command. The sample data file defined by CONNECT id in the diagnosis machine transfers to the robot. In the molecule distribution module, it will transfer an existing molecule to your robot of the desired type. For the laboratory module, the player's robot must carry a sample data file and the required amount of molecules to produce the medicine for that sample.
In conclusion, robots collect samples. After collecting samples, the robot connects to the sample with the sample id after connecting that sample, it goes to Diagnosis. If the robot cannot find the correct sample, then it goes to samples again. Researchable sample or another saying correct sample is got by robot and it goes to Laboratory. The laboratory is the place that medicines are produced. If any sample cannot be done or robot has at most 1 sample in the lab. Otherwise, write that robot go to the Diagnosis module or Molecules module. If the robot has less than 10 molecules robot cannot research it needs 10 molecules. If 10 molecules exist, then the robot goes to the laboratory to search.



NOTE: The codes written for the game were developed by both of us through this strategy. The same strategy is written in two different functional programming languages.
