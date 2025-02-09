import java.awt.Robot
import com.sun.xml.internal.ws.api.server.Module
import scala.io.StdIn._
import scala.util.control._
import scala.util.control.Breaks._
import scala.StringContext._

/**
Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/

object MoleculeType extends Enumeration {
  type MoleculeType = Value
  val A = Value(0,"A")
  val B = Value(1,"B")
  val C = Value(2,"C")
  val D = Value(3,"D")
  val E = Value(4,"E")
  //  val A,B,C,D,E= Value
}

class Player {
  var samplesTaken: Int
  var available: Array[Int]
  var projects: List[Project]
  var robots: List[Robot]
  var samples: List[Sample]

  def main(args: Array[String]): Unit = {
    robots =  List[Robot]()
    samples =  List[Sample]()
    projects = List[Project]()

    val firstTurn = true
    val projectCount = readLine.toInt
    for(i <- 0 until projectCount) {
      val Array(a, b, c, d, e) = (readLine split " ").map (_.toInt)
      val inputs = new Array[Int](5)
      inputs(0)=a.toInt
      inputs(1)=b.toInt
      inputs(2)=c.toInt
      inputs(3)=d.toInt
      inputs(4)=e.toInt
    }

    // game loop
    while(true) {
      for(i <- 0 until 2) {
        val Array(_target, _eta, _score, _storageA, _storageB, _storageC, _storageD, _storageE, _expertiseA, _expertiseB, _expertiseC, _expertiseD, _expertiseE) = readLine split " "
        val inputs = new Array[Int](12)
        val target:String = inputs(0).toString
        val eta = _eta.toInt
        inputs(1)=eta.toInt
        val score = _score.toInt
        inputs(2)=score.toInt
        val storageA = _storageA.toInt
        inputs(3)=storageA.toInt
        val storageB = _storageB.toInt
        inputs(4)=storageB.toInt
        val storageC = _storageC.toInt
        inputs(5)=storageC.toInt
        val storageD = _storageD.toInt
        inputs(6)=storageD.toInt
        val storageE = _storageE.toInt
        inputs(7)=storageE.toInt
        val expertiseA = _expertiseA.toInt
        inputs(8)=expertiseA.toInt
        val expertiseB = _expertiseB.toInt
        inputs(9)=expertiseB.toInt
        val expertiseC = _expertiseC.toInt
        inputs(10)=expertiseC.toInt
        val expertiseD = _expertiseD.toInt
        inputs(11)=expertiseD.toInt
        val expertiseE = _expertiseE.toInt
        inputs(12)=expertiseE.toInt

        var modTarget : Module = new StartPoint()
        def target (target:String) : String = target match {
          case "START_POSITION" => modTarget = new StartPoint()
          case "SAMPLES" => modTarget = new Samples()
          case "DIAGNOSIS" => modTarget = new Diagnosis()
          case "MOLECULES" => modTarget = new Molecules()
          case "LABORATORY" => modTarget = new Laboratory()
        }

        robots+ = new Robot{
          override var target: Module = _
          override var eta: Int = _
          override var score: Int = _
          override var storage: Array[Int] = _
          override var expertise: Array[Int] = _
          override var sample: Sample = _
          override def Connect(moleculeType: MoleculeType.MoleculeType): Unit = ???}

      }
      val Array(availableA, availableB, availableC, availableD, availableE) = (readLine split " ").map (_.toInt)
      val sampleCount = readLine.toInt
      val inputs = new Array[Int](4)
      inputs(0)=availableA.toInt
      inputs(1)=availableB.toInt
      inputs(2)=availableC.toInt
      inputs(3)=availableD.toInt
      inputs(4)=availableE.toInt
      val avaliable = Array(availableA, availableB, availableC, availableD, availableE)

      for(i <- 0 until sampleCount) {
        val Array(_sampleId, _carriedBy, _rank, _expertiseGain, _health, _costA, _costB, _costC, _costD, _costE) = readLine split " "
        val inputs = new Array[Int](9)
        val sampleId = _sampleId.toInt
        inputs(0)= sampleId.toInt
        val carriedBy = _carriedBy.toInt
        inputs(1)=carriedBy.toInt
        val rank = _rank.toInt
        inputs(2)= rank.toInt
        val health = _health.toInt
        inputs(3)= health.toInt
        val costA = _costA.toInt
        inputs(4)= costA.toInt
        val costB = _costB.toInt
        inputs(5)= costB.toInt
        val costC = _costC.toInt
        inputs(6)= costC.toInt
        val costD = _costD.toInt
        inputs(7)= costD.toInt
        val costE = _costE.toInt
        inputs(8)= costE.toInt
        val expertiseGain = _expertiseGain.toInt
        inputs(9)= expertiseGain.toInt


        val thisSample : Sample = new Sample {
          override var id: Int = _
          var SampleId: Int = _
          override var cost: Array[Int] = _
          override var health: Int = _
          override var rank: Int = _
          var expertiseGain: Boolean = _
          override var diagnosticated: Boolean = _
        }

        carriedBy match {
          case -1 => samples+ = (thisSample)
          case 0 => robots(0).samples+ = thisSample
          case 1 => robots(1).samples+ = thisSample
        }
      }
    }
    val myRobot: Robot = robots(0)

    Console.err.println("Module : " +  myRobot.target.toString)
    Console.err.println("Storage (A B C D E) : " + myRobot.storage)
    Console.err.println("Expert. (A B C D E) : " + myRobot.expertise)
    val potential = new Array[Int](5)
    for(i <- 0 until 2){
      potential(i) = myRobot.storage(i) + myRobot.expertise(i)
    }
    for(project <- projects){
      Console.err.println("Projet " + projects.indexOf(project) + "  Expertise : " + project.expertise)
    }
    Console.err.println("")
    Console.err.println("=========== SAMPLES ===========")
    Console.err.println("")

    Console.err.println("Potential (A B C D E) : " + potential)
    Console.err.println("Available (A B C D E) : " + available)
    Console.err.println("")

    for(sample <- myRobot.samples.sortBy(item => item.health).reverse){
      Console.err.println("Samp Cost (A B C D E) : " +  sample.cost + " - Rank " + sample.rank + " - Health : " + sample.health + " - Gain : " + sample.gain.toString + " - ID : " + sample.id)
    }
    myRobot.Update
  }
}

abstract class Project{
  var expertise: Array[Int]
  def project (_expertise:Array[Int]): Unit ={
    this.expertise = _expertise
  }
}

abstract class Sample{
  //Sample object will be defined its own properties.
  var id: Int
  var cost :Array[Int]
  var health : Int
  var rank: Int
  var gain = MoleculeType(gain)
  var diagnosticated : Boolean

  def Sample (_id:Int, _cost:Array[Int], _health:Int, _rank:Int, _gain: String): Unit ={
    this.id=_id
    this.cost=_cost
    this.health=_health
    this.rank=_rank
    gain = MoleculeType.withName(_gain)
    var isEmpty :Boolean = false
    if(_cost.length <= 0){
      isEmpty = true
    }
    diagnosticated = isEmpty
  }
}

abstract class Molecule{
  //Molecule class is where MoleculeType property defined.
  var moltype: MoleculeType.MoleculeType
  def Molecule(_type:String): Unit ={
    moltype = MoleculeType.withName(_type)
  }
}

abstract class Robot {
  var samples: List[Sample] = samples // list of samples that robot has
  var target: Module
  var eta: Int
  var score: Int
  var storage: Array[Int] //for memory
  var expertise: Array[Int]
  var toBeResearched = new Nothing // //to be placed samples that will be research (write for Queue)

  def Robot(_target: Module, _eta: Int, _score: Int, _storage: Array[Int], _expertise: Array[Int]): Unit = {
    this.target = _target
    this.eta = _eta
    this.score = _score
    this.expertise = _expertise
    this.samples = List()
  }
  //To update destination of robot
  def Update: Unit = {
    target.GetDecision(this)
  }
  //To control if collect molecules needed
  def CanCollectMolecules(sample: Sample): Boolean = {
    var canDo = true
    var nbNeeded: Int = 0
    for (i <- 0 until 5) {
      nbNeeded += Math.max(0, sample.cost(i) - storage(i) - expertise(i))
    }
    if (nbNeeded > (10 - storage.sum)) {
      canDo = false
    }
    println(canDo)
    return canDo
  }

  var sample: Sample
  //Control if samples can be researchable
  def SamplesResearchable: Boolean = {
    //Control all samples one by one
    for (sample <- samples) {
      //Control if the sample can be researchable
      if (CanResearchSample(sample)) {
        //If it is researchable then, return true
        return true
      }
    }
    return false
  }
  //control if samples can be done
  def SampleDoAble: Boolean = {
    for (sample <- samples) {
      if (CanResearchSample(sample: Sample)) {
        return true
      }
    }
    return false
  }
  //Evaluate if sample is convenient
  def IsSampleGoodForProjects(sample: Sample): Boolean = {
    var isGood: Boolean = true
    if (6 <= expertise.sum) {
      //to compute points depend on module
      val goals = new Array[Int](5)
      val player: Player = new Player()
      for (project <- player.projects) {
        for (i <- 0 until 5) {
          goals(i) = Math.max(project.expertise(i), goals(i))
        }
      }
      //assigning gain of sample to gainMolecule
      val gainMolecule: Int = sample.gain.id
      //compare goal of gainMolecule and expertise of gainMolecule
      //if goal is smaller than expertise than sample is not good
      if (expertise(gainMolecule) >= goals(gainMolecule)) {
        isGood = false
      }
    }
    return isGood
  }
  //Calculation of project score
  def ScoreFromProject(project: Project): Int = {
    var score: Int = 0
    for (i <- 0 until 5) {
      score += Math.max(project.expertise(i) - expertise(i), 0)
    }
    println("Score" + score)
    return score
  }
  //Control if any sample can be done
  def CanDoSample(sample: Sample): Boolean = {
    var canDo: Boolean = true
    //If any sample cannot be researchable and storage array contains more than or equal to 10 elements or sample is not available for project then sample can not be done
    // so canDo property changed to false
    if ((!(CanResearchSample(sample)) && storage.sum >= 10) || !IsSampleGoodForProjects(sample)) {
      canDo = false
    } else {
      for (i <- 0 until 5) {
        //control player's available list's ith element and sum up it with ith element of storage and ith element of expertise
        //compare this sum with cost of the sample.
        var player: Player = new Player()
        if ((player.available(i) + storage(i) + expertise(i)) < sample.cost(i)) {
          canDo = false
          break
        }
      }
    }
    Console.err.println("Sample " + samples.indexOf(sample) + " can be done : " + canDo.asInstanceOf[Nothing])
    return canDo
  }
  //control if project is close to the most convenient solution
  def IsCloseToProjectEnd(sample: Sample): Boolean = {
    val closestProject = ClosestProject()
    if (CanDoSample(sample)) {
      val molecule = sample.gain.asInstanceOf[Int]
      if (closestProject.expertise(molecule) > expertise(molecule) && ScoreFromProject(closestProject) < 2) {
        return true
      }
    }
    return false
  }
  //to calculate neeeded depend on sample
  def neededForSample(sample: Sample): Int = {
    val needed: Array[Int] = new Array[Int](5)
    for (i <- 0 until 5) {
      needed(i) = Math.max(sample.cost(i) - expertise(i) - storage(i), 0)
    }
    return needed.sum
  }
  //To find closest point of the project
  def ClosestProject(): Project = {
    var minScore: Int = 20
    val player: Player = new Player()
    var cProject = player.projects.head
    for (p <- player.projects) {
      val tempScore: Int = ScoreFromProject(p)
      if (tempScore < minScore && tempScore > 0) {
        minScore = tempScore
        cProject = p
      }
    }
    Console.err.println("Project : " + player.projects.indexOf(cProject) + minScore)
    return cProject
  }
  //to define diagnosticable sample
  def ChooseDiagnosticableSample(): Sample = {
    val tempSamples: List[Sample] = samples.filter(_.diagnosticated == false)
    if (tempSamples.length == 0) {
      return null
    }
    return tempSamples.sortBy(sample => sample.health).reverse.head
  }
  //To control if sample can be researchable.
  def CanResearchSample(sample: Sample): Boolean = {
    var canDo: Boolean = true
    for (i <- 0 until 5) {
      //Control ith element of storage and expertise and sum up
      //If calculated sum is smaller than cost of the sample then sample cannot be researched.
      if ((storage(i) + expertise(i)) < sample.cost(i)) {
        canDo = false
      }
    }
    Console.err.println("Sample " + samples.indexOf(sample) + " can be researched : " + canDo.asInstanceOf[Nothing])
    return canDo
  }
  //To send robot selected module
  def GoTo(module: String): Unit = {
    Console.println("GOTO " + module)
  }
  //To connect robot selected module id
  def Connect(id: Int): Unit = {
    Console.println("CONNECT " + id)
  }
  //To connect robot selected molecule type
  def Connect(moleculeType: MoleculeType.MoleculeType): Unit = {
    Console.println("CONNECT " + moleculeType)
  }
  //Wait for next operation
  def Wait: Unit = {
    Console.println("WAIT ")
  }
}

abstract class Module() {
  //To define decision of robot or another saying moves.
  def GetDecision(robot: Robot): Unit
  //Check if any sample researchable.
  def SampleResearchable(robot: Robot): Boolean = {
    val canDo: Boolean = robot.SamplesResearchable
    Console.err.println("At least a research can be done : " + canDo.asInstanceOf[Nothing])
    return canDo
  }
  //Check if any sample can do.
  def SampleDoAble(robot: Robot): Boolean = {
    val canDo = robot.SampleDoAble
    Console.err.println("At least a sample can be done : " + canDo.asInstanceOf[Nothing])
    return canDo
  }
}

class StartPoint extends Module{
  def GetDecision(robot: Robot): Unit = {
    robot.GoTo("SAMPLES")
  }
}

class Samples extends Module {
  override def GetDecision(robot: Robot): Unit = {
    val rnd = scala.util.Random
    if (robot.expertise.sum == 0) {
      if (robot.samples.length < 2) {
        robot.Connect(1)
        return
      } else {
        robot.GoTo("DIAGNOSIS")
        return
      }
    }
    else if (robot.samples.length < 3) {
      if (robot.ScoreFromProject(robot.ClosestProject()) < 1) {
        val start = -4
        val end = 4
        robot.Connect(start + rnd.nextInt((end - start) + 1))
        return
      }
      if (robot.expertise.sum < 6) {
        robot.Connect(1)
        return
      }
      else if (robot.expertise.sum < 10) {
        robot.Connect(2)
        return
      }
      else {
        robot.Connect(3)
        return
      }
    } else {
      robot.GoTo("DIAGNOSIS")
      return
    }
    return
  }
}

class Diagnosis extends Module {
  //Diagnosis module
  override def GetDecision(robot: Robot): Unit = {
    // Select a sample which can be diagnosticable by robot
    val selectedSample = robot.ChooseDiagnosticableSample
    //If selectedSample exist, then connect that sample with sample id
    if (selectedSample != null) {
      robot.Connect(selectedSample.id)
      return
    }
    else {
      //Check samples that robot has one by one
      for (s <- robot.samples) {
        //If next sample can not be done by robot or needed sample is greater than 6 and rank of the sample is smaller than 3
        if (!robot.CanDoSample(s) || (robot.neededForSample(s) > 6 && s.rank < 3)) {
          robot.Connect(s.id)
          return
        }
      }
      // We take researchable samples if they exist in the cloud
      if (robot.samples.length < 3) {
        val player: Player = new Player()
        for (s <- player.samples) {
          //if sample can be researchabe and robot needs sample
          if (robot.CanResearchSample(s) && robot.neededForSample(s) < 3) {
            robot.Connect(s.id)
            return
          }
        }
      }
      //If robot has any sample and score of the player smaller than 1
      if (robot.samples.length > 0 && robot.ScoreFromProject(robot.ClosestProject()) < 1) {
        for (sample <- robot.samples) {
          if (!robot.IsCloseToProjectEnd(sample)) {
            robot.Connect(sample.id)
            return
          }
        }
      }
      var module: Module = new Module {
        override def GetDecision(robot: Robot): Unit = ???
      }
      // if at least one sample is researchable goto labobratory
      if (module.SampleResearchable(robot): Boolean) {
        robot.GoTo("LABORATORY")
        return
      }
      // if at least one sample is realisable goto molecules
      else if (SampleDoAble(robot)) {
        robot.GoTo("MOLECULES")
        return
      }
    }
    // go to sample and collect new samples
    robot.GoTo("SAMPLES")
    return
  }
}

class Molecules extends Module {
  //Module of Molecule
  override def GetDecision(robot: Robot): Unit = {
    val needed: Array[Int] = new Array[Int](5)
    //A robot can carry at most 10 molecule
    if (robot.storage.sum < 10) {
      val index: Int = 0
      for (sample <- robot.samples.sortBy(s => s.rank)) {
        if (index != 0) {
          val previousSample: Sample = robot.samples.sortBy(s => s.rank).toList(index - 1)
          if (robot.CanDoSample(previousSample)) {
            needed(previousSample.gain.asInstanceOf[Int]) -= 1
          }
        }
        //Check if robot can collect molecule
        if (robot.CanCollectMolecules(sample)) {
          for (i <- 0 until 5) {
            needed(i) += Math.max(sample.cost(i) - robot.expertise(i), 0)
            if (needed.sum > 10) {
              if (robot.samples.sortBy(s => s.rank).indexOf(sample) != 0) {
              }
              Console.err.println("First Break")
              break()
            }
          }
          Console.err.println("needed : " + needed)
          if (needed.sum > 10) {
            if (robot.samples.sortBy(s => s.rank).indexOf(sample) != 0) {
            }
            Console.err.println("Second Break")
            break()
          }
          //control if robot can not research sample
          if (!robot.CanResearchSample(sample)) {
            Console.err.println("Can not research sample")
            if (robot.CanDoSample(sample) && needed.sum <= 10) {
              Console.err.println("Beginning molecule collection")
              for (i <- 0 until 5) {
                val player: Player = new Player()
                Console.err.println("Collection molecule" + MoleculeType(i) + "??")
                if (needed(i) - robot.storage(i) > 0 && player.available(i) > 0) {
                  Console.err.println(": Yes")
                  robot.Connect(MoleculeType(i))
                  return
                }
                else {
                  Console.err.println(": No")
                }
              }
            }
          }
        }
      }
      //check if sample researchable or needed sequence sum is greater than 10
      if (SampleResearchable(robot) || needed.sum > 10) {
        Console.err.println(" +10 needed : " + needed)
        Console.err.println("Can research sample, goto laboratory ")
        robot.GoTo("LABORATORY")
        return
      }
      else {
        Console.err.println(" +10 needed : " + needed)
        Console.err.println("Cannot research sample, goto diagnosis")
        robot.GoTo("DIAGNOSIS")
        return
      }
      return
    }
    else {
      //If sample can not be researchable
      if (!SampleResearchable(robot)) {
        Console.err.println(" +10 needed : " + needed)
        Console.err.println("Cannot research sample, goto diagnosis")
        robot.GoTo("DIAGNOSIS")
        return
      }
      else {
        Console.err.println(" +10 needed : " + needed)
        Console.err.println("Can research sample, goto laboratory")
        robot.GoTo("LABORATORY")
        return
      }
    }
    return
  }
}

class Laboratory extends Module {
  //Laboratory module.
  override def GetDecision(robot: Robot): Unit = {
    if (SampleResearchable(robot)) {
      for (sample <- robot.samples) {
        if (robot.CanResearchSample(sample)) {
          robot.Connect(sample.id)
          return
        }
      }
    }
    if (!SampleDoAble(robot) || robot.samples.length < 2) {
      if (robot.samples.length < 3) {
        robot.GoTo("SAMPLES")
        return
      }
      else {
        robot.GoTo("DIAGNOSIS")
        return
      }
    }
    else {
      robot.GoTo("MOLECULES")
      return
    }
    return
  }
}