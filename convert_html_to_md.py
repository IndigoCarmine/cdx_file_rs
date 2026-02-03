"""
HTML to Markdown converter for CDX specification files.
Converts all HTML files in new_doc/html to Markdown in new_doc/md.
"""

import os
import re
from pathlib import Path
from html.parser import HTMLParser
from html import unescape


class HTMLToMarkdown(HTMLParser):
    """Custom HTML to Markdown converter optimized for CDX specification files."""
    
    def __init__(self):
        super().__init__()
        self.markdown = []
        self.current_tag = []
        self.list_level = 0
        self.in_table = False
        self.table_rows = []
        self.current_cell_content = []
        self.in_cell = False
        self.in_pre = False
        self.in_code = False
        self.skip_content = False
        self.current_link_href = None
        self.link_text = []
        self.pending_bold = False
        self.pending_italic = False
        
    def handle_starttag(self, tag, attrs):
        attrs_dict = dict(attrs)
        
        # Skip script, style, and wayback machine elements
        if tag in ['script', 'style'] or 'wayback' in str(attrs).lower():
            self.skip_content = True
            return
        
        # Skip link tags with xxhref (invalid links)
        if tag == 'link' and 'xxhref' in attrs_dict:
            return
            
        self.current_tag.append(tag)
        
        if tag == 'h1':
            self.markdown.append('\n# ')
        elif tag == 'h2':
            self.markdown.append('\n## ')
        elif tag == 'h3':
            self.markdown.append('\n### ')
        elif tag == 'h4':
            self.markdown.append('\n#### ')
        elif tag == 'h5':
            self.markdown.append('\n##### ')
        elif tag == 'h6':
            self.markdown.append('\n###### ')
        elif tag == 'p':
            if not self.in_table:
                self.markdown.append('\n\n')
        elif tag == 'br':
            if self.in_cell:
                self.current_cell_content.append(' ')
            else:
                self.markdown.append('  \n')
        elif tag == 'hr':
            self.markdown.append('\n\n---\n\n')
        elif tag == 'b' or tag == 'strong':
            if self.in_cell:
                self.pending_bold = True
            else:
                self.markdown.append('**')
        elif tag == 'i' or tag == 'em':
            if self.in_cell:
                self.pending_italic = True
            else:
                self.markdown.append('*')
        elif tag == 'code':
            self.in_code = True
            if not self.in_cell:
                self.markdown.append('`')
        elif tag == 'pre':
            self.in_pre = True
            self.markdown.append('\n```\n')
        elif tag == 'a':
            href = attrs_dict.get('href', '')
            # Skip archive.org links, invalid links (xhref), and CSS references
            if (href and 
                not href.startswith(('http://web.archive.org', 'https://web.archive.org', 'css/', 'CDX%20')) and 
                'xhref' not in attrs_dict and
                not href.endswith('.css')):
                self.current_link_href = href
                self.link_text = []
        elif tag == 'ul' or tag == 'ol':
            self.list_level += 1
            if not self.in_table:
                self.markdown.append('\n')
        elif tag == 'li':
            indent = '  ' * (self.list_level - 1)
            if self.current_tag[-2] == 'ol' if len(self.current_tag) > 1 else False:
                prefix = f'{indent}1. '
            else:
                prefix = f'{indent}- '
            if self.in_cell:
                self.current_cell_content.append(prefix)
            else:
                self.markdown.append(prefix)
        elif tag == 'table':
            self.in_table = True
            self.table_rows = []
        elif tag == 'tr':
            pass  # Start a new row
        elif tag in ['td', 'th']:
            self.in_cell = True
            self.current_cell_content = []
        elif tag == 'blockquote':
            self.markdown.append('\n> ')
        elif tag == 'font':
            # Skip font tags - they're presentational
            pass
            
    def handle_endtag(self, tag):
        if tag in ['script', 'style']:
            self.skip_content = False
            return
            
        if self.skip_content:
            return
            
        if tag == 'link':
            return
            
        if not self.current_tag:
            return
            
        if self.current_tag and self.current_tag[-1] == tag:
            self.current_tag.pop()
        
        if tag in ['h1', 'h2', 'h3', 'h4', 'h5', 'h6']:
            self.markdown.append('\n')
        elif tag == 'p':
            if not self.in_table:
                self.markdown.append('\n')
        elif tag == 'b' or tag == 'strong':
            if self.in_cell:
                self.pending_bold = False
            else:
                self.markdown.append('**')
        elif tag == 'i' or tag == 'em':
            if self.in_cell:
                self.pending_italic = False
            else:
                self.markdown.append('*')
        elif tag == 'code':
            self.in_code = False
            if not self.in_cell:
                self.markdown.append('`')
        elif tag == 'pre':
            self.in_pre = False
            self.markdown.append('\n```\n')
        elif tag == 'a':
            # If we have a link in progress, complete it
            if self.current_link_href:
                link_text = ''.join(self.link_text).strip()
                if link_text:
                    if self.in_cell:
                        self.current_cell_content.append(f'[{link_text}]({self.current_link_href})')
                    else:
                        self.markdown.append(f'[{link_text}]({self.current_link_href})')
                self.current_link_href = None
                self.link_text = []
        elif tag in ['ul', 'ol']:
            self.list_level -= 1
            if not self.in_table:
                self.markdown.append('\n')
        elif tag == 'li':
            if self.in_cell:
                pass
            else:
                self.markdown.append('\n')
        elif tag in ['td', 'th']:
            # End of cell - add content to current row
            cell_text = ''.join(self.current_cell_content).strip()
            if not self.table_rows:
                self.table_rows = [[]]
            self.table_rows[-1].append(cell_text)
            self.current_cell_content = []
            self.in_cell = False
            self.pending_bold = False
            self.pending_italic = False
        elif tag == 'tr':
            # End of row - start a new one if we're still in the table
            if self.in_table:
                self.table_rows.append([])
        elif tag == 'table':
            self.in_table = False
            self._flush_table()
        elif tag == 'font':
            # Skip font tags
            pass
            
    def handle_data(self, data):
        if self.skip_content:
            return
            
        # Clean up the data
        if not self.in_pre and not self.in_code:
            # Normalize whitespace but preserve intentional line breaks
            data = re.sub(r'[ \t]+', ' ', data)
            if not data.strip() and not self.in_cell:
                return
                
        if self.in_cell:
            # Collecting cell content
            if self.current_link_href is not None:
                self.link_text.append(unescape(data))
            else:
                self.current_cell_content.append(unescape(data.strip()))
        elif self.current_link_href is not None:
            # Collecting link text
            self.link_text.append(unescape(data))
        else:
            self.markdown.append(unescape(data))
            
    def _flush_table(self):
        """Convert collected table rows to Markdown table."""
        # Remove empty rows
        self.table_rows = [row for row in self.table_rows if row and any(cell.strip() for cell in row)]
        
        if not self.table_rows:
            return
        
        # Find maximum number of columns
        max_cols = max(len(row) for row in self.table_rows)
        
        # Pad rows to have the same number of columns
        for row in self.table_rows:
            while len(row) < max_cols:
                row.append('')
        
        self.markdown.append('\n\n')
        
        for i, row in enumerate(self.table_rows):
            self.markdown.append('| ')
            self.markdown.append(' | '.join(row))
            self.markdown.append(' |\n')
            
            # Add header separator after first row
            if i == 0:
                self.markdown.append('| ')
                self.markdown.append(' | '.join(['---'] * len(row)))
                self.markdown.append(' |\n')
                
        self.markdown.append('\n')
        
    def get_markdown(self):
        """Return the converted Markdown text."""
        text = ''.join(self.markdown)
        # Clean up multiple blank lines
        text = re.sub(r'\n{3,}', '\n\n', text)
        return text.strip()


