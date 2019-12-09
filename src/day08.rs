use crate::utils;

pub fn solve() {
    let lines = utils::lines_from_file("input_8.1.txt");
    let mut input: Vec<String> = lines[0]
        .split("")
        .map(|s| s.to_string() )
        .collect();

    input.remove(0);
    input.remove(input.len() - 1);

    let data: Vec<i32> = input
        .iter()
        .map(|s|
            s.parse::<i32>()
                .unwrap()
        )
        .collect();

    let wide = 25;
    let tall = 6;

    let sum = wide * tall;
    let num_of_layers = data.len() / sum;

    let mut space_images: Vec<SpaceImage> = Vec::new();
    for i in 0..num_of_layers {
        let start_index = i * sum;
        let end_index = start_index + sum;
        let subdata: Vec<i32> = data[start_index..end_index]
            .iter()
            .cloned()
            .collect();
        let space_image = SpaceImage::new(i, subdata);
        space_images.push(space_image);
    }

    part1(space_images.clone());
    part2(space_images.clone(), tall, wide, num_of_layers);
}

fn part1(space_images: Vec<SpaceImage>) {
    let mut min_count = 256;
    let mut layer_index = 0;
    for image in space_images.iter() {
        let occurences = image.find_num_of_occurrences(0);
        if min_count > occurences {
            min_count = occurences;
            layer_index = image.layer_index;
        }
    }

    let image = &space_images[layer_index];
    let ones = image.find_num_of_occurrences(1);
    let twos = image.find_num_of_occurrences(2);
    println!("Day 8/Part 1: {}", ones * twos);
}

fn part2 (space_images: Vec<SpaceImage>, tall: usize, wide: usize, num_of_layers: usize) {
    let sum = wide * tall;
    let mut final_image: Vec<i32> = Vec::new();
    for i in 0..sum {
        for j in 0..num_of_layers {
            if space_images[j].data[i] == 2 {
                continue
            }
            final_image.push(space_images[j].data[i]);
            break;
        }
    }

    println!("Day 8/Part 2:");
    for i in 0..tall {
        let start_index = i * wide;
        let end_index = start_index + wide;
        println!("{:?}", &final_image[start_index..end_index]);
    }
}

#[derive(Clone)]
struct SpaceImage {
    layer_index: usize,
    data: Vec<i32>,
}

impl SpaceImage {
    fn new(layer_index: usize, data: Vec<i32>) -> SpaceImage {
        SpaceImage{
            layer_index,
            data
        }
    }

    fn find_num_of_occurrences(&self, value: i32) -> i32 {
        // Find number of occurrences
        self.data
            .iter()
            .filter(|&n|
                *n == value
            )
            .count() as i32
    }
}