use rand::Rng;
use std::io;

fn main() {
    println!(" ");
    println!("Welcome to my number guessing game!");
    println!(" ");
    println!("The random number can be anywhere from -128 to 127");
    println!(" ");
//     keep_playing = 'yes'
    let mut keep_playing = String::from("yes");
// while keep_playing =='yes':
    while keep_playing.trim().eq("yes"){
//     magic_number = random.randint(1,100)
        let mut magic_number: i8 =rand::thread_rng().gen_range(-128..127); 
//     guess_count = 0
        let mut guess_count = 0;
        // println!("{}", magic_number);
//     while guess !=magic_number:
        loop {
//     guess = -1
        let mut guess = String::new();
//         guess = int(input('What is your guess? '))
            println!("What is your guess");
            io::stdin().read_line(&mut guess);
//         guess_count = guess_count + 1
            guess_count = guess_count + 1;
//         if guess<magic_number:
            let guess: i8 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
            if guess < magic_number{
//             print('Higher')
                println!("Higher");
                println!(" ");
            }
//         elif guess > magic_number:
//             print('Lower')
            else if guess > magic_number{
                println!("Lower");
                println!(" ");
            }
//         else:
//             print('You guessed it!')   
            else{
                println!("You guessed it!");
                println!(" ");
                // println!("Would you like to play again (yes/no");
                // io::stdin().read_line(&mut keep_playing);
                break;
            };
        }
//     print(f'it took you{guess_count} guesses')
        println!("it took you {} guesses", guess_count);
        println!(" ");
//     keep_playing = input('Would you like to play again (yes/no)? ')
        // let keep_playing_bool: bool = true;
        // if keep_playing_bool == true{
        //     println!("Would you like to play again (yes/no");
        //     io::stdin().read_line(&mut keep_playing);
        //     keep_playing = yes;
        // }
        keep_playing.clear();
        println!("Would you like to play again (yes/no)");
        io::stdin().read_line(&mut keep_playing);
        println!(" ");
        // let keep_playing: i8 = match keep_playing.trim().parse();
        // keep_playing = keep_playing.trim();
        // println!("{}", keep_playing == "yes\n");
        // println!("[{}]", keep_playing.trim());
        // break;
    }
// print('Thank you for playing. Goodbye.')
println!(" ");
println!("Thank you for playing. Have a great day.");
println!(" ");
}
