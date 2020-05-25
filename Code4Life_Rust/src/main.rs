use std::io;
//for random generator
//extern crate rand;
use rand::Rng;
// for foreach 
use foreach::Continue::*;
//for max operation
use std::cmp;
//for queue
//extern crate queues;
use queues::*;
use crate::MoleculeType::e;

enum MoleculeType { a, b, c, d, e}
 
trait Module{
     fn get_desicion(robot: Robot) -> ();
     fn sample_researchable(robot: Robot) -> bool{
         let mut can_do: bool = robot.samples_researchable();
         println!("At least one research can be done: {}" , can_do);
        can_do;
     }

     fn sample_doable(robot: Robot) -> bool{
         let mut can_do: bool = robot.samples_doable();
         println!("At least one sample can be done: {}", can_do);
         can_do;
     }
}
 
impl StartPoint for Module {
    fn get_desicion(robot: Robot) -> (){
        robot.go_to("SAMPLES");
    }
}
 
impl Samples for Module {
    fn get_desicion(robot: Robot) -> (){
        let mut rng = rand::thread_rng();
         if robot.expertise.sum() == 0 {
             if robot.samples.len() <2 {
                 robot.connect(1);
                 return;
             }else{
                 robot.go_to("DIAGNOSIS");
                 return;
             }
         }else if robot.samples.len() <3 {
             if robot.score_from_project(robot.closest_project())<1 {
                 robot.connect(rng.gen_range(1, 4));
                 return;
             }
             if robot.expertise.sum() <6 {
                 robot.connect(1);
                 return;
             }else if robot.expertise.sum() <10  {
                 robot.connect(2);
                 return;
             }else{
                 robot.connect(3);
                 return;
             }
         }else{
             robot.go_to("DIAGNOSIS");
             return;
         }  
         return;
    }
}
 
impl Diagnosis for Module {
    fn get_desicion(robot: Robot) -> (){
        let selected_sample: Sample = robot.choose_diagnosticable_sample();
        if selected_sample == None {
            robot.connect(selected_sample.id);
            return;
        }
        else{
            // We put apart samples not realisable
            for_each!(s in robot.Samples{
                if !robot.can_do_sample(s) || (robot.needed_for_sample(s) >6 && s.rank <3) {
                    robot.connect(s.id);
                    return;
                }
            }); 

            // We take researchable samples if they exist in the cloud
            if robot.samples.len()<3 {
                for_each!(s in Player.samples{
                    if robot._can_research_sample(s) && robot.needed_for_sample(s) <3 {
                        robot.connect(s.id);
                        return;
                    }
                });
            }

            // If we're really close to one project, we ditch all samples that do not help to finish it
            if robot.samples.len() >0 && robot.score_from_project(robot.closest_project())<1 {
                for_each!(sample in robot.samples{
                    if !robot.is_close_to_project_end(sample) {
                        robot.connect(sample.id);
                        return;
                    }
                });
            }

            // if at least one sample is researchable goto lab
            if sample_researchable(robot) {
                robot.go_to("LABORATORY");
                return;
            }
            // if at least one sample is realisable goto molecules
            else if sample_doable(&robot) {
                robot.go_to("MOLECULES");
                return;
            }
        }
        robot.go_to("SAMPLES");
        return;
    }
}
 
