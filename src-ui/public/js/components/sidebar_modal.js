document.addEventListener('DOMContentLoaded', () => {
    const sidebarHomeBtn = document.getElementById('sidebar-home-btn');
    if (sidebarHomeBtn) {
      console.log('Adding event listener to sidebar-home-btn');
      sidebarHomeBtn.addEventListener('click', (event) => {
        event.preventDefault(); // Prevent any default behavior
        console.log('sidebar-home-btn clicked');
        if (visibleModal != null) {
          console.log('Closing modal');
          closeModal(visibleModal);
        }
      });
    } else {
      console.error('sidebar-home-btn not found');
    }
  });