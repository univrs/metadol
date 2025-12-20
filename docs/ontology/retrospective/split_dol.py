#!/usr/bin/env python3
"""Split multi-declaration DOL files into individual files."""

import re
import os
from pathlib import Path

def extract_balanced_braces(content: str, start_pos: int) -> tuple[str, int]:
    """Extract content between balanced braces starting at start_pos."""
    if content[start_pos] != '{':
        raise ValueError(f"Expected '{{' at position {start_pos}")

    depth = 0
    end_pos = start_pos

    for i in range(start_pos, len(content)):
        if content[i] == '{':
            depth += 1
        elif content[i] == '}':
            depth -= 1
            if depth == 0:
                end_pos = i
                break

    return content[start_pos+1:end_pos], end_pos

def fix_body_syntax(body: str, decl_type: str) -> str:
    """Fix common DOL syntax issues in declaration bodies."""
    lines = body.split('\n')
    fixed_lines = []

    for line in lines:
        stripped = line.strip()
        indent = len(line) - len(line.lstrip())
        prefix = ' ' * indent

        if not stripped or stripped.startswith('//'):
            fixed_lines.append(line)
            continue

        # Fix 'derive from' -> 'derives from'
        stripped = re.sub(r'\bderive\s+from\b', 'derives from', stripped)
        # Fix 'require ' -> 'requires '
        stripped = re.sub(r'\brequire\s+(?!clause)', 'requires ', stripped)

        # Fix qualified subjects (dots) -> simple identifier (take last part)
        # e.g., 'authentication.temporal matches x' -> 'authentication matches x'
        match = re.match(r'^([\w.]+)\s+(has|is|derives|requires|matches|never|emits)\b', stripped)
        if match:
            subject = match.group(1)
            if '.' in subject:
                # Take last part of qualified name as subject
                simple_subject = subject.split('.')[-1]
                stripped = simple_subject + stripped[len(subject):]

        if decl_type == 'trait':
            # In traits: 'uses' should not have a subject
            # 'subject uses x' -> 'uses x'
            stripped = re.sub(r'^[\w.]+\s+uses\s+', 'uses ', stripped)

            # Standalone 'emits x' -> 'action emits x'
            if stripped.startswith('emits '):
                stripped = 'action ' + stripped

            # Standalone 'is x' (at start of line) -> 'behavior is x'
            if re.match(r'^is\s+\w', stripped):
                stripped = 'behavior ' + stripped

        elif decl_type == 'constraint':
            # 'matches x y' without subject -> 'x matches y'
            match = re.match(r'^matches\s+(\w+)\s+(.+)$', stripped)
            if match:
                stripped = f"{match.group(1)} matches {match.group(2)}"

            # 'never x' without subject -> 'invariant never x'
            if stripped.startswith('never '):
                stripped = 'invariant ' + stripped

        elif decl_type == 'system':
            # Remove 'system' prefix from requires
            stripped = re.sub(r'^system\s+requires\s+', 'requires ', stripped)
            # Remove 'all' prefix with requires
            stripped = re.sub(r'^all\s+(\w+)\s+is\s+', r'\1 is ', stripped)

            # Fix requires without version
            # 'requires module.name' -> 'requires module.name >= 0.0.1'
            match = re.match(r'^requires\s+([\w.]+)\s*$', stripped)
            if match:
                stripped = f"requires {match.group(1)} >= 0.0.1"

        fixed_lines.append(prefix + stripped)

    return '\n'.join(fixed_lines)

def split_dol_file(input_path: Path, output_dir: Path):
    """Split a DOL file with multiple declarations into individual files."""
    with open(input_path, 'r') as f:
        content = f.read()

    # Remove single-line comments to avoid matching keywords inside them
    content_no_comments = re.sub(r'//[^\n]*', '', content)

    declarations = []

    # Find all declaration starts - must be at start of line (after newline/whitespace)
    decl_pattern = r'(?:^|\n)\s*(gene|trait|constraint|system)\s+([\w.@\s>]+?)\s*\{'

    pos = 0
    while True:
        match = re.search(decl_pattern, content_no_comments[pos:])
        if not match:
            break

        decl_type = match.group(1)
        name = match.group(2).strip()

        # Find the body (balanced braces) - use content_no_comments consistently
        body_start = pos + match.end() - 1  # Position of opening brace
        body, body_end = extract_balanced_braces(content_no_comments, body_start)

        # Fix syntax issues in body
        body = fix_body_syntax(body, decl_type)

        # Find exegesis block
        remaining = content_no_comments[body_end+1:]
        exegesis_match = re.search(r'\s*exegesis\s*\{', remaining)

        if exegesis_match:
            exegesis_start = body_end + 1 + exegesis_match.end() - 1
            exegesis, exegesis_end = extract_balanced_braces(content_no_comments, exegesis_start)
            next_pos = exegesis_end + 1
        else:
            exegesis = ""
            next_pos = body_end + 1

        # Clean up name for filename
        filename = name.replace(' ', '_').replace('@', '_').replace('>', '_').replace('.', '_')

        # Escape curly braces in exegesis (parser doesn't handle nested braces)
        # Replace {name} with <name> for template placeholders
        exegesis_escaped = re.sub(r'\{(\w+)\}', r'<\1>', exegesis)
        # Also replace remaining braces with parentheses
        exegesis_escaped = exegesis_escaped.replace('{', '(').replace('}', ')')

        declarations.append({
            'type': decl_type,
            'name': name,
            'body': body,
            'exegesis': exegesis_escaped,
            'filename': filename
        })

        pos = next_pos

    # Create output directories
    for decl_type in ['genes', 'traits', 'constraints', 'systems']:
        (output_dir / decl_type).mkdir(parents=True, exist_ok=True)

    # Write each declaration to its own file
    for decl in declarations:
        type_dir = decl['type'] + 's'  # gene -> genes, trait -> traits, etc.
        if decl['type'] == 'constraint':
            type_dir = 'constraints'

        filepath = output_dir / type_dir / f"{decl['filename']}.dol"

        # Format the DOL content
        dol_content = f"""{decl['type']} {decl['name']} {{{decl['body']}}}

exegesis {{{decl['exegesis']}}}
"""
        with open(filepath, 'w') as f:
            f.write(dol_content)

        print(f"  Created: {filepath.relative_to(output_dir.parent)}")

    return len(declarations)

def main():
    retrospective = Path('/home/ardeshir/repos/metadol/docs/ontology/retrospective')

    modules = ['container', 'state', 'cluster', 'api', 'events']

    total = 0
    for module in modules:
        input_file = retrospective / f'{module}.dol'
        output_dir = retrospective / module

        if input_file.exists():
            print(f"\nSplitting {module}.dol:")
            count = split_dol_file(input_file, output_dir)
            total += count
            print(f"  Split into {count} files")

    print(f"\nTotal: {total} individual .dol files created")

if __name__ == '__main__':
    main()
