module.exports = {
  tutorialSidebar: [  // <-- Ensure this matches what Docusaurus expects
    {
      type: 'category',
      label: 'Getting Started',
      collapsed: false,
      items: ['intro'],  // Make sure "intro.md" exists
    },
    {
      type: 'category',
      label: 'Data Structures',
      collapsed: false,
      items: ['vector', 'stack'],  // Ensure these files exist
    },
  ],
};
