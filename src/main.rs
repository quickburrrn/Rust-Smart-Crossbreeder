use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands{
    /// Adds a seed to the program. --seed "seed"
    Add{
        #[arg(short, long)]
        seed: String,
    },
    /// Removes a seed from the program. --seed "seed"
    Remove{
        #[arg(short, long)]
        seed: String,
    },
    /// shows all your seed
    Show{

    },
    /// Reomves all the seeds
    Clear{

    },
    /// You can crossbreed manualy usese seed number for instance --seeds "0,4,7,4"
    Crossbreed{
        #[arg(short, long)]
        seeds: String,
    },
    /// Finds the best combination for you
    Find{
    
    }
}

fn main() {
    //contains all the loaded seeds
    let mut loaded_seeds : Vec<[String; 6]> = Vec::new();

    match load_seeds() {
        Ok(output) => {
            loaded_seeds = output;
        }        
        Err (err) =>{
            eprintln!("Error loading seeds: {}", err)
        }
    }


    let cli = Cli::parse();

    //runs any function
    match &cli.command {
        //ads seed and saves
        Some(Commands::Add { seed }) => {
            if seed != "" {
                match verify_seeds(seed.to_string()) {
                    Some(s) => {
                        loaded_seeds.push(s);
                        match save_seeds(loaded_seeds) {
                            Ok(_) => println!("Added seed"),
                            Err(_) => println!("error saving seeds")
                        }
                    },
                    
                    None => eprintln!("invalid seed")
                }
            } else {
                println!("No seed");
            }
        },
        
        Some(Commands::Remove { seed }) => {
            if seed != "" {
                println!("Not done");
            } else {
                println!("Unvalid seed");
            }
        },

        Some(Commands::Show {  })=>{
            for (i, seed) in loaded_seeds.into_iter().enumerate(){
                print!("{i} ");
                for gene in seed{
                    print!("{gene}")
                }
                println!();
            }
        },

        Some(Commands::Clear {  }) => {
            loaded_seeds = Vec::new();
            match save_seeds(loaded_seeds) {
                Ok(_) => println!("Cleared seeds"),
                Err(_) => println!("error saving seeds")
            }
        }

        Some(Commands::Crossbreed { seeds }) => {
            //gets the seeds
            // let mut crossbreed_seeds : Vec<[&str; 6]>; 

            //need to put the result in a list and transpose before calculating
            for seed in seeds.split(','){
                let index : usize = seed.parse().unwrap();
                println!("{:?}", &loaded_seeds[index]);
                // crossbreed_seeds.push()
            }
        }

        Some(Commands::Find { }) => {
            println!("Finds the best combinations")
        }

        None => {println!("not a valid command. type help for help")}
    }

    
    
    //valides and converts to seed
    fn verify_seeds(seed: String) -> Option<[String; 6]>{
        let count : usize = seed.chars().count();
        //validates the seed
        if count != 6{
            println!("{seed} is invalid. You need 6 characters, yours is {count}");
            return None;
        }
        let s : &str = &seed.to_lowercase();
        for g in s.chars(){
            match g{
                'y' => {},
                'g' => {},
                'h' => {},
                'x' => {},
                'w' => {},
                _ => {println!("can't add {seed} because it contains unknown gene. Your seed can only contain y,g,h,x or ,w");
            return None;}
            }
        }
        
        //ads the seed
        let mut result : [String; 6] = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
        
        //ONLY ADDING 5 NOT 6
        for (i, field) in seed.chars().take(6).enumerate(){
            result[i] = field.to_string();
        }
        return Some(result);
    }


    //reloads the seeds
    fn load_seeds() -> Result<Vec<[String; 6]>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path("./seeds.csv")?;
        let mut output = Vec::new();


        // converting the csv read to string
        for result in rdr.records() {

            let record = result?;

            let mut record_array: [String; 6] = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
            
            for (i, field) in record.iter().take(6).enumerate() 
            {
                record_array[i] = field.to_string();
            }   

            output.push(record_array)
                  
        }

        Ok(output)
        
    }

    
    //saves the seeds
    fn save_seeds(seeds : Vec<[String; 6]>) -> Result<(), Box<dyn std::error::Error>>{
        let mut data : String = "gene1,gene2,gene3,gene4,gene5,gene6 \n".to_string();
        for seed in seeds{
            data.push_str(&seed.join(","));
            data.push_str("\n")
        }
        fs::write("./seeds.csv", data)?;
        
        Ok(())
    }



    


    //tenger nå funksjon til å adde og fjenre seeds

    // let seed1 : [&str; 6] = ["x", "g", "w", "g", "h" , "g"];
    // let seed2 : [&str; 6] = ["x", "y", "g", "g", "h" , "g"];

    // let input : [[&str; 6]; 2] = [
    //     seed1,
    //     seed2
    // ];

    // fn transpose_matrix(matrix : [[&str; 6]; 2]) -> [[&str; 2]; 6]
    // {
    //     let mut transposed = [[""; 2]; 6];

    //     for i in 0..2 {
    //         for j in 0..6 {
    //             transposed[j][i] = &matrix[i][j];
    //         }
    //     }

    //     transposed
    // }
    
    // let output : [[&str; 2]; 6] = transpose_matrix(input);

    // fn find_dominant_gene(row : &[&str]) -> String
    // {
    //     //                          g    y    h    x    w
    //     let mut genes : [f32; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];

    //     for gene in row{
    //         match gene{
    //             &"g" => genes[0] +=0.6,
    //             &"y" => genes[1] += 0.6,
    //             &"h" => genes[2] += 0.6,
    //             &"x" => genes[3] += 1.0,
    //             &"w" => genes[4] += 1.0,
    //             &_=>println!("thats not a gene")
    //         }
    //     }

    //     if let Some((index, _)) = genes.iter().enumerate().max_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap()){
    //         return match index
    //         {
    //             0 => "g".to_string(),
    //             1 => "y".to_string(),
    //             2 => "h".to_string(),
    //             3 => "x".to_string(),
    //             4 => "w".to_string(),
    //             _=>"error".to_string()
    //         };
    //     };

    //     "error".to_string()
    // }


    // fn calculate_crossbreed(transposed_seeds : [[&str; 2]; 6]) -> [String; 6]
    // {
    //     let mut result : [String; 6] = ["".to_string(), "".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string()];

    //     for (index, _seed) in transposed_seeds.iter().enumerate() {
    //         result[index] = find_dominant_gene(&transposed_seeds[index])
    //     }

    //     result
    // }
    
    
}