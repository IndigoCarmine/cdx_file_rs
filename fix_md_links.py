"""
Fix links in converted Markdown files to point to .md instead of .htm
"""

import re
from pathlib import Path


def fix_links_in_file(file_path):
    """Fix all .htm and .html links to .md in a markdown file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Replace .htm) and .html) with .md)
    original_content = content
    content = re.sub(r'\.htm\)', '.md)', content)
    content = re.sub(r'\.html\)', '.md)', content)
    
    # Only write if changes were made
    if content != original_content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False


def main():
    """Fix links in all markdown files."""
    md_dir = Path('new_doc/md')
    
    # Get all markdown files recursively
    md_files = list(md_dir.glob('**/*.md'))
    
    print(f"Found {len(md_files)} Markdown files to process.\n")
    
    fixed = 0
    for md_file in sorted(md_files):
        if fix_links_in_file(md_file):
            print(f"Fixed links in {md_file.relative_to(md_dir.parent)}")
            fixed += 1
    
    print(f"\nComplete! Fixed links in {fixed} files.")


if __name__ == '__main__':
    main()
