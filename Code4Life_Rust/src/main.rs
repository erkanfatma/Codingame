use std::io;

//for random generator
extern crate rand;
use rand::Rng;
// for foreach 
use foreach::Continue::*;
//for max operation
use std::cmp;
//for queue
extern crate queues;
use queues::*;
 
enum MoleculeType { A, B, C, D, E }
 
trait Module{
     fn GetDesicion<T: Robot>(robot: T) -> ();
     fn SampleResearchable<T: Robot>(robot: T) -> bool{
         let mut canDo: bool = robot.SamplesResearchable();
         println!("At least one research can be done: {}" , canDo);
        canDo;
     }

     fn SampleDoable<T: Robot>(robot: T) -> bool{
         let mut canDo: bool = robot.SamplesDoable();
         println!("At least one sample can be done: {}", canDo);
         canDo;
     }
}
 
impl StartPoint: Module{
    fn GetDesicion<T: Robot>(robot: T) -> (){
        robot.GoTo("SAMPLES");
    }
}
 
impl Samples: Module{
    fn GetDesicion<T: Robot>(robot: T) -> (){
        let mut rng = rand::thread_rng();
         if robot.expertise.Sum() == 0 {
             if robot.samples.Count <2 {
                 robot.Connect(1);
                 return;
             }else{
                 robot.GoTo("DIAGNOSIS");
                 return;
             }
         }else if robot.samples.Count <3 {
             if robot.ScoreFromProject(robot.ClosestProject())<1 {
                 robot.Connect(rng.gen_range(1, 4));
                 return;
             }
             if robot.expertise.Sum() <6 {
                 robot.Connect(1);
                 return;
             }else if robot.expertise.Sum() <10  {
                 //robot.Connect(rng.gen_range(1,3));
                 robot.Connect(2);
                 return;
             }else{
                 robot.Connect(3);
                 return;
             }
         }else{
             robot.GoTo("DIAGNOSIS");
             return;
         }  
         return;
    }
}
 
impl Diagnosis: Module{ 
    fn GetDesicion<T: Robot>(robot: T) -> (){ 
        let selectedSample: Sample = robot.ChooseDiagnosticableSample();
        if selectedSample == None {
            robot.Connect(selectedSample.id);
            return;
        }
        else{
            // We put apart samples not realisable
            for_each!(s in robot.Samples{
                if !robot.CanDoSample(s) || (robot.neededForSample(s) >6 && s.rank <3) {
                    robot.Connect(s.id);
                    return;
                }
            }); 

            // We take researchable samples if they exist in the cloud
            if robot.samples.Count<3 {
                for_each!(s in Player.samples{
                    if robot.CanResearchSample(s) && robot.neededForSample(s) <3 {
                        robot.Connect(s.id);
                        return;
                    }
                });
            }

            // If we're really close to one project, we ditch all samples that do not help to finish it
            if robot.samples.Count >0 && robot.ScoreFromProject(robot.ClosestProject())<1 {
                for_each!(sample in robot.samples{
                    if !robot.IsCloseToProjectEnd(sample) {
                        robot.Connect(sample.id);
                        return;
                    }
                });
            }

            // if at least one sample is researchable goto lab
            if SampleResearchable(robot) {
                robot.GoTo("LABORATORY");
                return;
            }
            // if at least one sample is realisable goto molecules
            else if SampleDoable(robot) {
                robot.GoTo("MOLECULES");
                return;
            }
        }
        robot.GoTo("SAMPLES");
        return;
    }
}
 