impl Molecules for Module {
    fn get_desicion(robot: Robot) -> (){
        let mut needed:[i32;5] = [0,0,0,0,0];
        //
        if robot.storage.sum() <10 {
            let index: i32 = 0;
            //

            for_each!(sample in robot.samples.sort_by_key(|s| s.rank){
               if index != 0 {
                   //
                   let mut previous_sample: Sample = robot.samples.sort_by_key(|s| s.rank)[index-1];
                   if robot.can_do_sample(previous_sample) {
                       needed[previous_sample.gain.parse::<i32>()unwrap()] -=1;
                   }
               } 
               if robot.can_collect_molecules(sample) {
                   for i in 0..5 as usize {
                       needed[i] += cmp::max(sample.cost[i] - robot.expertise[i], 0);
                       if needed.sum() > 10 {
                           //
                           if robot.samples.sort_by_key(|s| s.rank).iter().position(|&r| r == sample).unwrap() !=0 {
                                needed = [0,0,0,0,0];
                           }
                           println!("First break");
                           break;
                       }    
                   }
                   println!("needed: {}", needed);
                   if needed.sum() > 10 {
                       //
                       if robot.samples.sort_by_key(|s| s.rank).iter().position(|&r| r == sample).unwrap() != 0 {
                            needed = [0,0,0,0,0];
                       }
                       println!("Second break");
                       break;
                   } 
                   if !robot.can_research_sample(sample) {
                       println!("Cannot research sample");
                       if robot.can_do_sample(sample) && needed.sum() <=10 {
                           println!("Beginning molecule collection");
                           for i in 0...5 as usize{
                               //
                               println!("Collection module {} ??", (MoleculeType)i) ;
                               if needed[i] - robot.storage[i] > 0 && Player.available[i] >0 {
                                   println!(" : Yes" );
                                   //
                                   robot.connect((MoleculeType)i);
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
            if sample_researchable(robot) || needed.sum() > 10 {
              println!("needed: {}", needed );  
              println!("Can research sample, goto lab");
              robot.go_to("LABORATORY");
              return;
            }else{
                println!("needed: {}",needed );
                println!("Cannot research sample, goto diag");
                robot.go_to("DOAGNOSIS");
                return;
            }
            return;
        }
        else{
            if !sample_researchable(robot) {
                println("+10 needed: {}", needed);
                println!("Cannot research sample, goto diag" );
                robot.go_to("DIAGNOSIS");
                return;
            }else{
                println("+10 needed: {}", needed);
                println!("Can research sample, goto lab" );
                robot.go_to("LABORATORY");
                return;
            }
        }
        return;
    }
}

impl Laboratory for Module {
    fn get_desicion(robot: Robot) -> (){
        if sample_researchable(robot) {
            for_each!(sample in robot.samples{
                if robot.can_research_sample(sample) {
                    robot.connect(sample.id);
                    return;
                }
            });
        }
        //
        if !sample_doable(&robot) || robot.samples.len() <2 {
            //
            if robot.samples.len() <3 {
                robot.go_to("SAMPLES");
                return;
            }else{
                robot.go_to("DIAGNOSIS");
                return;
            }
        }else {
            robot.go_to("MOLECULES");
            return;
        }
        return;
    }
}

struct Project{
    expertise: [i32;256],
}

struct Sample{
    id :i32,
    cost: [i32],
    health: i32,
    rank: i32,
    gain: MoleculeType,
    diagnosticated: bool,
}

struct Molecule {
    moltype : MoleculeType,
}

struct Robot{
    samples: Vec::new(),
    target: Module,
    eta: i32, 
    score: i32,
    //
    storage: [i32],
    expertise: [i32],
    toBeResearched: Queue<isize>, //= queue![],
}

impl Robot{
    fn update() ->(){
        target.get_desicion();
    }

    fn can_collect_molecules(sample: Sample) -> bool{
        let mut nb_needed: i32 = 0;
        let mut can_do:bool = true;

        for i in 0..5 as usize{
            nb_needed += cmp::max(0, sample.cost[i] - storage[i] - expertise[i]);
        }
        //
        if nb_needed > (10 - storage.sum()) {
            can_do = false;
        }
        can_do;
    }

    fn samples_doable() -> bool{
        for_each!(sample in samples{
            if can_do_sample(sample) {
                true;
            }
        });
        false;
    }

    fn samples_researchable() -> bool{
        for_each!(sample in samples{
            if can_research_sample(sample) {
                true;
            }
        });
        false;
    }

    fn is_sample_good_for_projects(sample: Sample) -> bool{
        let mut is_good:bool = true;
        //
        if expertise.sum() >= 6 {
            let mut goals:[i32;5] = [0,0,0,0,0];
            for_each!(project in Player.projects{
                for i in 0..5 as usize{
                    goals[i] = cmp::max(project.expertise[i], goals[i]);
                }
            });

            let mut gain_molecule:i32 = sample.gain.parse::<i32>().unwrap();
            if goals[gain_molecule] <= expertise[gain_molecule] {
                is_good = false;
            }
        }
        is_good;
    }

    fn score_from_project(project: Project) ->i32{
        let mut score:i32 = 0;
        for i in 0..5 as usize{
            score += cmp::max(project.expertise[i] - expertise[i],0);
        }
        score;
    }

    fn is_close_to_project_end(sample: Sample) -> bool{
        let mut closest_project: Project = closest_project();

        if can_do_sample(sample) {
            let mut molecule: i32 = sample.gain.parse::<i32>().unwrap();
            if closest_project.expertise[molecule] > expertise[molecule] && score_from_project(closest_project) <32 {
                true;
            }
        }
        false;
    }

    fn needed_for_sample(sample: Sample) -> i32{
        let mut needed:[i32;5] = [0,0,0,0,0];
        for i in 0..5 as usize{
            needed[i] = cmp::max(sample.cost[i] -expertise[i] - storage[i],0);
        }
        needed.Sum();
    }

    fn closest_project() -> Project {
        let mut min_score: i32 = 20;
        let mut c_project = Player.projects.First();

        for_each!(p in Player.Projects{
            let mut temp_score:i32 = score_from_project(p);
            if temp_score < min_score && temp_score > 0 {
                min_score = temp_score;
                c_project = p;
            }
        });
        println!("Project vise: {}, ecart: {}",Player.projects.iter().position(|&r| r == cProject).unwrap(),  min_score);
        c_project;
    }

    fn choose_diagnosticable_sample() -> Sample {
         let mut temp_samples = Vec::new();
        temp_samples = samples.find(|&&x| diagnosticated== false);
        temp_samples.sort_by_key(|x| x.health)[0];
    }

    fn can_research_sample(sample: Sample) -> bool {
        let mut can_do:bool = true;

        for i in 0..3 as usize{
            if storage[i] + expertise[i] < sample.cost[i] {
                can_do = false;
            }
        }
        println!("Sample {} can be researched {}", samples.iter().position(|&r| r ==sample).unwrap(), can_do);
        can_do;
    }

    fn can_do_sample(sample: Sample) -> bool{
        let mut can_do:bool = true;
        if !can_research_sample(sample) && storage.sum() >=10 || is_sample_good_for_projects(&sample) {
            can_do = false;
        }else{
            for i in 0..5 as usize{
                if Player.available[i] + storage[i] + expertise[i] < sample.cost[i] {
                    can_do = false;
                    break;
                }
            }
        }
        print!("Sample {} can be done {}", samples(&sample), can_do);
        return can_do;
    }

    fn go_to(module: String) ->(){
        println!("GOTO {}", module);
    }

    fn connect(id:i32) ->(){
        println!("CONNECT {}", id);
    }

    fn connect_mol(moltype: MoleculeType) -> (){
        println!("CONNECT {}", moltype );
    }

    fn wait() -> (){
        println!("WAIT");
    }

}

struct Player {
    samplesTaken: i32,
    available: [i32;256],
    projects: Vec::new(),
    robots: Vec::new(),
    samples: Vec::new(),

}
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

impl Player{
    /**
    * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
    **/
    fn main() {
        let mut first_turn:bool = true;

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
            let ee = parse_input!(inputs[4], i32);

            let mut project: [i32;5] = [a,b,c,d,ee];
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

                let mut mod_target:Module = None;
                match target{
                    "START_POS" =>{
                        mod_target = StartPoint();
                    },
                    "SAMPLES" =>{
                        mod_target = Samples();
                    },
                    "DIAGNOSIS" =>{
                        mod_target = Diagnosis();
                    },
                    "MOLECULES" => {
                        mod_target = Molecules();
                    },
                    "LABORATORY" =>{
                        mod_target = Laboratory();
                    },
                    _ => {
                    }
                }

                let mut storage_array: [i32;5] = [storage_a, storage_b, storage_c, storage_d, storage_e];
                let mut expertise_array: [i32;5] = [expertise_a, expertise_b, expertise_c, expertise_d, expertise_e];

                let rbt = Robot(mod_target, eta,score, storage_array, expertise_array);
                robots.push(rbt);


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

                let mut cost_array: [i32;5] = [cost_a, cost_b, cost_c, cost_d, cost_e];

                let this_sample = Sample(sample_id, cost_array, health, rank, expertise_gain);
                
                match carried_by{
                    -1 =>{
                        samples.push(this_sample);
                    },
                    0 => {
                        robots[0].samples.push(this_sample);
                    },
                    1 => {
                        robots[1].samples.push(this_sample);
                    }
                }

                let mut my_robot: Robot = robots[0];
                println!("Module : {}", my_robot.target);
                println!("Storage (a b c d e) : {}", my_robot.storage);
                println!("Expert. (a b c d e) : {}", my_robot.expertise);
                let mut potential: [i32;5] = [0,0,0,0,0];
                for i in 0..5 as usize{
                    potential[i] = my_robot.storage[i] + my_robot.expertise[i];
                }

                for_each!(project in projects{
                    println!("Project {} Expertise : {}", projects.iter().position(|&r| r == project).unwrap(), project.expertise);
                });

                println!("\n------------- SAMPLES -------------\n");

                println!("Potential (a b c d e) : {}", potential);
                println!("Available (a b c d e) : {}", available);

                //
                for_each!(sample in myRobot.samples.sort_by_key(|x| -x.health){
                    println!("Samp Cost (a b c d e) : {} - Rank {} - Health : {} - Gain : {} - ID : " sample.cost, sample.rank, sample.health, sample.gain, sample.id);
                });

                my_robot.Update();
            }

            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");

            // println!("GOTO DIAGNOSIS");
        }
    }
}