import os


def generate_readme(base_path):
    readme_content = "# Advent of Code Solutions\n\n"

    for year in reversed(sorted(os.listdir(base_path))):
        year_path = os.path.join(base_path, year)
        if os.path.isdir(year_path) and year.isdigit():
            readme_content += f"## Year {year}\n\n"
            readme_content += "| Day | Languages | Task Link |\n|---|---|---|\n"

            for day in range(1, 26):
                day_path = os.path.join(year_path, f"day-{day}")
                languages = []

                if os.path.exists(day_path) and os.path.isdir(day_path):
                    for lang_dir in os.listdir(day_path):
                        languages.append(f'[{lang_dir}](./{year}/day-{day}/{lang_dir})')

                languages_column = ' <br> '.join(languages) if languages else "❌ Unresolved"
                task_link = f"[Link](https://adventofcode.com/{year}/day/{day})"
                readme_content += f"| {day} | {languages_column} | {task_link} |\n"

            readme_content += "\n"

    with open(os.path.join(base_path, 'README.md'), 'w', encoding='utf-8') as file:
        file.write(readme_content)


base_path = os.path.abspath(os.path.join(os.path.dirname(__file__)))
generate_readme(base_path)