impl Molecules: Module{
    fn GetDesicion<T: Robot>(robot: T) -> (){
        let mut needed:[i32;5] = [0,0,0,0,0];
        //
        if robot.storage.Sum() <10 {
            let index: i32 = 0;
            //
            for_each!(sample in robot.samples.OrderBy(s => s.rank){
               if index != 0 {
                   //
                   let mut previousSample: Sample = robot.samples.OrderBy(s => s.rank)[index-1];
                   if robot.CanDoSample(previousSample) {
                       needed[previousSample.gain.parse::<i32>()unwrap()] -=1;
                   }
               } 
               if robot.CanCollectMolecules(sample) {
                   for i in 0..5 as usize {
                       needed[i] += cmp::max(sample.cost[i] - robot.expertise[i], 0);
                       if needed.Sum() > 10 {
                           //
                           if robot.samples.OrderBy( s => s.rank).IndexOf(sample) !=0 {
                                needed = [0,0,0,0,0];
                           }
                           println!("First break");
                           break;
                       }    
                   }
                   println!("needed: {}", needed);
                   if needed.Sum() > 10 {
                       //
                       if robot.samples.OrderBy(s => s.rank).IndexOf(sample) != 0 {
                            needed = [0,0,0,0,0];
                       }
                       println!("Second break");
                       break;
                   } 
                   if !robot.CanResearchSample(sample) {
                       println!("Cannot research sample");
                       if robot.CanDoSample(sample) && needed.Sum() <=10 {
                           println!("Beginning molecule collection");
                           for i in 0...5 as usize{
                               //
                               println!("Collection module {} ??", (MoleculeType)i) ;
                               if needed[i] - robot.storage[i] > 0 && Player.available[i] >0 {
                                   println!(" : Yes" );
                                   //
                                   robot.Connect((MoleculeType)i);
                                   return;
                               }else {
                                   println!(" : No");
                               }
                           }
                       }
                   }   
               }
            });
            //
            if SampleResearchable(robot) || needed.Sum() > 10 {
              println!("needed: {}", needed );  
              println!("Can research sample, goto lab");
              robot.GoTo("LABORATORY"); 
              return;
            }else{
                println!("needed: {}",needed );
                println!("Cannot research sample, goto diag");
                robot.GoTo("DOAGNOSIS");
                return;
            }
            return;
        }
        else{
            if !SampleResearchable(robot) {
                println("+10 needed: {}", needed);
                println!("Cannot research sample, goto diag" );
                robot.GoTo("DIAGNOSIS");
                return;
            }else{
                println("+10 needed: {}", needed);
                println!("Can research sample, goto lab" );
                robot.GoTo("LABORATORY");
                return;
            }
        }
        return;
    }
}

impl Laboratory: Module{
    fn GetDesicion<T: Robot>(robot: T) -> (){
        if SampleResearchable(robot) {
            for_each!(sample in robot.samples{
                if robot.CanResearchSample(sample) {
                    robot.Connect(sample.id);
                    return;
                }
            });
        }
        //
        if !SampleDoable(robot) || robot.samples.Count <2 {
            //
            if robot.samples.Count <3 {
                robot.GoTo("SAMPLES");
                return;
            }else{
                robot.GoTo("DIAGNOSIS");
                return;
            }
        }else {
            robot.GoTo("MOLECULES");
            return;
        }
        return;
    }
}

struct Project{
    //
    expertise: [i32],
}
//impl Project{
    
// }

struct Sample{
    id :i32,
    cost[i32],
    health: i32,
    rank: i32,
    gain: MoleculeType,
    diagnosticated: bool,
}

impl Struct{
    //
    diagnosticated = Array.exist(cost, number == -1) ? false: true;
}

struct Molecule{
    type: MoleculeType,
}

struct Robot{
    samples: vec![],
    target: Module,
    eta: i32, 
    score: i32,
    //
    storage: [i32],
    expertise: [i32],
    toBeResearched: Queue<isize> = queue![],
}

impl Robot{
    fn Update() ->(){
        target.GetDesicion();
    }

    fn CanCollectMolecules<T: Sample>(sample: T) -> bool{
        let mut nbNeeded: i32 = 0;
        let mut canDo:bool = true;

        for i in 0...5 as usize{
            nbNeeded += cmp::max(0, sample.cost[i] - storage[i] - expertise[i]);
        }
        //
        if nbNeeded > (10 - storage.Sum()) {
            canDo = false;
        }
        canDo;
    }

    fn SamplesDoable() -> bool{
        for_each!(sample in samples{
            if CanDoSample(sample) {
                true;
            }
        });
        false;
    }

    fn SamplesResearchable() -> bool{
        for_each!(sample in samples{
            if CanResearchSample(sample) {
                true;
            }
        });
        false;
    }

