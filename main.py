import os
import re
from datetime import datetime

def get_last_modified_date(path):
    timestamp = os.path.getmtime(path)
    return datetime.fromtimestamp(timestamp).strftime('%d.%m.%Y')

def generate_readme(base_path):
    readme_content = "# Advent of Code Solutions\n\n"

    for year in sorted(os.listdir(base_path)):
        year_path = os.path.join(base_path, year)
        if os.path.isdir(year_path) and year.isdigit():
            readme_content += f"## Year {year}\n\n"
            readme_content += "| Day | Languages | Task Link |\n|---|---|---|\n"

            for day in range(1, 26):
                day_path = os.path.join(year_path, f"day-{day}")
                languages = []

                if os.path.exists(day_path) and os.path.isdir(day_path):
                    for lang_dir in os.listdir(day_path):
                        lang_path = os.path.join(day_path, lang_dir)
                        match = re.match(r'(\w+)_(\d{2}\.\d{2}\.\d{4})', lang_dir)
                        if match:
                            lang, date = match.groups()
                            date_text = f" ({date})" if date else ""
                        else:
                            lang = lang_dir
                            date_text = ""

                        link_text = f"{lang}{date_text if date_text else f' ({get_last_modified_date(lang_path)})'}"
                        languages.append(f'[{link_text}](./{year}/day-{day}/{lang_dir})')

                languages_column = ' <br> '.join(languages) if languages else "❌ Unresolved"
                task_link = f"[Link](https://adventofcode.com/{year}/day/{day})"
                readme_content += f"| {day} | {languages_column} | {task_link} |\n"

            readme_content += "\n"

    with open(os.path.join(base_path, 'README.md'), 'w', encoding='utf-8') as file:
        file.write(readme_content)

base_path = 'C:/Users/radek/Documents/GitHub/advent-of-code' 
generate_readme(base_path)
