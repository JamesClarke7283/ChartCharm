SELECT t.name FROM settings s
JOIN theme t ON t.id = s.value
WHERE s.name = 'selected_theme';