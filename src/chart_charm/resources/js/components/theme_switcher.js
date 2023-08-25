// Function to set the selected theme
const setSelectedTheme = (themeValue) => {
    const themeSwitcher = document.getElementById('theme-switcher');
    themeSwitcher.value = themeValue;
    document.documentElement.setAttribute('data-theme', themeValue);
  };
  
  // Function to get the selected theme from the server
const loadSelectedTheme = async () => {
    const response = await fetch('/settings/theme/get');
    const data = await response.json();
    const themeValue = data.theme_selected;
    setSelectedTheme(themeValue);
  };

document.addEventListener('DOMContentLoaded', function() {
    loadSelectedTheme();
    const html = document.documentElement;
    const themeSwitcher = document.getElementById('theme-switcher');
    
    if (themeSwitcher) { // Check if themeSwitcher is not null
      html.setAttribute('data-theme', 'auto');

      themeSwitcher.addEventListener('change', (e) => {
        const selectedTheme = e.target.value;
        html.setAttribute('data-theme', selectedTheme);
      
        // Send the selected theme to the server
        fetch('/settings/theme/save', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ theme_selected: selectedTheme })
        });
      });
      
    }
  });