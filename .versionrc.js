module.exports = {
  "types": [
    {"type":"🚀  feat","section":"🚀  feat"},
    {"type":"fix","section":"Bug Fixes"},
    {"type":"test","section":"Tests", "hidden": true},
    {"type":"build","section":"Build System", "hidden": true},
    {"type":"ci","hidden":true}
  ],
  "parserOpts": {
    headerPattern: "/^(.*\s{2}\w+)\((.+)\).*:\s*(.*)\s*$/",
    headerCorrespondence: [
      `type`,
      `scope`,
      `subject`
    ]
  }
}
