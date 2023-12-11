import os
import json

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
CONFIG_FILE = "docs/config.json"


def get_config(filename: str) -> dict:
    with open(filename, 'r') as f:
        config = json.load(f)

    return config


def generate_progress_bar(config: dict, numerator: int, denominator: int) -> str:
    percent = str(round((numerator / denominator) * 100))
    url = config['progress_bar'].format(
        percent=percent,
    )

    return url


def generate_badge(config: dict, language: str) -> str:
    url = config['shields_io_badge'].format(
        language=language,
        bgcolor=config['languages'][language]['bgcolor'],
        fgcolor=config['languages'][language]['fgcolor'],
    )

    return url


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
    config = get_config(f"{ROOT_DIR}/{CONFIG_FILE}")
    year = config['year']
    puzzles = config['puzzles']
    languages = config['languages']
    title = config['title']
    description = config["description"]
    logo = config['logo']
    logo_width = config['logo_width']

    with open(f'{ROOT_DIR}/README.md', 'w') as readme_file:
        readme_file.write('<p align="center">\n')
        readme_file.write(f'    <img alt="Advent of Code {year} Logo" src="{logo}" width={logo_width} />\n')
        readme_file.write('</p>\n\n')
        readme_file.write(title.format(year=year) + '\n\n')
        readme_file.write(description.format(year=year) + "\n\n")

        progress_bar = generate_progress_bar(config, len(puzzles), 25)

        readme_file.write(f'Completed **{len(puzzles)}** out of **25** advent day puzzles.\n\n')
        readme_file.write(f'![Progress Bar]({progress_bar})\n\n')
        readme_file.write('Day | Puzzle | Solutions\n')
        readme_file.write('--- | --- | ---\n')

        for puzzle in puzzles:
            puzzle_day = puzzle['day']
            puzzle_dir = f'Day0x{puzzle_day:02X}'
            puzzle_title = puzzle['title']
            puzzle_link = f'[{puzzle_title}](https://adventofcode.com/{year}/day/{puzzle_day})'
            file_key = 'file'

            solved_languages = sorted(
                [
                    l for l in os.listdir(f'{ROOT_DIR}/{puzzle_dir}')
                    if l in config['languages']
                ]
            )

            badges = ' '.join(
                [
                    f'[![]({generate_badge(config, l)})]({puzzle_dir}/{l}/{languages[l][file_key]})'
                    for l in solved_languages
                ]
            )

            readme_file.write(f'{puzzle_day} | {puzzle_link} | {badges}\n')
