
// for desktop and mobile ui dashboard
const closeButton = document.getElementById('close-button');
const openButton = document.getElementById('menu-open');
const offCanvasMenu = document.getElementById('mobile-menu');
const menuBackdrop = document.getElementById('menu-backdrop');
const menu = document.getElementById('menu');
const mobileMenuButton = document.getElementById('mobile-menu-button');

// for mobile logged out
closeButton.addEventListener('click', (e) => {
  e.preventDefault();
  offCanvasMenu.classList.remove('active');
  menuBackdrop.classList.remove('active');
  menu.classList.remove('active');
  menuBackdrop.classList.add('hidden');
  menu.classList.add('hidden');
  offCanvasMenu.classList.add('hidden');
  offCanvasMenu.classList.remove('h-screen');
});


openButton.addEventListener('click', (e) => {
  e.preventDefault();
  offCanvasMenu.classList.remove('hidden')
  menuBackdrop.classList.remove('hidden')
  menu.classList.remove('hidden')
  offCanvasMenu.classList.add('active');
  menuBackdrop.classList.add('active');
  menu.classList.add('active');
  menuBackdrop.style.display = 'block';
  menu.style.display = 'block';
  offCanvasMenu.display = 'block';
  offCanvasMenu.classList.add('h-screen');
});

menuBackdrop.addEventListener('click', (e) => {
  e.preventDefault();
  alert('clicked');
});