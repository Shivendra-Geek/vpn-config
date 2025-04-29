'use client';

import { useState } from 'react';
import Link from 'next/link';
import { Menu, X } from 'lucide-react';

export default function Header() {
  const [isOpen, setIsOpen] = useState(false);

  const toggleMenu = () => setIsOpen(!isOpen);

  return (
    <header className="bg-white shadow-md sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 py-4 flex items-center justify-between">
        {/* Logo */}
        <Link href="/">
          <span className="text-xl font-bold text-blue-600 cursor-pointer">YourApp</span>
        </Link>

        {/* Desktop Menu */}
        <nav className="hidden md:flex space-x-8 text-gray-700 font-medium">
          <Link href="/" className="hover:text-blue-600 transition">Home</Link>
          <Link href="/product" className="hover:text-blue-600 transition">Products</Link>
          <Link href="/about" className="hover:text-blue-600 transition">About</Link>
          <Link href="/contact" className="hover:text-blue-600 transition">Contact</Link>
        </nav>

        {/* Mobile Menu Icon */}
        <button className="md:hidden" onClick={toggleMenu}>
          {isOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
        </button>
      </div>

      {/* Mobile Dropdown Menu */}
      {isOpen && (
        <nav className="md:hidden bg-white px-4 pb-4">
          <Link href="/" className="block py-2 text-gray-700 hover:text-blue-600">Home</Link>
          <Link href="/product" className="block py-2 text-gray-700 hover:text-blue-600">Products</Link>
          <Link href="/about" className="block py-2 text-gray-700 hover:text-blue-600">About</Link>
          <Link href="/contact" className="block py-2 text-gray-700 hover:text-blue-600">Contact</Link>
        </nav>
      )}
    </header>
  );
}
