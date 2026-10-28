#!/usr/bin/env python3
"""Builds output/pdf/ArcVu-User-Manual.pdf from docs/USER_MANUAL.md and docs/images.

The parameter reference is generated from plugin/PluginProcessor.cpp, so it cannot go stale.
"""
import re
from pathlib import Path
from xml.sax.saxutils import escape
from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import mm
from reportlab.platypus import HRFlowable, Image, KeepTogether, PageBreak, Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle
from reportlab.platypus.tableofcontents import TableOfContents
from PIL import Image as PILImage

ROOT = Path(__file__).resolve().parents[1]
VERSION = re.search(r'project\(ArcVu VERSION ([0-9.]+)\)', (ROOT / 'CMakeLists.txt').read_text()).group(1)
DOC = ROOT / 'docs'
OUT = ROOT / 'output/pdf/ArcVu-User-Manual.pdf'
NAVY = colors.HexColor('#1d252b')
GREY = colors.HexColor('#566470')
ACCENT = colors.HexColor('#2a9a4a')
PANEL = colors.HexColor('#eef3f1')

GROUPS = [
    ("Meter type and reference", ["kind", "reference"]),
    ("Display", ["face", "single", "guiScale", "metersOnly", "showValues", "showHold", "showTools", "showLabel"]),
    ("Meter settings", ["peak", "rmsMs", "aes17", "vuModel", "riseMs", "fallMs", "overshoot", "peakHold", "valueHold",
                        "clipHold", "clipDb", "peakThresh", "redDb", "comfort", "analogRms", "weight"]),
    ("I/O and channel tools", ["midSide", "trimL", "trimR", "trimM", "trimS", "trimLink", "polL", "polR", "muteL", "muteR",
                               "monitor", "forceMono", "inPlace"]),
    ("Processing tools", ["hpfHz", "hpfPoles", "lpfHz", "lpfPoles", "monoHz", "monoAmt", "dynOn", "dynType", "dynHz",
                          "dynDepth", "dynListen"]),
    ("Preferences", ["dragSens", "refresh", "bypassHidden", "tooltips", "alwaysOnTop"]),
]


def read_parameters():
    """id -> (name, range or choices, default) read from the parameter declarations."""
    text = (ROOT / 'plugin/PluginProcessor.cpp').read_text()
    params = {}
    for m in re.finditer(r'\b(flt|num)\(\s*"(\w+)"\s*,\s*"([^"]+)"\s*,\s*(-?[\d.]+)f?\s*,\s*(-?[\d.]+)f?\s*,\s*(-?[\d.]+)f?\s*\)', text):
        kind, pid, name, lo, hi, default = m.groups()
        params[pid] = (name, f"{lo} to {hi}", default)
    for m in re.finditer(r'\bflag\(\s*"(\w+)"\s*,\s*"([^"]+)"\s*,\s*(true|false)\s*\)', text):
        pid, name, default = m.groups()
        params[pid] = (name, "Off / On", "On" if default == "true" else "Off")
    for m in re.finditer(r'\bchoice\(\s*"(\w+)"\s*,\s*"([^"]+)"\s*,\s*juce::StringArray\s*\{([^}]*)\}\s*,\s*(\d+)\s*\)', text):
        pid, name, items, default = m.groups()
        names = re.findall(r'"([^"]*)"', items)
        params[pid] = (name, ", ".join(names), names[int(default)] if int(default) < len(names) else default)
    # The four trims are declared in a loop over two name tables.
    loop = re.search(r'trimNames\[\] = \{([^}]*)\};\s*const char\* trimLabels\[\] = \{([^}]*)\};.*?flt\(trimNames\[i\], trimLabels\[i\], (-?[\d.]+)f?, (-?[\d.]+)f?, (-?[\d.]+)f?\)', text, re.S)
    if loop:
        ids, labels = re.findall(r'"([^"]*)"', loop.group(1)), re.findall(r'"([^"]*)"', loop.group(2))
        for pid, label in zip(ids, labels):
            params[pid] = (label, f"{loop.group(3)} to {loop.group(4)}", loop.group(5))
    # The reference and peak parameters are declared directly, with their choices on the next line.
    for pid, name, items, default in re.findall(r'ParameterID \{ "(\w+)", 1 \}, "([^"]+)",\s*juce::StringArray \{([^}]*)\}, (\d+)\)', text):
        names = re.findall(r'"([^"]*)"', items)
        shown = ["-" + n + " dBFS" for n in names] if pid == "reference" else names
        params[pid] = (name, ", ".join(shown), shown[int(default)])
    return params


