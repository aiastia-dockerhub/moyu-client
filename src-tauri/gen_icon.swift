// 用法: swift gen_icon.swift 输出路径.png  (生成 1024x1024 应用图标)
import AppKit

let out = CommandLine.arguments[1]
let S: CGFloat = 1024

let image = NSImage(size: NSSize(width: S, height: S))
image.lockFocus()

// 圆角方形裁剪（macOS 大面板风格）
let margin: CGFloat = 100
let rect = NSRect(x: margin, y: margin, width: S - margin * 2, height: S - margin * 2)
let path = NSBezierPath(roundedRect: rect, xRadius: 185, yRadius: 185)
path.addClip()

// 墨色渐变背景
let bg = NSGradient(
    starting: NSColor(red: 0.13, green: 0.14, blue: 0.20, alpha: 1),
    ending: NSColor(red: 0.23, green: 0.25, blue: 0.33, alpha: 1)
)!
bg.draw(in: rect, angle: -90)

// 楷体「墨」字，取第一个可用的字体
let fontName = ["STKaiti", "Kaiti SC", "Songti SC", "PingFang SC"]
    .compactMap { NSFont(name: $0, size: 520) }
    .first ?? NSFont.systemFont(ofSize: 520)
let attrs: [NSAttributedString.Key: Any] = [
    .font: fontName,
    .foregroundColor: NSColor(red: 0.94, green: 0.92, blue: 0.86, alpha: 1),
]
let text = NSAttributedString(string: "墨", attributes: attrs)
let bounds = text.boundingRect(
    with: NSSize(width: S, height: S),
    options: [.usesLineFragmentOrigin]
)
text.draw(at: NSPoint(
    x: (S - bounds.width) / 2,
    y: (S - bounds.height) / 2 + margin / 2 - 40
))

image.unlockFocus()

let tiff = image.tiffRepresentation!
let rep = NSBitmapImageRep(data: tiff)!
let png = rep.representation(using: .png, properties: [:])!
try! png.write(to: URL(fileURLWithPath: out))
print("saved \(out)")
