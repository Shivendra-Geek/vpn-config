

import React from 'react';


export default async function Page() {
  const res = await fetch('https://dummyjson.com/products');
  if (!res.ok) {
    throw new Error('Failed to fetch data');
  }

  const data = await res.json();
  const products = data.products;

  return (
    <main className="min-h-screen bg-gray-50 px-6 py-10">
      <h1 className="text-4xl font-bold text-center text-blue-700 mb-10">Our Products</h1>

      <div className="grid gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
        {products.map((product) => (
          <div
            key={product.id}
            className="bg-white rounded-2xl shadow hover:shadow-xl transition p-4 flex flex-col"
          >
            <div className="relative overflow-hidden rounded-xl mb-4">
              <img
                src={product.thumbnail}
                alt={product.title}
                className="w-full h-48 object-cover transform hover:scale-105 transition duration-300"
              />
            </div>
            <h2 className="text-lg font-semibold text-gray-800">{product.title}</h2>
            <p className="text-sm text-gray-500 line-clamp-2">{product.description}</p>
            <div className="mt-auto flex justify-between items-center pt-4">
              <span className="text-blue-600 font-bold text-lg">${product.price}</span>
              <button className="px-3 py-1 bg-blue-600 text-white rounded-full text-sm hover:bg-blue-700 transition">
                Add to Cart
              </button>
            </div>
          </div>
        ))}
      </div>
    </main>
  );
}