def convert_html_file(html_path, md_path):
    """Convert a single HTML file to Markdown."""
    print(f"Converting {html_path.name}...")
    
    try:
        with open(html_path, 'r', encoding='utf-8') as f:
            html_content = f.read()
    except UnicodeDecodeError:
        # Try with windows-1252 encoding
        with open(html_path, 'r', encoding='windows-1252') as f:
            html_content = f.read()
    
    # Remove Wayback Machine toolbar (everything before the actual content)
    html_content = re.sub(r'<!-- BEGIN WAYBACK TOOLBAR INSERT -->.*?<!-- END WAYBACK TOOLBAR INSERT -->', '', html_content, flags=re.DOTALL)
    
    # Remove other Wayback Machine elements
    html_content = re.sub(r'<div id="wm-ipp-base".*?</div>\s*(?=<script|<body|<!--)', '', html_content, flags=re.DOTALL)
    html_content = re.sub(r'<script[^>]*archive\.org[^>]*>.*?</script>', '', html_content, flags=re.DOTALL)
    html_content = re.sub(r'<script[^>]*wayback[^>]*>.*?</script>', '', html_content, flags=re.DOTALL | re.IGNORECASE)
    
    # Remove HTML comments
    html_content = re.sub(r'<!--.*?-->', '', html_content, flags=re.DOTALL)
    
    # Remove the Wayback Machine print div
    html_content = re.sub(r'<div id="wm-ipp-print">.*?</div>', '', html_content, flags=re.DOTALL)
    
    converter = HTMLToMarkdown()
    converter.feed(html_content)
    markdown_content = converter.get_markdown()
    
    # Write to markdown file
    with open(md_path, 'w', encoding='utf-8') as f:
        f.write(markdown_content)
        
    print(f"  → {md_path.name}")


def main():
    """Main conversion function."""
    html_dir = Path('new_doc/html')
    md_dir = Path('new_doc/md')
    
    # Create md directory if it doesn't exist
    md_dir.mkdir(parents=True, exist_ok=True)
    
    # Get all HTML files (excluding subdirectories for now)
    html_files = list(html_dir.glob('*.htm')) + list(html_dir.glob('*.html'))
    
    print(f"Found {len(html_files)} HTML files to convert.\n")
    
    converted = 0
    for html_file in sorted(html_files):
        md_file = md_dir / (html_file.stem + '.md')
        try:
            convert_html_file(html_file, md_file)
            converted += 1
        except Exception as e:
            print(f"Error converting {html_file.name}: {e}")
            
    # Also convert subdirectories
    for subdir in ['properties', 'DataType']:
        subdir_path = html_dir / subdir
        if subdir_path.exists():
            md_subdir = md_dir / subdir
            md_subdir.mkdir(parents=True, exist_ok=True)
            
            sub_html_files = list(subdir_path.glob('*.htm')) + list(subdir_path.glob('*.html'))
            print(f"\nConverting {subdir}/ ({len(sub_html_files)} files)...")
            
            for html_file in sorted(sub_html_files):
                md_file = md_subdir / (html_file.stem + '.md')
                try:
                    convert_html_file(html_file, md_file)
                    converted += 1
                except Exception as e:
                    print(f"Error converting {html_file.name}: {e}")
    
    print(f"\nConversion complete! {converted} files converted to Markdown.")
    print(f"Output directory: {md_dir.absolute()}")


if __name__ == '__main__':
    main()