    fn IsSampleGoodForProjects<T:Sample(sample: T) -> bool{
        let mut isGood:bool = true;
        //
        if expertise.Sum() >= 6 {
            let mut goals:[i32;5] = [0,0,0,0,0];
            for_each!(project in Player.projects{
                for i in 0...5 as usize{
                    goals[i] = cmp::max(project.expertise[i], goals[i]);
                }
            });

            let mut gainMolecule:i32 = sample.gain;
            if goals[gainMolecule] <= expertise[gainMolecule] {
                isGood = false;
            }
        }
        isGood;
    }

    fn ScoreFromProject<T: Project>(project:T) ->i32{
        let mut score:i32 = 0;
        for i in 0...5 as usize{
            score += cmp::max(project.expertise[i] -expertise[i],0);
        }
        score;
    }

    fn IsCloseToProjectEnd<T: Sample>(sample:T) -> bool{
        let mut closestProject: Project = ClosestProject();

        if CanDoSample(sample) {
            let mut molecule:i32 = sample.gain;
            if closestProject.expertise[molecule] > expertise[molecule] && ScoreFromProject(closestProject) <32 {
                true;
            }
        }
        false;
    }

    fn neededForSample<T: Sample>(sample: T) -> i32{
        let mut needed:[i32;5] = [0,0,0,0,0];
        for i in 0...5 as usize{
            needed[i] = cmp::max(sample.cost[i] -expertise[i] - storage[i],0);
        }
        needed.Sum();
    }

    fn ClosestProject() -> Project{
        let mut minScore: i32 = 20;
        let mut cProject:Project = Player.projects.First();

        for_each!(p in Player.Projects{
            let mut tempScore:i32 = ScoreFromProject(p);
            if tempScore < minScore && tempScore > 0 {
                minScore = tempScore;
                cProject = p;
            }
        });
        println!("Project vise: {}, ecart: {}",Player.projects.IndexOf(cProject),  minScore);
        cProject;
    }

    fn ChooseDiagnosticableSample() -> Sample {
        // ???
        let tempSamples: vec! =
   //     List<Sample> tempSamples = samples.Where(sample => sample.diagnosticated == false).ToList();
     //   return tempSamples.Count == 0 ? null : tempSamples.OrderByDescending(sample => sample.health).First();
    
    }

    fn CanResearchSample<T:Sample>(sample: T) -> bool {
        let mut canDo:bool = true;

        for i in 0...3 as usize{
            if storage[i] + expertise[i] < sample.cost[i] {
                canDo = false;                
            }
        }
        println!("Sample {} can be researched {}", samples.IndexOf(sample), canDo);
        canDo;
    }

    fn CanDoSample<T:Sample>(sample: T) -> bool{
        let mut canDo:bool = true;
        if !CanResearchSample(sample) && storage.Sum() >=10 || IsSampleGoodForProjects(sample) {
            canDo = false;
        }else{
            for i in 0...5 as usize{
                if Player.available[i] + storage[i] + expertise[i] < sample.cost[i] {
                    canDo = false;
                    break;
                }
            }
        }
        print!("Sample {} can be done {}", samples.IndexOf(sample), canDo);
        return canDo;
    }

    fn GoTo(module: String) ->(){
        println!("GOTO {}", module);
    }

    fn Connect(id:i32) ->(){
        println!("CONNECT {}", id);
    }

    fn Connect<T:MoleculeType>(type: T) -> (){
        println!("CONNECT {}", type );
    }

    fn Wait() -> (){
        println!("WAIT");
    }

}

struct Player{
    static samplesTaken: i32, 
    static available: [i32],
    //
    static projects: Vec::new(),
    static robots: Vec::new(),
    static samples: Vec::new(),

}

impl Player{

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

    /**
    * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
    **/
    fn main() {
        let mut firstTurn:bool = true;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let project_count = parse_input!(input_line, i32);
        for i in 0..project_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let a = parse_input!(inputs[0], i32);
            let b = parse_input!(inputs[1], i32);
            let c = parse_input!(inputs[2], i32);
            let d = parse_input!(inputs[3], i32); 

            let mut project: [i32;5] = [a,b,c,d,e];
            projects.push(project);
        }

