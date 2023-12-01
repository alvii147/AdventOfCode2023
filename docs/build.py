import os


ADVENT_YEAR = 2023
SHIELDS_IO_BADGE_URL = 'https://img.shields.io/badge'
PROGRESS_BAR_URL = 'https://progress-bar.dev'
LANGUAGE_LOGOS = {
    'go': 'go-00ADD8?style=for-the-badge&logo=go&logoColor=FFFFFF',
    'python': 'python-3670A0?style=for-the-badge&logo=python&logoColor=FFDD54',
    'rust': 'rust-000000?style=for-the-badge&logo=rust&logoColor=FFFFFF',
}
LANGUAGE_FILENAMES = {
    'go': 'main.go',
    'python': 'main.py',
    'rust': 'src/main.rs',
}


def generate_progress_bar(numerator, denominator):
    progress_bar = f'{PROGRESS_BAR_URL}/{str(round((numerator / denominator) * 100))}'

    return progress_bar


def get_puzzles_info(dirnames):
    puzzles_info = {}

    sorted_puzzles = [i.split('_') + [i] for i in dirnames]
    sorted_puzzles = [[int(i[1]), ' '.join(i[2:-1]), i[-1]] for i in sorted_puzzles if len(i) >= 3 and i[0].lower() == 'day']
    sorted_puzzles = sorted(sorted_puzzles, key=lambda x: x[0])

    for puzzle in sorted_puzzles:
        day_num, puzzle_name, dirname = puzzle

        puzzles_info[day_num] = {
            'name': puzzle_name,
            'dirname': dirname,
        }

    return puzzles_info


if __name__ == '__main__':
    with open('../README.md', 'w') as readme_file:
        readme_file.write('<p align="center">\n')
        readme_file.write(f'<img alt="Advent of Code {ADVENT_YEAR} Logo" src="docs/img/logo.png" width=600 />\n')
        readme_file.write('</p>\n\n')
        readme_file.write(f'# Advent of Code {ADVENT_YEAR}\n\n')
        readme_file.write(
            '[Advent of Code](https://adventofcode.com) is an Advent calendar of small programming puzzles '
            'for a variety of skill sets and skill levels that can be solved in any programming language you like. '
            f'This repository contains solutions to the {ADVENT_YEAR} Advent of Code calendar.\n\n'
        )

        puzzles_info = get_puzzles_info(os.listdir('../'))
        progress_bar = generate_progress_bar(len(puzzles_info), 25)

        readme_file.write(f'Completed **{len(puzzles_info)}** out of **25** advent day puzzles.\n\n')
        readme_file.write(f'![Progress Bar]({progress_bar})\n\n')
        readme_file.write('Day | Puzzle | Solutions\n')
        readme_file.write('--- | --- | ---\n')

        for day_num, puzzle_info in puzzles_info.items():
            puzzle_name, dirname = puzzle_info['name'], puzzle_info['dirname']

            puzzle_link = f'[{puzzle_name}](https://adventofcode.com/{ADVENT_YEAR}/day/{day_num})'
            languages = sorted([lang for lang in os.listdir(f'../{dirname}') if lang in LANGUAGE_LOGOS])
            language_badges = ' '.join([f'[![]({SHIELDS_IO_BADGE_URL}/{LANGUAGE_LOGOS[lang]})]({dirname}/{lang}/{LANGUAGE_FILENAMES[lang]})' for lang in languages])

            readme_file.write(f'{day_num} | {puzzle_link} | {language_badges}\n')