def parameter_section(styles):
    params = read_parameters()
    items = [Paragraph("Parameter reference", styles['h2'])]
    items.append(Paragraph("Every host-automatable parameter, its range and its default value.", styles['body']))
    for title, ids in GROUPS:
        rows = [["Parameter", "Range / choices", "Default"]]
        for pid in ids:
            name, rng, default = params.get(pid, (pid, "", ""))
            rows.append([Paragraph(escape(name), styles['cell']), Paragraph(escape(rng), styles['cell']), Paragraph(escape(default), styles['cell'])])
        table = Table(rows, colWidths=[52 * mm, 90 * mm, 28 * mm], repeatRows=1)
        table.setStyle(TableStyle([('BACKGROUND', (0, 0), (-1, 0), NAVY), ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
                                   ('GRID', (0, 0), (-1, -1), 0.3, GREY), ('VALIGN', (0, 0), (-1, -1), 'TOP'),
                                   ('FONTSIZE', (0, 0), (-1, 0), 9)]))
        items += [Paragraph(title, styles['h3']), table, Spacer(1, 4 * mm)]
    return items


class ManualDoc(SimpleDocTemplate):
    # Level-2 headings feed the table of contents (page numbers are filled on the second pass).
    def afterFlowable(self, flowable):
        if getattr(flowable, 'tocText', None):
            self.notify('TOCEntry', (0, flowable.tocText, self.page))


def callout(text, styles):
    box = Table([[Paragraph(inline(text), styles['callout'])]], colWidths=[170 * mm])
    box.setStyle(TableStyle([('BACKGROUND', (0, 0), (-1, -1), PANEL), ('LINEBEFORE', (0, 0), (0, -1), 3, ACCENT),
                             ('LEFTPADDING', (0, 0), (-1, -1), 10), ('RIGHTPADDING', (0, 0), (-1, -1), 8),
                             ('TOPPADDING', (0, 0), (-1, -1), 6), ('BOTTOMPADDING', (0, 0), (-1, -1), 6)]))
    return [box, Spacer(1, 3 * mm)]


def inline(text):
    text = escape(text)
    text = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', text)
    text = re.sub(r'\*(.+?)\*', r'<i>\1</i>', text)
    text = re.sub(r'`(.+?)`', r'<font face="Courier">\1</font>', text)
    return text