        // game loop
        loop {
            robots.clear();
            samples.clear();

            for i in 0..2 as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();
                let target = inputs[0].trim().to_string();
                let eta = parse_input!(inputs[1], i32);
                let score = parse_input!(inputs[2], i32);
                let storage_a = parse_input!(inputs[3], i32);
                let storage_b = parse_input!(inputs[4], i32);
                let storage_c = parse_input!(inputs[5], i32);
                let storage_d = parse_input!(inputs[6], i32);
                let storage_e = parse_input!(inputs[7], i32);
                let expertise_a = parse_input!(inputs[8], i32);
                let expertise_b = parse_input!(inputs[9], i32);
                let expertise_c = parse_input!(inputs[10], i32);
                let expertise_d = parse_input!(inputs[11], i32);
                let expertise_e = parse_input!(inputs[12], i32);

                let mut modTarget:Module = None;
                match target{
                    "START_POS" =>{
                        modTarget = new StartPoint();
                    },
                    "SAMPLES" =>{
                        modTarget = new Samples();
                    },
                    "DIAGNOSIS" =>{
                        modTarget = new Diagnosis();
                    },
                    "MOLECULES" => {
                        modTarget = new Molecules();
                    },
                    "LABORATORY" =>{
                        modTarget = new Laboratory();
                    },
                    _ => {
                    }
                }

                let mut storageArray: [i32;5] = [storage_a, storage_b, storage_c, storage_d, storage_e];
                let mut expertiseArray: [i32;5] = [expertise_a, expertise_b, expertise_c, expertise_d, expertise_e];
                
                robots.push(Robot {modTarget, eta,score, storageArray, expertiseArray});


            }
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let available_a = parse_input!(inputs[0], i32);
            let available_b = parse_input!(inputs[1], i32);
            let available_c = parse_input!(inputs[2], i32);
            let available_d = parse_input!(inputs[3], i32);
            let available_e = parse_input!(inputs[4], i32);
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let sample_count = parse_input!(input_line, i32);
            for i in 0..sample_count as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();
                let sample_id = parse_input!(inputs[0], i32);
                let carried_by = parse_input!(inputs[1], i32);
                let rank = parse_input!(inputs[2], i32);
                let expertise_gain = inputs[3].trim().to_string();
                let health = parse_input!(inputs[4], i32);
                let cost_a = parse_input!(inputs[5], i32);
                let cost_b = parse_input!(inputs[6], i32);
                let cost_c = parse_input!(inputs[7], i32);
                let cost_d = parse_input!(inputs[8], i32);
                let cost_e = parse_input!(inputs[9], i32);

                let mut costArray: [i32;5] = [cost_a, cost_b, cost_c, cost_d, cost_e];
                
                thisSample: Sample {sample_id, costArray, health, rank, expertise_gain};
                
                match carried_by{
                    -1 =>{
                        samples.push(thisSample);
                    },
                    0 => {
                        robots[0].samples.push(thisSample);
                    },
                    1 => {
                        robots[1].samples.push(thisSample);
                    }
                }

                let mut myRobot: Robot = robots[0];
                println!("Module : {}", myRobot.target);
                println!("Storage (A B C D E) : {}", myRobot.storage);
                println!("Expert. (A B C D E) : {}", myRobot.expertise);
                let mut potential: [i32,5];
                for i in 0...5 {
                    potential[i] = myRobot.storage[i] + myRobot.expertise[i];
                }

                for_each!(project in projects{
                    println!("Project {} Expertise : {}", projects.IndexOf(project), project.expertise);
                });

                println!("");
                println!("------------- SAMPLES -------------");
                println!("");

                println!("Potential (A B C D E) : {}", potential);
                println!("Available (A B C D E) : {}", available);
                println!("");

                //
                for_each!(sample in myRobot.samples.OrderByDescending(item => item.health){
                    println!("Samp Cost (A B C D E) : {} - Rank {} - Health : {} - Gain : {} - ID : " sample.cost, sample.rank, sample.health, sample.gain, sample.id);
                });

                myRobot.Update();
            }

            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");

            // println!("GOTO DIAGNOSIS");
        }
    }
}