def main():
    base = getSampleStyleSheet()
    styles = {
        'title': ParagraphStyle('title', parent=base['Title'], fontSize=26, textColor=NAVY, spaceAfter=8),
        'h2': ParagraphStyle('h2', parent=base['Heading2'], fontSize=16, textColor=NAVY, spaceBefore=12, spaceAfter=6, keepWithNext=1),
        'h3': ParagraphStyle('h3', parent=base['Heading3'], fontSize=12, textColor=NAVY, spaceBefore=8, spaceAfter=4, keepWithNext=1),
        'body': ParagraphStyle('body', parent=base['BodyText'], fontSize=10, leading=14, spaceAfter=5),
        'bullet': ParagraphStyle('bullet', parent=base['BodyText'], fontSize=10, leading=14, leftIndent=14, bulletIndent=2, spaceAfter=3),
        'cell': ParagraphStyle('cell', parent=base['BodyText'], fontSize=8.5, leading=11),
        'callout': ParagraphStyle('callout', parent=base['BodyText'], fontSize=9.5, leading=13),
        'tagline': ParagraphStyle('tagline', parent=base['BodyText'], fontSize=12, leading=17, textColor=GREY, spaceAfter=10),
        'toc': ParagraphStyle('toc', parent=base['BodyText'], fontSize=10.5, leading=17),
        'caption': ParagraphStyle('caption', parent=base['BodyText'], fontSize=8.5, leading=11, textColor=GREY, alignment=1),
    }
    toc = TableOfContents()
    toc.levelStyles = [styles['toc']]
    flow = []
    cover = False
    lines = (DOC / 'USER_MANUAL.md').read_text().splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith('# '):
            flow.append(Spacer(1, 6 * mm))
            flow.append(Paragraph(inline(line[2:]), styles['title']))
            flow.append(HRFlowable(width='100%', thickness=2, color=ACCENT, spaceAfter=8))
            cover = True
        elif line.startswith('## '):
            heading = Paragraph(inline(line[3:]), styles['h2'])
            heading.tocText = re.sub(r'<.*?>', '', inline(line[3:]))
            flow.append(heading)
        elif line.startswith('### '):
            flow.append(Paragraph(inline(line[4:]), styles['h3']))
        elif line.startswith('<!-- TOC -->'):
            flow += [Paragraph('Contents', styles['h2']), toc, PageBreak()]
        elif line.startswith('> '):
            flow += callout(line[2:], styles)
        elif line.startswith('<!-- PARAMETERS -->'):
            flow += parameter_section(styles)
        elif line.startswith('!['):
            m = re.match(r'!\[(.*?)\]\((.*?)\)', line)
            path = DOC / m.group(2)
            if path.exists():
                w, h = PILImage.open(path).size
                width = 170 * mm
                width = min(width, 170 * mm * min(1.0, h and 1.0))
                if h / w > 1.2:
                    width = 80 * mm  # tall images (the Vertical face) stay narrow
                flow.append(KeepTogether([Image(str(path), width=width, height=width * h / w), Paragraph(inline(m.group(1)), styles['caption']), Spacer(1, 3 * mm)]))
        elif line.startswith('|'):
            rows = []
            while i < len(lines) and lines[i].startswith('|'):
                cells = [c.strip() for c in lines[i].strip('|').split('|')]
                if not all(set(c) <= set('-: ') for c in cells):
                    rows.append([Paragraph(inline(c), styles['cell']) for c in cells])
                i += 1
            n = len(rows[0])
            widths = [35 * mm, 135 * mm] if n == 2 else [170 * mm / n] * n
            table = Table(rows, colWidths=widths, repeatRows=1)
            table.setStyle(TableStyle([('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#dde4e8')), ('GRID', (0, 0), (-1, -1), 0.3, GREY),
                                       ('VALIGN', (0, 0), (-1, -1), 'TOP')]))
            flow += [table, Spacer(1, 3 * mm)]
            continue
        elif line.startswith('- '):
            flow.append(Paragraph(inline(line[2:]), styles['bullet'], bulletText='•'))
        elif line.strip():
            flow.append(Paragraph(inline(line), styles['tagline'] if cover else styles['body']))
            cover = False
        i += 1
    OUT.parent.mkdir(parents=True, exist_ok=True)
    doc = ManualDoc(str(OUT), pagesize=A4, leftMargin=20 * mm, rightMargin=20 * mm, topMargin=18 * mm, bottomMargin=18 * mm,
                            title=f"ArcVu {VERSION} User Manual", author="OpenGrid")
    doc.multiBuild(flow, onFirstPage=footer, onLaterPages=footer)
    print(OUT)


def footer(canvas, doc):
    canvas.saveState()
    canvas.setFont('Helvetica', 8)
    canvas.setFillColor(GREY)
    canvas.drawString(20 * mm, 10 * mm, f"ArcVu {VERSION} User Manual - OpenGrid")
    canvas.drawRightString(A4[0] - 20 * mm, 10 * mm, str(doc.page))
    canvas.restoreState()


if __name__ == '__main__':
    main()
